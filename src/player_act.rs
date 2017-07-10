use std::result;
use rand::Rng;
use content::ActionType;
use meta_action::*;
use cgmath::{Vector2, Vector3};
use geometry::direction::Direction;
use geometry::straight_line::*;
use geometry::limits::LimitsRect;
use render_overlay::RenderOverlay;
use player_render;
use policy::*;
use entity_store::*;
use observation::shadowcast::ShadowcastEnv;
use knowledge::PlayerKnowledgeGrid;
use spatial_hash::*;
use renderer::GameRenderer;
use input::*;
use entity_observe;
use observation::ObservationMetadata;
use input::GameInput;

#[derive(Debug)]
pub enum Error {
    RenderingFailed(player_render::Error),
    ObservationFailed(entity_observe::Error),
}
pub type Result<T> = result::Result<T, Error>;

impl From<player_render::Error> for Error {
    fn from(e: player_render::Error) -> Self {
        Error::RenderingFailed(e)
    }
}

impl From<entity_observe::Error> for Error {
    fn from(e: entity_observe::Error) -> Self {
        Error::ObservationFailed(e)
    }
}

pub struct PlayerActEnv<'a, R: 'a + Rng, Ren: 'a + GameRenderer, Inp: 'a + GameInput> {
    pub renderer: &'a mut Ren,
    pub input: &'a mut Inp,
    pub change: &'a mut EntityStoreChange,
    pub entity_store: &'a mut EntityStore,
    pub spatial_hash: &'a mut SpatialHashTable,
    pub entity_id: EntityId,
    pub knowledge: &'a mut PlayerKnowledgeGrid,
    pub shadowcast: &'a mut ShadowcastEnv,
    pub time: &'a mut u64,
    pub policy: &'a mut GamePolicy,
    pub rng: &'a mut R,
}

impl<'a, R: Rng, Ren: GameRenderer, Inp: GameInput> PlayerActEnv<'a, R, Ren, Inp> {
    pub fn render(&mut self) -> player_render::Result<()>{
        player_render::player_render(
            self.entity_id,
            self.entity_store,
            self.spatial_hash,
            *self.time,
            self.knowledge,
            self.shadowcast,
            self.renderer
        )
    }

    fn input_to_action(&mut self, input: InputEvent) -> Result<Option<ActionType>> {
        match input {
            InputEvent::Up => return Ok(Some(ActionType::Walk(self.entity_id, Direction::North))),
            InputEvent::Down => return Ok(Some(ActionType::Walk(self.entity_id, Direction::South))),
            InputEvent::Left => return Ok(Some(ActionType::Walk(self.entity_id, Direction::West))),
            InputEvent::Right => return Ok(Some(ActionType::Walk(self.entity_id, Direction::East))),
            InputEvent::Space => return Ok(Some(ActionType::Null)),
            InputEvent::Char('f') => {
                let start = *self.entity_store.position.get(&self.entity_id).expect("Missing position");
                let action = if let Some(traverse) = self.aim(start)? {
                    ActionType::FireBullet(traverse)
                } else {
                    ActionType::Null
                };

                self.renderer.clear();
                self.renderer.draw();
                self.renderer.publish();

                return Ok(Some(action));
            }
            _ => return Ok(None),
        }
    }

    fn input_to_external(&mut self, input: InputEvent) -> Option<External> {
        match input {
            InputEvent::Quit => return Some(External::Quit),
            _ => return None,
        }
    }

    fn input_to_debug(&mut self, input: InputEvent) -> Option<DebugAction> {
        match input {
            InputEvent::Char('1') => return Some(DebugAction::ChangeVeilMin(-0.05)),
            InputEvent::Char('2') => return Some(DebugAction::ChangeVeilMin(0.05)),
            InputEvent::Char('3') => return Some(DebugAction::ChangeVeilMax(-0.05)),
            InputEvent::Char('4') => return Some(DebugAction::ChangeVeilMax(0.05)),
            InputEvent::Char('8') => return Some(DebugAction::ToggleDiminishingLighting),
            InputEvent::Char('9') => return Some(DebugAction::TogglePlayerOmniscient),
            InputEvent::Char('0') => return Some(DebugAction::Wait),
            InputEvent::Char('!') => return Some(DebugAction::ChangeVeilStep(Vector3::new(-0.01, 0.0, 0.0))),
            InputEvent::Char('@') => return Some(DebugAction::ChangeVeilStep(Vector3::new(0.01, 0.0, 0.0))),
            InputEvent::Char('#') => return Some(DebugAction::ChangeVeilStep(Vector3::new(0.0, -0.01, 0.0))),
            InputEvent::Char('$') => return Some(DebugAction::ChangeVeilStep(Vector3::new(0.0, 0.01, 0.0))),
            InputEvent::Char('%') => return Some(DebugAction::ChangeVeilStep(Vector3::new(0.0, 0.0, -0.01))),
            InputEvent::Char('^') => return Some(DebugAction::ChangeVeilStep(Vector3::new(0.0, 0.0, 0.01))),
            _ => return None,
        }
    }

    pub fn act(&mut self) -> Result<MetaAction> {

        self.render()?;

        loop {

            let event = self.input.next_external();

            if let Some(frame) = event.frame() {
                self.policy.on_frame_animate(frame, self.entity_store, self.spatial_hash, self.rng, self.change);
                *self.time += 1;
                self.spatial_hash.update(self.entity_store, self.change, *self.time);
                self.entity_store.commit_change(self.change);
                self.render()?;
            }

            if let Some(input) = event.input() {
                let maybe_meta_action = self.input_to_action(input)?.map(MetaAction::Action)
                    .or_else(|| self.input_to_external(input).map(MetaAction::External))
                    .or_else(|| self.input_to_debug(input).map(MetaAction::Debug));

                if let Some(meta_action) = maybe_meta_action {
                    return Ok(meta_action);
                }
            }
        }
    }

    fn observe(&mut self) -> Result<ObservationMetadata> {
        entity_observe::entity_observe(
            self.entity_id,
            self.entity_store,
            self.spatial_hash,
            *self.time,
            self.knowledge,
            self.shadowcast
        ).map_err(Error::ObservationFailed)
    }

    fn aim(&mut self, start: Vector2<i32>) -> Result<Option<InfiniteAbsoluteLineTraverse>> {
        let mut end = start;
        loop {
            let line = FiniteAbsoluteLineTraverse::new_between(start, end);
            let overlay = RenderOverlay {
                aim_line: line,
            };
            self.renderer.clear();
            self.renderer.update_player_knowledge(self.knowledge, *self.time);
            self.renderer.draw();
            self.renderer.draw_overlay(overlay);
            self.renderer.publish();

            let event = self.input.next_external();

            if let Some(frame) = event.frame() {
                    self.policy.on_frame_animate(frame, self.entity_store, self.spatial_hash, self.rng, self.change);
                    *self.time += 1;
                    self.spatial_hash.update(self.entity_store, self.change, *self.time);
                    self.entity_store.commit_change(self.change);
                    self.observe()?;
            }

            let change = if let Some(input) = event.input() {
                match input {
                    InputEvent::Up => Vector2::new(0, -1),
                    InputEvent::Down => Vector2::new(0, 1),
                    InputEvent::Left => Vector2::new(-1, 0),
                    InputEvent::Right => Vector2::new(1, 0),
                    InputEvent::Return | InputEvent::Char('f') => {
                        return Ok(Some(InfiniteAbsoluteLineTraverse::new_between(start, end)));
                    }
                    _ => return Ok(None),
                }
            } else {
                continue;
            };

            end = self.spatial_hash.saturate(end + change);
        }
    }
}
