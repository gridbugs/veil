#![allow(dead_code)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate enum_primitive;

mod entity_store;

use entity_store::{EntityStore, EntityStoreChange, ComponentType, EntityId};

fn main() {

    let mut entity_store = EntityStore::new();

    println!("{:?}", entity_store);

    let mut change = EntityStoreChange::new();

    let e0 = EntityId::new(0);
    let e1 = EntityId::new(1);

    change.insertions.position.insert(e0, cgmath::Vector2::new(1, 2));
    change.insertions.position.insert(e1, cgmath::Vector2::new(3, 4));
    change.insertions.solid.insert(e1);

    entity_store.commit(&mut change);

    println!("{:?}", entity_store);

    change.swaps.swap(e0, e1, ComponentType::Position);
    entity_store.commit(&mut change);

    println!("{:?}", entity_store);

    change.removals.insert(e0, ComponentType::Position);
    entity_store.commit(&mut change);

    println!("{:?}", entity_store);

    change.removals.insert_all(e1, &entity_store);
    entity_store.commit(&mut change);

    println!("{:?}", entity_store);
}
