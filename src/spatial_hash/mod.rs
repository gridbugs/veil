#![allow(unreachable_patterns)]
use std::collections::HashSet;
use entity_store::{EntityId, EntityStore, EntityStoreChange, ComponentType};
use grid::{StaticGridIdx, StaticGrid};

imports!{}

mod coord;
#[macro_use] mod generated_component_list_macros;

type Position = position_type!();

spatial_hash_cell_decl!{SpatialHashCell}

impl Default for SpatialHashCell {
    fn default() -> Self {
        spatial_hash_cell_cons!{SpatialHashCell}
    }
}

impl SpatialHashCell {
    fn remove_implicit(&mut self, entity_id: EntityId, store: &EntityStore, change: &EntityStoreChange) {
        if !change.removals.contains(entity_id, ComponentType::Opacity) {
            if let Some(v) = store.opacity.get(&entity_id) {
                self.opacity_total -= *v;
            }
        }
        if !change.removals.contains(entity_id, ComponentType::Solid) {
            if store.solid.contains(&entity_id) {
                self.solid_count -= 1;
            }
        }
        if !change.removals.contains(entity_id, ComponentType::Enemy) {
            if store.enemy.contains(&entity_id) {
                self.enemy_set.remove(&entity_id);
            }
        }

        self.entities.remove(&entity_id);
    }

    fn insert_implicit(&mut self, entity_id: EntityId, store: &EntityStore, change: &EntityStoreChange) {
        if !change.removals.contains(entity_id, ComponentType::Opacity) {
            if let Some(v) = store.opacity.get(&entity_id) {
                self.opacity_total += *v;
            }
        }
        if !change.removals.contains(entity_id, ComponentType::Solid) {
            if store.solid.contains(&entity_id) {
                self.solid_count += 1;
            }
        }
        if !change.removals.contains(entity_id, ComponentType::Enemy) {
            if store.enemy.contains(&entity_id) {
                self.enemy_set.insert(entity_id);
            }
        }

        self.entities.insert(entity_id);
    }
}

pub struct SpatialHashTable {
    grid: StaticGrid<SpatialHashCell>,
}

impl SpatialHashTable {
    pub fn new(width: usize, height: usize) -> Self {
        SpatialHashTable {
            grid: StaticGrid::new_default(width, height),
        }
    }

    pub fn width(&self) -> usize { self.grid.width() }
    pub fn height(&self) -> usize { self.grid.height() }

    pub fn get<I: StaticGridIdx>(&self, index: I) -> Option<&SpatialHashCell> {
        self.grid.get(index)
    }

    pub fn update(&mut self, store: &EntityStore, change: &EntityStoreChange, time: u64) {

        for (entity_id, component_type) in change.removals.iter() {
            if let Some(position) = store.position.get(&entity_id) {
                if let Some(mut cell) = self.grid.get_mut(*position) {
                    match component_type {
                        ComponentType::Position => {
                            cell.remove_implicit(entity_id, store, change);
                        }
                        ComponentType::Opacity => {
                            if let Some(v) = store.opacity.get(&entity_id) {
                                cell.opacity_total -= *v;
                            }
                        }
                        ComponentType::Solid => {
                            if store.solid.contains(&entity_id) {
                                cell.solid_count -= 1;
                            }
                        }
                        ComponentType::Enemy => {
                            if store.enemy.contains(&entity_id) {
                                cell.enemy_set.remove(&entity_id);
                            }
                        }
                        _ => {
                            // prevent the last_updated field from being changed
                            continue;
                        }
                    }
                    cell.last_updated = time;
                }
            }
        }

        for (entity_id, position) in change.insertions.position.iter() {
            if let Some(current) = store.position.get(entity_id) {
                if let Some(mut cell) = self.grid.get_mut(*current) {
                    if !change.removals.contains(*entity_id, ComponentType::Position) {
                        cell.remove_implicit(*entity_id, store, change);
                        cell.last_updated = time;
                    }
                }
            }

            if let Some(mut cell) = self.grid.get_mut(*position) {
                cell.insert_implicit(*entity_id, store, change);
                cell.last_updated = time;
            }
        }

        // At this point we've processed all removed components, and all
        // entities that were moved or added to the table, assuming that
        // the values of their components weren't modified as prt of the
        // same change. Now we'll take care of any changes to component
        // values.

        for (entity_id, new) in change.insertions.opacity.iter() {
            if let Some(position) = post_change_get!(store, change, *entity_id, position) {
                let old = if change.removals.contains(*entity_id, ComponentType::Opacity) {
                    0.0
                } else {
                    store.opacity.get(entity_id).map(Clone::clone).unwrap_or(0.0)
                };
                let increase = new - old;
                if let Some(mut cell) = self.grid.get_mut(*position) {
                    cell.opacity_total += increase;
                }
            }
        }

        for entity_id in change.insertions.solid.iter() {
            if let Some(position) = post_change_get!(store, change, *entity_id, position) {
                if !store.solid.contains(&entity_id) || change.removals.contains(*entity_id, ComponentType::Solid) {
                    if let Some(mut cell) = self.grid.get_mut(*position) {
                        cell.solid_count += 1;
                    }
                }
            }
        }

        for entity_id in change.insertions.enemy.iter() {
            if let Some(position) = post_change_get!(store, change, *entity_id, position) {
                if !store.enemy.contains(&entity_id) || change.removals.contains(*entity_id, ComponentType::Enemy) {
                    if let Some(mut cell) = self.grid.get_mut(*position) {
                        cell.enemy_set.insert(*entity_id);
                    }
                }
            }
        }
    }
}
