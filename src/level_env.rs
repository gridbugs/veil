use std::collections::HashMap;
use rand::Rng;
use entity_store::{EntityId, EntityStore, EntityStoreChange};
use spatial_hash::SpatialHashTable;
use util::schedule::Schedule;
use knowledge::PlayerKnowledgeGrid;
use behaviour::{BehaviourState, BehaviourEnv};
use veil_state::VeilState;
use content::VeilStepInfo;

pub struct LevelEnv {
    pub entity_store: EntityStore,
    pub spatial_hash: SpatialHashTable,
    pub turn_schedule: Schedule<EntityId>,
    pub knowledge: HashMap<EntityId, PlayerKnowledgeGrid>,
    pub behaviour: HashMap<EntityId, BehaviourState>,
    pub behaviour_env: BehaviourEnv,
    pub veil_state: VeilState,
    pub player_knowledge: PlayerKnowledgeGrid,
}

impl LevelEnv {
    pub fn new<R: Rng>(width: usize, height: usize,
                       rng: &mut R, veil_step_info: &VeilStepInfo) -> Self {
        LevelEnv {
            entity_store: EntityStore::new(),
            spatial_hash: SpatialHashTable::new(width, height),
            turn_schedule: Schedule::new(),
            knowledge: HashMap::new(),
            behaviour: HashMap::new(),
            behaviour_env: BehaviourEnv::new(width, height),
            veil_state: VeilState::new(width, height, rng, veil_step_info),
            player_knowledge: PlayerKnowledgeGrid::new(width, height),
        }
    }

    pub fn commit(&mut self, change: &mut EntityStoreChange, time: u64) {
        self.spatial_hash.update(&self.entity_store, change, time);
        self.entity_store.commit_change(change);
    }
}
