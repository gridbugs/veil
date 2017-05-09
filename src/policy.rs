use entity_store::*;
use spatial_hash::*;

pub struct GamePolicy;

impl GamePolicy {
    pub fn on_action(&self, change: &EntityStoreChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable) -> bool {
        for (id, position_change) in change.position.iter() {
            if !entity_store.collider.contains(id) {
                continue;
            }
            if let &DataChangeType::Insert(position) = position_change {
                if let Some(cell) = spatial_hash.get(position) {
                    if cell.solid_count > 0 {
                        return false;
                    }
                }
            }
        }

        true
    }
}
