use entity_store::*;
use content::*;
use cgmath::Vector2;

pub fn player(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.position.insert(entity_id, position);
    change.player.insert(entity_id);
    change.tile.insert(entity_id, ComplexTile::Simple(TileType::Player));
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
