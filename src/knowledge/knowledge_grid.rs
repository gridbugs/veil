use entity_store::EntityStore;
use spatial_hash::SpatialHashCell;
use observation::ObservationMetadata;
use cgmath::Vector2;

pub trait KnowledgeGrid {
    fn update_cell(&mut self, coord: Vector2<i32>, spatial_hash_cell: &SpatialHashCell,
                   entity_store: &EntityStore) -> ObservationMetadata;

    fn set_time(&mut self, time: u64);
}
