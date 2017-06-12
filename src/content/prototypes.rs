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
    change.forgetable.insert(entity_id);
    change.turn_period.insert(entity_id, 1);
    change.behaviour_type.insert(entity_id, BehaviourType::Player);
    change.vision_distance.insert(entity_id, 20);
    change.door_opener.insert(entity_id);
}

pub fn undead(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.enemy.insert(entity_id);
    change.npc.insert(entity_id);
    change.collider.insert(entity_id);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::Undead));
    change.tile_priority.insert(entity_id, 4);
    change.forgetable.insert(entity_id);
    change.turn_period.insert(entity_id, 2);
    change.behaviour_type.insert(entity_id, BehaviourType::Undead);
    change.vision_distance.insert(entity_id, 10);
    change.door_opener.insert(entity_id);
    change.shootable.insert(entity_id);
    change.veil_change.insert(entity_id);
}

pub fn stone_floor(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.floor.insert(entity_id);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::StoneFloor));
    change.tile_priority.insert(entity_id, 1);
    change.veil_slot.insert(entity_id);
}

pub fn brick_floor(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.floor.insert(entity_id);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::BrickFloor));
    change.tile_priority.insert(entity_id, 1);
    change.veil_slot.insert(entity_id);
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

pub fn bullet(change: &mut EntityStoreChange, entity_id: EntityId, mut traverse: InfiniteAbsoluteLineTraverse) {
    change.position.insert(entity_id, traverse.step_in_place());
    change.infinite_trajectory.insert(entity_id, traverse);
    change.tile_priority.insert(entity_id, 2);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::Bullet));
    change.forgetable.insert(entity_id);
    change.realtime.insert(entity_id);
    change.bullet.insert(entity_id);
    change.collider.insert(entity_id);
    change.realtime_period.insert(entity_id, 2);
}

pub fn page(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::Page));
    change.tile_priority.insert(entity_id, 2);
    change.page.insert(entity_id);
}

pub fn water<R: Rng>(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>, rng: &mut R) {
    change.position.insert(entity_id, position);
    change.water.insert(entity_id);
    if rng.gen::<f64>() < WATER_FOREGROUND_PROBABILITY {
        change.tile.insert(entity_id, ComplexTile::Simple(TileType::WaterWithFg));
    } else {
        change.tile.insert(entity_id, ComplexTile::Simple(TileType::WaterBgOnly));
    }
    change.tile_priority.insert(entity_id, 2);
}
