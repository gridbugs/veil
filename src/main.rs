#![allow(dead_code)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate enum_primitive;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use enum_primitive::FromPrimitive;

#[macro_use] mod component_list_macros;

component_list_imports!{}

const NUM_COMPONENTS: usize = component_list_num_components!();
const COMPONENT_BITS: usize = component_list_component_bits!();

component_list_component_type_decl!{ComponentType}

component_list_entity_store_decl!{EntityStore}

impl EntityStore {
    pub fn new() -> Self {
        component_list_entity_store_cons!(EntityStore)
    }

    fn commit_insertions(&mut self, insertions: &mut EntityStore) {
        component_list_commit_insertions!(self, insertions)
    }

    fn swap_component(&mut self, a: EntityId, b: EntityId, component_type: ComponentType) {
        component_list_swap_component!(self, a, b, component_type, ComponentType);
    }

    fn remove_component(&mut self, entity: EntityId, component_type: ComponentType) {
        component_list_remove_component!(self, entity, component_type, ComponentType);
    }

    pub fn commit(&mut self, change: &mut EntityStoreChange) {
        for (entity_a, entity_b, component_type) in
            change.swaps.swaps.drain()
                .filter(|&(k, v)| k.entity().0 < v.0)
                .map(|(k, v)| (k.entity(), v, k.component()))
        {
            self.swap_component(entity_a, entity_b, component_type);
        }

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

fn swap_hash_map<K: Hash + Eq, V>(hm: &mut HashMap<K, V>, a_key: K, b_key: K) {
    let maybe_a = hm.remove(&a_key);
    let maybe_b = hm.remove(&b_key);
    maybe_a.map(|a| hm.insert(b_key, a));
    maybe_b.map(|b| hm.insert(a_key, b));
}

fn swap_hash_set<T: Hash + Eq>(s: &mut HashSet<T>, a: T, b: T) {
    let had_a = s.remove(&a);
    let had_b = s.remove(&b);
    if had_a { s.insert(b); }
    if had_b { s.insert(a); }
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
}

pub fn is_valid_entity_id(entity: EntityId) -> bool {
    entity.0 & !ENTITY_ID_MASK == 0
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
pub struct EntityComponentCombination(u64);
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
}

pub struct EntityStoreSwaps {
    swaps: HashMap<EntityComponentCombination, EntityId>,
}

impl EntityStoreSwaps {
    fn new() -> Self {
        EntityStoreSwaps {
            swaps: HashMap::new(),
        }
    }
    pub fn swap(&mut self, a: EntityId, b: EntityId, component: ComponentType) {

        let comb_a = EntityComponentCombination::new(a, component);
        let comb_b = EntityComponentCombination::new(b, component);

        let current_a = self.swaps.get(&comb_a).map(Clone::clone).unwrap_or(a);
        let current_b = self.swaps.get(&comb_b).map(Clone::clone).unwrap_or(b);

        self.swaps.insert(comb_a, current_b);
        self.swaps.insert(comb_b, current_a);
    }
}

pub struct EntityStoreChange {
    pub insertions: EntityStore,
    pub removals: EntityComponentSet,
    pub swaps: EntityStoreSwaps,
}

impl EntityStoreChange {
    pub fn new() -> Self {
        EntityStoreChange {
            insertions: EntityStore::new(),
            removals: EntityComponentSet::new(),
            swaps: EntityStoreSwaps::new(),
        }
    }
}

fn main() {

    let mut entity_store = EntityStore {
        position: HashMap::new(),
        solid: HashSet::new(),
    };

    println!("{:?}", entity_store);

    let mut change = EntityStoreChange::new();

    let e0 = EntityId::new(0);
    let e1 = EntityId::new(1);

    change.insertions.position.insert(e0, cgmath::Vector2::new(1, 2));
    change.insertions.position.insert(e1, cgmath::Vector2::new(3, 4));

    entity_store.commit(&mut change);

    println!("{:?}", entity_store);

    change.swaps.swap(e0, e1, ComponentType::Position);
    entity_store.commit(&mut change);

    println!("{:?}", entity_store);

    change.removals.insert(e0, ComponentType::Position);
    entity_store.commit(&mut change);

    println!("{:?}", entity_store);
}
