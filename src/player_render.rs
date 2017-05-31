use std::result;
use entity_store::*;
use spatial_hash::SpatialHashTable;
use entity_store::EntityStore;
use observation::shadowcast;
use knowledge::PlayerKnowledgeGrid;
use entity_observe;
use renderer::GameRendererGen;

#[derive(Debug)]
pub enum Error {
    ObservationFailed,
}
pub type Result<T> = result::Result<T, Error>;

pub fn player_render<Rdr: GameRendererGen>(
    id: EntityId,
    entity_store: &EntityStore,
    spatial_hash: &SpatialHashTable,
    time: u64,
    knowledge: &mut PlayerKnowledgeGrid,
    shadowcast: &mut shadowcast::ShadowcastEnv,
    renderer: &mut Rdr) -> Result<()> {

    let metadata = entity_observe::entity_observe(
        id,
        entity_store,
        spatial_hash,
        time,
        knowledge,
        shadowcast
    ).map_err(|_| Error::ObservationFailed)?;

    if metadata.changed {
        renderer.update(knowledge, time);
        renderer.clear();
        renderer.draw();
        renderer.publish();
    }

    Ok(())
}
