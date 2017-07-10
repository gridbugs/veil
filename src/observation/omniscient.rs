use game_data::spatial_hash::SpatialHashTable;
use game_data::entity_store::EntityStore;
use knowledge::KnowledgeGrid;
use observation::ObservationMetadata;

pub fn observe<K: KnowledgeGrid>(world: &SpatialHashTable, entity_store: &EntityStore,
                                 time: u64, knowledge: &mut K) -> ObservationMetadata {


    knowledge.set_time(time);

    let mut metadata = Default::default();

    for (coord, cell) in izip!(world.coord_iter(), world.iter()) {
        metadata |= knowledge.update_cell(coord, cell, entity_store);
    }

    metadata
}
