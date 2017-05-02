use entity_store::*;
use content::*;
use cgmath::Vector2;

pub fn player(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.insertions.position.insert(entity_id, position);
    change.insertions.player.insert(entity_id);
    change.insertions.tile.insert(entity_id, ComplexTile::Simple(TileType::Player));
    change.insertions.tile_priority.insert(entity_id, 4);
}

pub fn stone_floor(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.insertions.position.insert(entity_id, position);
    change.insertions.floor.insert(entity_id);
    change.insertions.tile.insert(entity_id, ComplexTile::Simple(TileType::StoneFloor));
    change.insertions.tile_priority.insert(entity_id, 1);
}

pub fn wall(change: &mut EntityStoreChange, entity_id: EntityId, position: Vector2<i32>) {
    change.insertions.position.insert(entity_id, position);
    change.insertions.solid.insert(entity_id);
    change.insertions.opacity.insert(entity_id, 1.0);
    change.insertions.tile.insert(entity_id, ComplexTile::Wall { front: TileType::WallFront, top: TileType::WallTop });
    change.insertions.tile_priority.insert(entity_id, 2);
}
