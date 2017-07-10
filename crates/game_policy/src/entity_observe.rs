use std::result;
use game_data::entity_store::{EntityId, EntityStore};
use game_data::spatial_hash::SpatialHashTable;
use knowledge::KnowledgeGrid;
use observation::{shadowcast, omniscient, ObservationMetadata};

#[derive(Debug)]
pub enum Error {
    MissingPosition,
    MissingVisionDistance,
}
pub type Result<T> = result::Result<T, Error>;

pub fn entity_observe<K: KnowledgeGrid>(
    id: EntityId,
    entity_store: &EntityStore,
    spatial_hash: &SpatialHashTable,
    time: u64,
    knowledge: &mut K,
    shadowcast: &mut shadowcast::ShadowcastEnv) -> Result<ObservationMetadata> {

    let position = entity_store.position.get(&id).ok_or(Error::MissingPosition)?;
    let vision_distance = entity_store.vision_distance.get(&id).ok_or(Error::MissingVisionDistance)?;

    if entity_store.omniscient.contains(&id) {
        return Ok(omniscient::observe(spatial_hash, entity_store, time, knowledge));
    }

    Ok(shadowcast::observe(
        shadowcast,
        *position,
        spatial_hash,
        *vision_distance,
        entity_store,
        time,
        knowledge
    ))
}

