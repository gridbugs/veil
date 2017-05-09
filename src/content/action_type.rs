use entity_store::*;
use direction::Direction;
use content::actions;

#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Walk(EntityId, Direction),
}

impl ActionType {
    pub fn populate(self, change: &mut EntityStoreChange, entity_store: &EntityStore) {
        match self {
            ActionType::Walk(id, direction) => actions::walk(change, entity_store, id, direction),
        }
    }
}
