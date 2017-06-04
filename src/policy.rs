use rand::Rng;
use cgmath::Vector2;
use entity_store::*;
use spatial_hash::*;
use straight_line::*;
use content::*;
use frame::*;
use reaction::Reaction;

pub struct GamePolicy {
    to_cancel: Vec<EntityId>,
    entities_to_remove: Vec<EntityId>,
}

enum RainUpdate {
    Fall(Vector2<i32>, FiniteAbsoluteLineTraverse),
    Reset(FiniteAbsoluteLineTraverse),
}

impl GamePolicy {

    pub fn new() -> Self {
        GamePolicy {
            to_cancel: Vec::new(),
            entities_to_remove: Vec::new(),
        }
    }

    fn update_finite_trajectory(&self, id: EntityId, entity_store: &EntityStore,
                                    spatial_hash: &SpatialHashTable) -> Option<RainUpdate> {

        if let Some(trajectory) = entity_store.finite_trajectory.get(&id) {
            if let Some((new_position, new_trajectory)) = trajectory.step() {
                if spatial_hash.contains(new_position) {
                    return Some(RainUpdate::Fall(new_position, new_trajectory));
                } else {
                    let wrapped_position = Vector2::new(new_position.x % spatial_hash.width() as i32,
                                                        new_position.y % spatial_hash.height() as i32);
                    let mut trajectory = new_trajectory.reset_position(wrapped_position);
                    trajectory.step_in_place();
                    return Some(RainUpdate::Fall(wrapped_position, trajectory));
                }
            }

            return Some(RainUpdate::Reset(*trajectory));
        }

        None
    }

    pub fn on_frame_animate<R: Rng>(&mut self, frame: Frame, entity_store: &EntityStore, spatial_hash: &SpatialHashTable,
                                    rng: &mut R, change: &mut EntityStoreChange) {
        if frame.animation_mode() == AnimationMode::RealTime && frame.id() % 8 != 0 {
            return;
        }
        for id in entity_store.rain.iter() {
            if let Some(update) = self.update_finite_trajectory(*id, entity_store, spatial_hash) {
                match update {
                    RainUpdate::Fall(new_position, new_trajectory) => {
                        if let Some(cell) = spatial_hash.get(new_position) {
                            if cell.inside_count > 0 {
                                change.invisible.insert(*id);
                            } else {
                                change.invisible.remove(*id);
                            }
                        }

                        if new_trajectory.is_complete() {
                            change.tile.insert(*id, ComplexTile::Simple(TileType::Splash));
                            change.splash.insert(*id);
                        }

                        change.position.insert(*id, new_position);
                        change.finite_trajectory.insert(*id, new_trajectory);
                    }
                    RainUpdate::Reset(trajectory) => {
                        if entity_store.splash.contains(id) {
                            change.splash.remove(*id);
                            continue;
                        }
                        let x = (rng.gen::<usize>() % (spatial_hash.width())) as i32;
                        let y = (rng.gen::<usize>() % (spatial_hash.height())) as i32;
                        let new_position = Vector2::new(x, y);
                        change.position.insert(*id, new_position);

                        if let Some(cell) = spatial_hash.get(new_position) {
                            if cell.inside_count > 0 {
                                change.invisible.insert(*id);
                            } else {
                                change.invisible.remove(*id);
                            }
                        }

                        let mut new_trajectory = trajectory.reset(new_position);
                        new_trajectory.step_in_place();
                        change.finite_trajectory.insert(*id, new_trajectory);

                        change.tile.insert(*id, ComplexTile::Simple(TileType::Rain));
                    }
                }
            }
        }
    }

    pub fn has_unresolved_realtime_frames(&mut self, entity_store: &EntityStore) -> bool {
        !entity_store.realtime.is_empty()
    }

    fn handle_realtime(&mut self, first_frame: Frame, frame: Frame,
                       change: &mut EntityStoreChange, entity_store: &EntityStore) {

        let relative_frame_diff = frame.id() - first_frame.id();

        for id in entity_store.realtime.iter() {

            if let Some(period) = entity_store.realtime_period.get(&id) {
                if relative_frame_diff % period != 0 {
                    continue;
                }
            }

            if let Some(trajectory) = entity_store.infinite_trajectory.get(&id) {
                let mut new_trajectory = *trajectory;
                change.position.insert(*id, new_trajectory.step_in_place());
                change.infinite_trajectory.insert(*id, new_trajectory);
            }
        }
    }

    fn handle_collisions(&mut self, entity_store: &EntityStore, spatial_hash: &SpatialHashTable,
                         id: EntityId, position: Vector2<i32>,
                         reactions: &mut Vec<Reaction>) {

        if let Some(cell) = spatial_hash.get(position) {

            // if it's a closed door and we can open doors, open the door instead
            if let Some(door_id) = cell.door_set.iter().next() {
                if cell.solid_count > 0 && entity_store.door_opener.contains(&id) {
                    reactions.push(Reaction::immediate(ActionType::OpenDoor(*door_id)));
                    self.to_cancel.push(id);
                    return;
                }
            }

            if let Some(shootable_id) = cell.shootable_set.iter().next() {
                if entity_store.bullet.contains(&id) {
                    self.entities_to_remove.push(*shootable_id);
                    self.entities_to_remove.push(id);
                }
            }

            if cell.solid_count > 0 {
                // we hit a solid cell
                if entity_store.bullet.contains(&id) {
                    // bullets get removed completely
                    self.entities_to_remove.push(id);
                } else {
                    // everything else just gets stopped
                    self.to_cancel.push(id);
                }
            }
        }
    }

    pub fn on_change(&mut self, first_frame: Frame, frame: Frame,
                     change: &mut EntityStoreChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable,
                     reactions: &mut Vec<Reaction>) {

        self.handle_realtime(first_frame, frame, change, entity_store);

        for (id, position_change) in change.position.iter() {
            if let &DataChangeType::Insert(position) = position_change {
                if entity_store.collider.contains(id) {
                    self.handle_collisions(entity_store, spatial_hash, *id, position, reactions);
                }
            }
        }

        for id in self.to_cancel.drain(..) {
            change.position.cancel(id);
        }

        for id in self.entities_to_remove.drain(..) {
            change.remove_entity(id, entity_store);
        }
    }
}
