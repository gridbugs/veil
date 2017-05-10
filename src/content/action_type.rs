use entity_store::*;
use direction::Direction;
use content::actions;

#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Walk(EntityId, Direction),
    CloseDoor(EntityId),
    OpenDoor(EntityId),
}

impl ActionType {
    pub fn populate(self, change: &mut EntityStoreChange, entity_store: &EntityStore) {
        match self {
            ActionType::Walk(id, direction) => actions::walk(change, entity_store, id, direction),
            ActionType::OpenDoor(id) => actions::open_door(change, id),
            ActionType::CloseDoor(id) => actions::close_door(change, id),
        }
    }
}
