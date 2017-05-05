use std::collections::{HashMap, HashSet, hash_map};

#[macro_use] mod generated_component_list_macros;
#[macro_use] pub mod post_change;
#[macro_use] pub mod migration;

entity_store_imports!{}

const NUM_COMPONENTS: usize = num_components!();
const COMPONENT_BITS: usize = component_bits!();

component_type_decl!{ComponentType}

entity_store_decl!{EntityStore}

impl EntityStore {
    pub fn new() -> Self {
        entity_store_cons!(EntityStore)
    }

    pub fn commit_change(&mut self, change: &mut EntityStoreChange) {
        commit_change!(self, change)
    }

    pub fn commit_change_into_change(&mut self, change: &mut EntityStoreChange, dest: &mut EntityStoreChange) {
        commit_change_into!(self, change, dest)
    }

    pub fn commit_change_into_store(&mut self, change: &mut EntityStoreChange, dest: &mut EntityStore) {
        commit_change_into!(self, change, dest)
    }
}

const ENTITY_ID_BITS: usize = 64 - COMPONENT_BITS;
const MAX_ENTITY_ID: u64 = (1 << (ENTITY_ID_BITS as u64)) - 1;
const ENTITY_ID_MASK: u64 = MAX_ENTITY_ID;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct EntityId(u64);

impl EntityId {
    pub fn new(value: u64) -> Self {
        EntityId(value)
    }
    pub fn is_valid(self) -> bool {
        self.0 & !ENTITY_ID_MASK == 0
    }
}

impl ComponentType {
    fn index(self) -> u64 { self as u64 }
    fn shifted_index(self) -> u64 { self.index() << ENTITY_ID_BITS }
}

#[derive(Debug, Clone, Copy)]
pub enum DataChangeType<T> {
    Insert(T),
    Remove,
}

#[derive(Debug, Clone, Copy)]
pub enum FlagChangeType {
    Insert,
    Remove,
}

#[derive(Debug, Clone)]
pub struct DataComponentChange<T>(HashMap<EntityId, DataChangeType<T>>);
#[derive(Debug, Clone)]
pub struct FlagComponentChange(HashMap<EntityId, FlagChangeType>);

impl<T> DataComponentChange<T> {
    pub fn get(&self, id: &EntityId) -> Option<&DataChangeType<T>> {
        self.0.get(&id)
    }
    pub fn iter(&self) -> hash_map::Iter<EntityId, DataChangeType<T>> {
        self.0.iter()
    }
    pub fn insert(&mut self, id: EntityId, value: T) {
        self.0.insert(id, DataChangeType::Insert(value));
    }
    pub fn remove(&mut self, id: EntityId) {
        self.0.insert(id, DataChangeType::Remove);
    }
}
impl FlagComponentChange {
    pub fn iter(&self) -> hash_map::Iter<EntityId, FlagChangeType> {
        self.0.iter()
    }
    pub fn insert(&mut self, id: EntityId) {
        self.0.insert(id, FlagChangeType::Insert);
    }
    pub fn remove(&mut self, id: EntityId) {
        self.0.insert(id, FlagChangeType::Remove);
    }
}

entity_store_change_decl!{EntityStoreChange}

impl EntityStoreChange {
    pub fn new() -> Self {
        entity_store_change_cons!(EntityStoreChange)
    }
    pub fn remove_entity(&mut self, entity: EntityId, store: &EntityStore) {
        remove_entity!(self, entity, store);
    }
}
