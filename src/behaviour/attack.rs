use behaviour::{BehaviourEnv, BehaviourState};
use knowledge::{PlayerKnowledgeGrid, PlayerKnowledgeCell};
use content::ActionType;
use direction::DirectionsCardinal;
use entity_store::{EntityId, EntityStore};
use grid_search::bfs_predicate;

fn search_can_enter(cell: &PlayerKnowledgeCell) -> bool {
    !cell.solid || cell.door.is_some()
}

fn search_predicate(cell: &PlayerKnowledgeCell) -> bool {
    cell.player
}

pub fn attack(id: EntityId,
              entity_store: &EntityStore,
              knowledge: &PlayerKnowledgeGrid,
              env: &mut BehaviourEnv,
              state: &mut BehaviourState) -> Option<ActionType> {

    let position = *entity_store.position.get(&id).expect("Missing position");

    if let Err(_) = bfs_predicate(&mut env.search_env, knowledge, position, DirectionsCardinal,
                                  search_predicate, search_can_enter, &mut state.path) {
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
