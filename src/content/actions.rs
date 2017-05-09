use entity_store::*;
use direction::Direction;

pub fn walk(change: &mut EntityStoreChange, entity_store: &EntityStore,
            id: EntityId, direction: Direction) {

    let old = entity_store.position.get(&id)
        .expect("missing position");
    let new = old + direction.vector();
    change.position.insert(id, new);
}
