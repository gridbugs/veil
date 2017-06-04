use knowledge::PlayerKnowledgeGrid;
use behaviour::*;
use entity_store::*;
use spatial_hash::*;
use content::ActionType;
use observation::shadowcast::ShadowcastEnv;
use entity_observe;

pub struct NpcActEnv<'a> {
    pub entity_store: &'a EntityStore,
    pub spatial_hash: &'a SpatialHashTable,
    pub entity_id: EntityId,
    pub knowledge: &'a mut PlayerKnowledgeGrid,
    pub behaviour_env: &'a mut BehaviourEnv,
    pub behaviour_state: &'a mut BehaviourState,
    pub shadowcast: &'a mut ShadowcastEnv,
    pub time: &'a mut u64,
}

pub type Error = entity_observe::Error;
pub type Result<T> = entity_observe::Result<T>;

impl<'a> NpcActEnv<'a> {
    pub fn act(&mut self) -> Result<ActionType> {

        let metadata = entity_observe::entity_observe(self.entity_id,
                                                      self.entity_store,
                                                      self.spatial_hash,
                                                      *self.time,
                                                      self.knowledge,
                                                      self.shadowcast)?;

        Ok(attack::attack(self.entity_id, self.entity_store, self.knowledge, self.behaviour_env, self.behaviour_state).or_else(|| {
            patrol::patrol(self.entity_id, self.entity_store, self.knowledge, metadata, *self.time, self.behaviour_env, self.behaviour_state)
        }).unwrap_or(ActionType::Null))
    }
}
