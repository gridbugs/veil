use behaviour::{BehaviourEnv, BehaviourState};
use knowledge::{PlayerKnowledgeGrid, PlayerKnowledgeCell};
use content::ActionType;
use entity_store::{EntityId, EntityStore};
use geometry::direction::DirectionsCardinal;
use geometry::grid_search::bfs_coord;

pub fn attack(id: EntityId,
              entity_store: &EntityStore,
              knowledge: &PlayerKnowledgeGrid,
              env: &mut BehaviourEnv,
              state: &mut BehaviourState) -> Option<ActionType> {

    let position = *entity_store.position.get(&id).expect("Missing position");

    let dest = if let Some(dest) = knowledge.player_coord() {
        dest
    } else {
        return None;
    };

    let can_enter = |cell: &PlayerKnowledgeCell| {
        return !cell.solid || cell.door.is_some();
    };

    if let Err(_) = bfs_coord(&mut env.search_env, knowledge, position, DirectionsCardinal,
                              dest, can_enter, &mut state.path) {
        return None;
    }

    state.path_idx = 0;

    if let Some(step) = state.current_step() {
        state.path_idx += 1;
        state.prev_step = Some(step);
        Some(ActionType::Walk(id, step.direction()))
    } else {
        None
    }
}
