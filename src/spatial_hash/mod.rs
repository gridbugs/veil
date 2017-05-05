#![allow(unreachable_patterns)]
#![allow(unused_variables)]
use std::collections::HashSet;
use entity_store::{EntityId, EntityStore, EntityStoreChange, DataChangeType, FlagChangeType};
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
    fn remove(&mut self, entity_id: EntityId, store: &EntityStore) {
        remove!(self, entity_id, store);
    }

    fn insert(&mut self, entity_id: EntityId, store: &EntityStore) {
        insert!(self, entity_id, store);
    }
}

#[derive(Serialize, Deserialize)]
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
        for (entity_id, change) in position!(change).iter() {
            match change {
                &DataChangeType::Insert(position) => {
                    if let Some(current) = position!(store).get(entity_id) {
                        if let Some(mut cell) = self.grid.get_mut(*current) {
                            cell.remove(*entity_id, store);
                            cell.entities.remove(entity_id);
                            cell.last_updated = time;
                        }
                    }
                    if let Some(mut cell) = self.grid.get_mut(position) {
                        cell.insert(*entity_id, store);
                        cell.entities.insert(*entity_id);
                        cell.last_updated = time;
                    }
                }
                &DataChangeType::Remove => {
                    if let Some(current) = position!(store).get(entity_id) {
                        if let Some(mut cell) = self.grid.get_mut(*current) {
                            cell.remove(*entity_id, store);
                            cell.entities.remove(entity_id);
                            cell.last_updated = time;
                        }
                    }
                }
            }
        }

        update_component_loops!(self, store, change, time);
    }
}
