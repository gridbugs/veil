use std::result;
use entity_store::*;
use spatial_hash::SpatialHashTable;
use entity_store::EntityStore;
use observation::shadowcast;
use knowledge::PlayerKnowledgeGrid;
use entity_observe;
use renderer::GameRenderer;

#[derive(Debug)]
pub enum Error {
    ObservationFailed(entity_observe::Error),
    MissingPosition,
}
pub type Result<T> = result::Result<T, Error>;

pub fn player_render<Ren: GameRenderer>(
    id: EntityId,
    entity_store: &EntityStore,
    spatial_hash: &SpatialHashTable,
    time: u64,
    knowledge: &mut PlayerKnowledgeGrid,
    shadowcast: &mut shadowcast::ShadowcastEnv,
    renderer: &mut Ren) -> Result<()> {

    let metadata = entity_observe::entity_observe(
        id,
        entity_store,
        spatial_hash,
        time,
        knowledge,
        shadowcast
    ).map_err(Error::ObservationFailed)?;

    let player_position = entity_store.position.get(&id)
        .ok_or(Error::MissingPosition)?;

    if metadata.changed {
        renderer.update_player_position(*player_position);
        renderer.update(knowledge, time);
        renderer.clear();
        renderer.draw();
        renderer.publish();
    }

    Ok(())
}
