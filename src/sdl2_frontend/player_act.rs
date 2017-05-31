use std::result;
use rand::Rng;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use content::ActionType;
use meta_action::*;
use direction::Direction;
use cgmath::Vector2;
use straight_line::*;
use render_overlay::RenderOverlay;
use limits::LimitsRect;
use sdl2_frontend::player_render;
use policy::*;
use entity_store::*;
use observation::shadowcast::ShadowcastEnv;
use sdl2_frontend::renderer::GameRenderer;
use knowledge::PlayerKnowledgeGrid;
use spatial_hash::*;

#[derive(Debug)]
pub enum Error {
    RenderingFailed,
}
pub type Result<T> = result::Result<T, Error>;

pub struct PlayerActEnv<'a, 'renderer: 'a, R: 'a + Rng> {
    pub renderer: &'a mut GameRenderer<'renderer>,
    pub input: &'a mut EventPump,
    pub change: &'a mut EntityStoreChange,
    pub entity_store: &'a mut EntityStore,
    pub spatial_hash: &'a mut SpatialHashTable,
    pub entity_id: EntityId,
    pub knowledge: &'a mut PlayerKnowledgeGrid,
    pub shadowcast: &'a mut ShadowcastEnv,
    pub time: &'a mut u64,
    pub policy: &'a GamePolicy,
    pub rng: &'a mut R,
}

impl<'a, 'renderer: 'a, R: Rng> PlayerActEnv<'a, 'renderer, R> {
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

    pub fn act(&mut self) -> Result<MetaAction> {

        self.render().map_err(|_| Error::RenderingFailed)?;

        loop {
            match self.input.wait_event_timeout(128) {
                Some(Event::Quit { .. }) => return Ok(MetaAction::External(External::Quit)),
                Some(Event::KeyDown { keycode: Some(keycode), .. }) => {
                    let action = match keycode {
                        Keycode::Up => ActionType::Walk(self.entity_id, Direction::North),
                        Keycode::Down => ActionType::Walk(self.entity_id, Direction::South),
                        Keycode::Left => ActionType::Walk(self.entity_id, Direction::West),
                        Keycode::Right => ActionType::Walk(self.entity_id, Direction::East),
                        Keycode::F => {
                            let start = *self.entity_store.position.get(&self.entity_id).expect("Missing position");
                            self.aim(start);
                            ActionType::Null
                        }
                        _ => continue,
                    };

                    return Ok(MetaAction::Action(action));
                }
                None => {
                    self.policy.on_frame(0, self.entity_store, self.spatial_hash, self.rng, self.change);
                    *self.time += 1;
                    self.spatial_hash.update(self.entity_store, self.change, *self.time);
                    self.entity_store.commit_change(self.change);
                    self.render().map_err(|_| Error::RenderingFailed)?;
                }
                _ => continue,
            }
        }
    }

    fn aim(&mut self, start: Vector2<i32>) -> Option<InfiniteAbsoluteLineTraverse> {
        let mut end = start;
        loop {
            let line = FiniteAbsoluteLineTraverse::new_between(start, end);
            let overlay = RenderOverlay {
                aim_line: line,
            };
            self.renderer.clear();
            self.renderer.draw();
            self.renderer.draw_overlay(overlay);
            self.renderer.publish();
            let change = match self.input.wait_event() {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Up => Vector2::new(0, -1),
                        Keycode::Down => Vector2::new(0, 1),
                        Keycode::Left => Vector2::new(-1, 0),
                        Keycode::Right => Vector2::new(1, 0),
                        Keycode::Return => {
                            return Some(InfiniteAbsoluteLineTraverse::new_between(start, end));
                        }
                        _ => return None,
                    }
                }
                _ => continue,
            };

            end = self.spatial_hash.saturate(end + change);
        }
    }
}
