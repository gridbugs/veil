use grid::StaticGridIdx;
use entity_store::EntityStore;
use spatial_hash::SpatialHashCell;

pub trait KnowledgeGrid {
    fn update_cell<I: StaticGridIdx>(&mut self, coord: I, spatial_hash_cell: &SpatialHashCell,
                                     entity_store: &EntityStore, time: u64) -> bool;
}
