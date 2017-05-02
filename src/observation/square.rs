use cgmath::Vector2;
use entity_store::EntityStore;
use spatial_hash::SpatialHashTable;
use knowledge::PlayerKnowledgeGrid;

pub fn observe(eye: Vector2<i32>, distance: u32, spatial_hash: &SpatialHashTable, entity_store: &EntityStore,
               time: u64, knowledge: &mut PlayerKnowledgeGrid) -> bool {

    let mut changed = false;

    for i in (eye.y - (distance as i32))...(eye.y + (distance as i32)) {
        for j in (eye.x - (distance as i32))...(eye.x + (distance as i32)) {
            let coord = Vector2::new(j, i);
            if let Some(cell) = spatial_hash.get(coord) {
                if knowledge.update_cell(coord, cell, entity_store, time) {
                    changed = true;
                }
            }
        }
    }

    changed
}
