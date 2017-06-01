use std::result;
use std::collections::HashMap;
use rand::Rng;
use player_act::PlayerActEnv;
use npc_act::NpcActEnv;
use knowledge::PlayerKnowledgeGrid;
use reaction::Reaction;
use behaviour::*;
use entity_store::*;
use spatial_hash::*;
use entity_id_allocator::*;
use observation::shadowcast::ShadowcastEnv;
use meta_action::*;
use policy::*;
use commit::CommitEnv;
use renderer::GameRenderer;
use input::GameInput;
use schedule::Schedule;
use content::ActionType;

#[derive(Debug)]
pub enum Error {
    MissingNpcKnowledge,
    MissingNpcBehaviour,
    PlayerTurnError,
    NpcTurnError,
}
pub type Result<T> = result::Result<T, Error>;

pub enum TurnResolution {
    Reschedule,
    External(External),
}

pub struct TurnEnv<'a, R: 'a + Rng, Ren: 'a + GameRenderer, Inp: 'a + GameInput> {
    pub renderer: &'a mut Ren,
    pub input: &'a mut Inp,
    pub reactions: &'a mut Vec<Reaction>,
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
            let meta_action = PlayerActEnv {
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
            }.act().map_err(|_| Error::PlayerTurnError)?;

            match meta_action {
                MetaAction::Action(action) => action,
                MetaAction::External(external) => return Ok(TurnResolution::External(external)),
            }
        } else {
            NpcActEnv {
                entity_store: self.entity_store,
                spatial_hash: self.spatial_hash,
                entity_id: self.entity_id,
                knowledge: self.knowledge.get_mut(&self.entity_id).ok_or(Error::MissingNpcKnowledge)?,
                behaviour_state: self.behaviour.get_mut(&self.entity_id).ok_or(Error::MissingNpcBehaviour)?,
                behaviour_env: self.behaviour_env,
                shadowcast: self.shadowcast,
                time: self.time,
            }.act().map_err(|_| Error::NpcTurnError)?
        };

        CommitEnv {
            renderer: self.renderer,
            change: self.change,
            entity_store: self.entity_store,
            spatial_hash: self.spatial_hash,
            player_knowledge: self.player_knowledge,
            shadowcast: self.shadowcast,
            time: self.time,
            reactions: self.reactions,
            id_allocator: self.id_allocator,
            policy: self.policy,
            schedule: self.schedule,
        }.commit(initial_action);

        Ok(TurnResolution::Reschedule)
    }
}
