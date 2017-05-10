use entity_store::*;
use content::*;
use direction::Direction;

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
