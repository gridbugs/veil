#![allow(unreachable_patterns)]
#![allow(unused_variables)]
use std::collections::HashSet;
use entity_store::{EntityId, EntityStore, EntityStoreChange, ComponentType};
use grid::{StaticGridIdx, StaticGrid};

#[macro_use] mod generated_component_list_macros;

spatial_hash_imports!{}

mod coord;

type Position = position_type!();

spatial_hash_cell_decl!{SpatialHashCell}

impl Default for SpatialHashCell {
    fn default() -> Self {
        spatial_hash_cell_cons!{SpatialHashCell}
    }
}

impl SpatialHashCell {
    fn remove_implicit(&mut self, entity_id: EntityId, store: &EntityStore, change: &EntityStoreChange) {
        remove_implicit!(self, entity_id, store, change);
        self.entities.remove(&entity_id);
    }

    fn insert_implicit(&mut self, entity_id: EntityId, store: &EntityStore, change: &EntityStoreChange) {
        insert_implicit!(self, entity_id, store, change);
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
            if let Some(position) = position!(store).get(&entity_id) {
                if let Some(mut cell) = self.grid.get_mut(*position) {
                    update_match_stmt!(component_type, cell, entity_id, store, change);
                    cell.last_updated = time;
                }
            }
        }

        for (entity_id, position) in position!(change.insertions).iter() {
            if let Some(current) = position!(store).get(entity_id) {
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

        update_component_loops!(self, store, change, time);
    }
}
