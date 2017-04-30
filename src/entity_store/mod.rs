use std::collections::{HashMap, HashSet, hash_set};
use std::hash::Hash;

use enum_primitive::FromPrimitive;

#[macro_use] mod generated_component_list_macros;

imports!{}

const NUM_COMPONENTS: usize = num_components!();
const COMPONENT_BITS: usize = component_bits!();

component_type_decl!{ComponentType}

entity_store_decl!{EntityStore}

impl EntityStore {
    pub fn new() -> Self {
        entity_store_cons!(EntityStore)
    }

    fn commit_insertions(&mut self, insertions: &mut EntityStore) {
        commit_insertions!(self, insertions)
    }

    fn remove_component(&mut self, entity: EntityId, component_type: ComponentType) {
        remove_component!(self, entity, component_type, ComponentType);
    }

    pub fn commit(&mut self, change: &mut EntityStoreChange) {
        for (entity, component_type) in
            change.removals.set.drain()
                .map(|x| (x.entity(), x.component()))
        {
            self.remove_component(entity, component_type);
        }

        self.commit_insertions(&mut change.insertions);
    }
}

fn merge_hash_maps<K: Hash + Eq, V>(a: &mut HashMap<K, V>, b: &mut HashMap<K, V>) {
    for (k, v) in b.drain() {
        a.insert(k, v);
    }
}

fn merge_hash_sets<T: Hash + Eq>(a: &mut HashSet<T>, b: &mut HashSet<T>) {
    for x in b.drain() {
        a.insert(x);
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

#[derive(Serialize, Deserialize)]
pub struct EntityComponentSet {
    set: HashSet<EntityComponentCombination>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct EntityComponentCombination(u64);
impl EntityComponentCombination {
    fn new(entity: EntityId, component: ComponentType) -> Self {
        EntityComponentCombination(component.shifted_index() | entity.0)
    }
    fn entity(self) -> EntityId {
        EntityId(self.0 & ENTITY_ID_MASK)
    }
    fn component(self) -> ComponentType {
        ComponentType::from_u64(self.0 >> ENTITY_ID_BITS).expect("invalid component type")
    }
}

impl EntityComponentSet {
    pub fn new() -> Self {
        EntityComponentSet {
            set: HashSet::new(),
        }
    }
    pub fn insert(&mut self, entity: EntityId, component: ComponentType) -> bool {
        self.set.insert(EntityComponentCombination::new(entity, component))
    }
    pub fn remove(&mut self, entity: EntityId, component: ComponentType) -> bool {
        self.set.remove(&EntityComponentCombination::new(entity, component))
    }
    pub fn contains(&mut self, entity: EntityId, component: ComponentType) -> bool {
        self.set.contains(&EntityComponentCombination::new(entity, component))
    }
    pub fn insert_all(&mut self, entity: EntityId, store: &EntityStore) {
        insert_all!(self, entity, store)
    }
    pub fn iter(&self) -> EntityComponentSetIter {
        EntityComponentSetIter(self.set.iter())
    }
}

    }
}

pub struct EntityStoreChange {
    pub insertions: EntityStore,
    pub removals: EntityComponentSet,
}

impl EntityStoreChange {
    pub fn new() -> Self {
        EntityStoreChange {
            insertions: EntityStore::new(),
            removals: EntityComponentSet::new(),
        }
    }
}
