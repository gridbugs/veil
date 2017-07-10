use entity_store::*;
use entity_id_allocator::EntityIdAllocator;
use content::*;
use geometry::direction::Direction;
use geometry::straight_line::InfiniteAbsoluteLineTraverse;

pub fn walk(change: &mut EntityStoreChange, entity_store: &EntityStore,
            id: EntityId, direction: Direction) {

    let old = entity_store.position.get(&id)
        .expect("missing position");
    let new = old + direction.vector();
    change.position.insert(id, new);
}

pub fn open_door(change: &mut EntityStoreChange, id: EntityId) {
    change.door_state.insert(id, DoorState::Open);
    change.solid.remove(id);
    change.opacity.insert(id, 0.0);
    change.tile.insert(id, ComplexTile::Wall { front: TileType::OpenDoorFront, top: TileType::OpenDoorTop });
}

pub fn close_door(change: &mut EntityStoreChange, id: EntityId) {
    change.door_state.insert(id, DoorState::Closed);
    change.solid.insert(id);
    change.opacity.insert(id, 1.0);
    change.tile.insert(id, ComplexTile::Wall { front: TileType::ClosedDoorFront, top: TileType::ClosedDoorTop });
}

pub fn fire_bullet(change: &mut EntityStoreChange, traverse: InfiniteAbsoluteLineTraverse, ids: &mut EntityIdAllocator) {
    let bullet_id = ids.allocate();
    prototypes::bullet(change, bullet_id, traverse);
}

pub fn remove(change: &mut EntityStoreChange, id: EntityId, entity_store: &EntityStore) {
    change.remove_entity(id, entity_store);
}
