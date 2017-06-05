use std::result;
use rand::Rng;
use knowledge::PlayerKnowledgeGrid;
use reaction::Reaction;
use entity_store::*;
use spatial_hash::*;
use entity_id_allocator::*;
use content::ActionType;
use observation::shadowcast::ShadowcastEnv;
use policy::*;
use renderer::GameRenderer;
use input::GameInput;
use schedule::{Schedule, ScheduleEntry};
use player_render;

#[derive(Debug)]
pub enum Error {
    RenderingFailed(player_render::Error),
}
pub type Result<T> = result::Result<T, Error>;

impl From<player_render::Error> for Error {
    fn from(e: player_render::Error) -> Self {
        Error::RenderingFailed(e)
    }
}

pub struct CommitEnv<'a, R: 'a + Rng, Ren: 'a + GameRenderer, Inp: 'a + GameInput> {
    pub renderer: &'a mut Ren,
    pub change: &'a mut EntityStoreChange,
    pub entity_store: &'a mut EntityStore,
    pub spatial_hash: &'a mut SpatialHashTable,
    pub player_knowledge: &'a mut PlayerKnowledgeGrid,
    pub player_id: EntityId,
    pub shadowcast: &'a mut ShadowcastEnv,
    pub time: &'a mut u64,
    pub reactions: &'a mut Vec<Reaction>,
    pub action_schedule_entries: &'a mut Vec<ScheduleEntry<ActionType>>,
    pub id_allocator: &'a mut EntityIdAllocator,
    pub policy: &'a mut GamePolicy,
    pub schedule: &'a mut Schedule<ActionType>,
    pub rng: &'a mut R,
    pub input: &'a mut Inp,
}

struct FrameDescription {
    actions_occur: bool,
    real_time_passes: bool,
    schedule_empty: bool,
}

impl<'a, R: Rng, Ren: GameRenderer, Inp: GameInput> CommitEnv<'a, R, Ren, Inp> {

    fn render(&mut self) -> Result<()>{
        player_render::player_render(
            self.player_id,
            self.entity_store,
            self.spatial_hash,
            *self.time,
            self.player_knowledge,
            self.shadowcast,
            self.renderer
        ).map_err(Error::RenderingFailed)
    }

    fn next_frame_description(&mut self, prev_abs_time: u64) -> FrameDescription {
        if let Some(next_release_time) = self.schedule.peek().map(|e| e.release_time) {

            if next_release_time == prev_abs_time {
                FrameDescription {
                    actions_occur: true,
                    real_time_passes: false,
                    schedule_empty: false,
                }
            } else if next_release_time == prev_abs_time + 1 {
                FrameDescription {
                    actions_occur: true,
                    real_time_passes: true,
                    schedule_empty: false,
                }
            } else {
                FrameDescription {
                    actions_occur: false,
                    real_time_passes: true,
                    schedule_empty: false,
                }
            }
        } else {
            FrameDescription {
                actions_occur: false,
                real_time_passes: true,
                schedule_empty: true,
            }
        }
    }

    pub fn commit(&mut self, initial_action: ActionType) -> Result<()> {

        let mut maybe_first_frame = None;

        self.reactions.clear();

        self.schedule.insert(initial_action, 0);
        let mut prev_abs_time = self.schedule.absolute_time();

        loop {

            let description = self.next_frame_description(prev_abs_time);

            if description.schedule_empty {
                if !self.policy.has_unresolved_realtime_frames(self.entity_store) {
                    break;
                }
            }

            if description.actions_occur {
                self.schedule.all_next(self.action_schedule_entries);

                for entry in self.action_schedule_entries.drain(..) {
                    entry.value.populate(self.change, self.entity_store, self.id_allocator);
                }
            }

            if description.real_time_passes {
                prev_abs_time += 1;
                let frame = self.input.next_frame();
                let first_frame = if let Some(f) = maybe_first_frame {
                    f
                } else {
                    maybe_first_frame = Some(frame);
                    frame
                };

                self.policy.on_frame_animate(frame, self.entity_store, self.spatial_hash, self.rng, self.change);
                self.policy.on_realtime_change(first_frame, frame, self.change, self.entity_store);
            }

            self.policy.on_change(self.change, self.entity_store, self.spatial_hash, self.reactions);

            for Reaction { action, delay } in self.reactions.drain(..) {
                self.schedule.insert(action, delay);
            }

            *self.time += 1;
            self.spatial_hash.update(self.entity_store, self.change, *self.time);
            self.entity_store.commit_change(self.change);

            if description.real_time_passes {
                self.render()?;
            }
        }

        Ok(())
    }
}
