use std::result;
use std::collections::HashMap;
use rand::Rng;
use knowledge::PlayerKnowledgeGrid;
use reaction::Reaction;
use behaviour::*;
use entity_store::*;
use spatial_hash::*;
use entity_id_allocator::*;
use observation::shadowcast::ShadowcastEnv;
use meta_action::*;
use policy::*;
use commit::{self, CommitEnv};
use renderer::GameRenderer;
use input::GameInput;
use util::schedule::{Schedule, ScheduleEntry};
use content::ActionType;
use player_act;
use npc_act;

#[derive(Debug)]
pub enum Error {
    MissingNpcKnowledge,
    MissingNpcBehaviour,
    PlayerTurnError(player_act::Error),
    NpcTurnError(npc_act::Error),
    CommitFailed(commit::Error),
}
pub type Result<T> = result::Result<T, Error>;

impl From<player_act::Error> for Error {
    fn from(e: player_act::Error) -> Self {
        Error::PlayerTurnError(e)
    }
}

impl From<commit::Error> for Error {
    fn from(e: commit::Error) -> Self {
        Error::CommitFailed(e)
    }
}

impl From<npc_act::Error> for Error {
    fn from(e: npc_act::Error) -> Self {
        Error::NpcTurnError(e)
    }
}

pub enum TurnResolution {
    Reschedule,
    NoEntity,
    External(External),
    Debug(DebugAction),
}

pub struct TurnEnv<'a, R: 'a + Rng, Ren: 'a + GameRenderer, Inp: 'a + GameInput> {
    pub renderer: &'a mut Ren,
    pub input: &'a mut Inp,
    pub reactions: &'a mut Vec<Reaction>,
    pub action_schedule_entries: &'a mut Vec<ScheduleEntry<ActionType>>,
    pub change: &'a mut EntityStoreChange,
    pub entity_store: &'a mut EntityStore,
    pub id_allocator: &'a mut EntityIdAllocator,
    pub spatial_hash: &'a mut SpatialHashTable,
    pub behaviour_env: &'a mut BehaviourEnv,
    pub player_id: EntityId,
    pub entity_id: EntityId,
    pub player_knowledge: &'a mut PlayerKnowledgeGrid,
    pub knowledge: &'a mut HashMap<EntityId, PlayerKnowledgeGrid>,
    pub behaviour: &'a mut HashMap<EntityId, BehaviourState>,
    pub shadowcast: &'a mut ShadowcastEnv,
    pub time: &'a mut u64,
    pub policy: &'a mut GamePolicy,
    pub rng: &'a mut R,
    pub schedule: &'a mut Schedule<ActionType>,
}

impl<'a, R: Rng, Ren: GameRenderer, Inp: GameInput> TurnEnv<'a, R, Ren, Inp> {
    pub fn take_turn(self) -> Result<TurnResolution> {

        let initial_action = if self.entity_store.player.contains(&self.entity_id) {
            let meta_action = player_act::PlayerActEnv {
                renderer: self.renderer,
                input: self.input,
                change: self.change,
                entity_store: self.entity_store,
                spatial_hash: self.spatial_hash,
                entity_id: self.entity_id,
                knowledge: self.player_knowledge,
                shadowcast: self.shadowcast,
                time: self.time,
                policy: self.policy,
                rng: self.rng,
            }.act()?;

            match meta_action {
                MetaAction::Action(action) => action,
                MetaAction::External(external) => return Ok(TurnResolution::External(external)),
                MetaAction::Debug(debug_action) => return Ok(TurnResolution::Debug(debug_action)),
            }
        } else if self.entity_store.npc.contains(&self.entity_id) {
            npc_act::NpcActEnv {
                entity_store: self.entity_store,
                spatial_hash: self.spatial_hash,
                entity_id: self.entity_id,
                knowledge: self.knowledge.get_mut(&self.entity_id).ok_or(Error::MissingNpcKnowledge)?,
                behaviour_state: self.behaviour.get_mut(&self.entity_id).ok_or(Error::MissingNpcBehaviour)?,
                behaviour_env: self.behaviour_env,
                shadowcast: self.shadowcast,
                time: self.time,
            }.act()?
        } else {
            return Ok(TurnResolution::NoEntity);
        };

        CommitEnv {
            renderer: self.renderer,
            change: self.change,
            entity_store: self.entity_store,
            spatial_hash: self.spatial_hash,
            player_knowledge: self.player_knowledge,
            player_id: self.player_id,
            shadowcast: self.shadowcast,
            time: self.time,
            reactions: self.reactions,
            action_schedule_entries: self.action_schedule_entries,
            id_allocator: self.id_allocator,
            policy: self.policy,
            schedule: self.schedule,
            rng: self.rng,
            input: self.input,
        }.commit(initial_action)?;

        Ok(TurnResolution::Reschedule)
    }
}
