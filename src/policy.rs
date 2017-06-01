use rand::Rng;
use cgmath::Vector2;
use entity_store::*;
use spatial_hash::*;
use straight_line::*;
use content::*;
use frame::*;
use reaction::Reaction;

pub struct GamePolicy;

enum RainUpdate {
    Fall(Vector2<i32>, FiniteAbsoluteLineTraverse),
    Reset(FiniteAbsoluteLineTraverse),
}

impl GamePolicy {

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

    pub fn on_change(&mut self, change: &EntityStoreChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable,
                     reactions: &mut Vec<Reaction>) -> bool {
        for (id, position_change) in change.position.iter() {
            if let &DataChangeType::Insert(position) = position_change {
                if !entity_store.collider.contains(id) {
                    continue;
                }

                if let Some(cell) = spatial_hash.get(position) {
                    if let Some(door_id) = cell.door_set.iter().next() {
                        if cell.solid_count > 0 {
                            reactions.push(Reaction::immediate(ActionType::OpenDoor(*door_id)));
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
