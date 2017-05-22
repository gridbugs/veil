use rand::Rng;
use entity_store::*;
use content::*;
use straight_line::*;
use cgmath::Vector2;

pub fn player(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.player.insert(entity_id);
    change.collider.insert(entity_id);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::Player));
    change.tile_priority.insert(entity_id, 4);
}

pub fn undead(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.enemy.insert(entity_id);
    change.collider.insert(entity_id);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::Undead));
    change.tile_priority.insert(entity_id, 4);
}

pub fn stone_floor(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.floor.insert(entity_id);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::StoneFloor));
    change.tile_priority.insert(entity_id, 1);
}

pub fn wall(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.solid.insert(entity_id);
    change.opacity.insert(entity_id, 1.0);
    change.tile.insert(entity_id, ComplexTile::Wall { front: TileType::WallFront, top: TileType::WallTop });
    change.tile_priority.insert(entity_id, 2);
}

pub fn door(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>, state: DoorState) {
    change.position.insert(entity_id, position);
    change.tile_priority.insert(entity_id, 2);
    change.door_state.insert(entity_id, state);

    if state == DoorState::Closed {
        change.tile.insert(entity_id, ComplexTile::Wall { front: TileType::ClosedDoorFront, top: TileType::ClosedDoorTop });
        change.solid.insert(entity_id);
        change.opacity.insert(entity_id, 1.0);
    } else {
        change.tile.insert(entity_id, ComplexTile::Wall { front: TileType::OpenDoorFront, top: TileType::OpenDoorTop });
        change.opacity.insert(entity_id, 0.0);
    }
}

pub fn rain<R: Rng>(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>, rng: &mut R) {
    change.rain.insert(entity_id);
    change.position.insert(entity_id, position);
    change.tile_priority.insert(entity_id, 2);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::Rain));
    change.forgetable.insert(entity_id);

    let length = 8;
    let mut trajectory = FiniteAbsoluteLineTraverse::new_offset(position, Vector2::new(0, length));
    for _ in 0..(rng.next_u32() % length as u32) {
        trajectory.step_in_place();
    }
    change.finite_trajectory.insert(entity_id, trajectory);
}
