use behaviour::{BehaviourEnv, BehaviourState};
use knowledge::{PlayerKnowledgeGrid, PlayerKnowledgeCell};
use observation::ObservationMetadata;
use content::{ActionType, DoorState};
use direction::DirectionsCardinal;
use entity_store::{EntityId, EntityStore};
use grid_search::{bfs_best, SearchEnv, Step};
use invert_ord::InvertOrd;
use cgmath::Vector2;

const BFS_MAX: usize = 500;

fn maybe_make_step(position: Vector2<i32>,
                   knowledge: &PlayerKnowledgeGrid,
                   observation_metadata: ObservationMetadata,
                   time: u64,
                   state: &mut BehaviourState) -> Option<Step> {

    if observation_metadata.new {
        return None;
    }

    if let Some(destination) = state.path.destination() {
        if knowledge.is_visible(destination, time) {
            return None;
        }
    }

    for step in state.path_iter() {
        if let Some(cell) = knowledge.get(step.to_coord()) {
            if cell.solid && cell.door.is_none() {
                return None;
            }
            if !cell.is_visible(time) {
                break;
            }
        }
    }

    state.current_step().and_then(|step| {
        if step.from_coord() == position {
            Some(step)
        } else {
            None
        }
    })
}

fn search_score(cell: &PlayerKnowledgeCell) -> InvertOrd<u64> {
    InvertOrd::new(cell.last_updated)
}

fn make_step(id: EntityId,
             position: Vector2<i32>,
             knowledge: &PlayerKnowledgeGrid,
             observation_metadata: ObservationMetadata,
             time: u64,
             state: &mut BehaviourState,
             search_env: &mut SearchEnv) -> Option<Step> {

    if let Some(step) = maybe_make_step(position, knowledge, observation_metadata, time, state) {
        Some(step)
    } else {
        let can_enter = |cell: &PlayerKnowledgeCell| {
            if cell.last_updated == 0 {
                return true;
            }
            if let Some(enemy_id) = cell.enemy {
                if enemy_id != id && cell.last_updated == time {
                    return false;
                }
            }
            return !cell.solid || cell.door.is_some();
        };
        bfs_best(search_env, knowledge, position, DirectionsCardinal, search_score, can_enter, &mut state.path, BFS_MAX)
            .expect("Failed to search");
        state.path_idx = 0;
        state.path.first()
    }
}

pub fn patrol(id: EntityId,
              entity_store: &EntityStore,
              knowledge: &PlayerKnowledgeGrid,
              observation_metadata: ObservationMetadata,
              time: u64,
              env: &mut BehaviourEnv,
              state: &mut BehaviourState) -> Option<ActionType> {

    let position = *entity_store.position.get(&id).expect("Missing position");

    let step = if let Some(step) = make_step(id, position, knowledge, observation_metadata, time, state, &mut env.search_env) {
        step
    } else {
        return None;
    };

    if let Some(prev_step) = state.prev_step {
        if prev_step.from_coord() != step.to_coord() {
            if let Some(prev_kcell) = knowledge.get(prev_step.from_coord()) {
                if let Some(door_id) = prev_kcell.door {
                    if state.opened_doors.remove(&door_id) {
                        return Some(ActionType::CloseDoor(door_id));
                    }
                }
            }
        }
    }

    if let Some(dest_kcell) = knowledge.get(step.to_coord()) {
        if let Some(door_id) = dest_kcell.door {
            if let Some(&DoorState::Closed) = entity_store.door_state.get(&door_id) {
                state.opened_doors.insert(door_id);
                return Some(ActionType::OpenDoor(door_id));
            }
        }

        state.path_idx += 1;
        state.prev_step = Some(step);
        return Some(ActionType::Walk(id, step.direction()));
    }

    None
}
