use entity_store::*;
use entity_id_allocator::EntityIdAllocator;
use content::actions;
use geometry::direction::Direction;
use geometry::straight_line::InfiniteAbsoluteLineTraverse;

#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Null,
    Walk(EntityId, Direction),
    CloseDoor(EntityId),
    OpenDoor(EntityId),
    FireBullet(InfiniteAbsoluteLineTraverse),
    Remove(EntityId),
}

impl ActionType {
    pub fn populate(self, change: &mut EntityStoreChange, entity_store: &EntityStore, ids: &mut EntityIdAllocator) {
        match self {
            ActionType::Null => (),
            ActionType::Walk(id, direction) => actions::walk(change, entity_store, id, direction),
            ActionType::OpenDoor(id) => actions::open_door(change, id),
            ActionType::CloseDoor(id) => actions::close_door(change, id),
            ActionType::FireBullet(traverse) => actions::fire_bullet(change, traverse, ids),
            ActionType::Remove(id) => actions::remove(change, id, entity_store),
        }
    }
}
