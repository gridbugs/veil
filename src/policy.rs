use entity_store::*;
use spatial_hash::*;
use content::ActionType;

pub struct GamePolicy;

impl GamePolicy {
    pub fn on_action(&self, change: &EntityStoreChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable,
                     reactions: &mut Vec<ActionType>) -> bool {
        for (id, position_change) in change.position.iter() {
            if let &DataChangeType::Insert(position) = position_change {
                if !entity_store.collider.contains(id) {
                    continue;
                }

                if let Some(cell) = spatial_hash.get(position) {
                    if let Some(door_id) = cell.door_set.iter().next() {
                        if cell.solid_count > 0 {
                            reactions.push(ActionType::OpenDoor(*door_id));
                            return false;
                        }
                    } else if cell.solid_count > 0 {
                        return false;
                    }

                }
            }
        }

        true
    }
}
