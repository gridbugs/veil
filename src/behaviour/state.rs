use std::collections::HashSet;
use game_data::entity_store::EntityId;
use geometry::grid_search::{Path, Step, PathIterFrom};

pub struct BehaviourState {
    pub opened_doors: HashSet<EntityId>,
    pub prev_step: Option<Step>,
    pub path: Path,
    pub path_idx: usize,
}

impl BehaviourState {
    pub fn new() -> Self {
        BehaviourState {
            opened_doors: HashSet::new(),
            prev_step: None,
            path: Path::new(),
            path_idx: 0,
        }
    }

    pub fn current_step(&self) -> Option<Step> {
        self.path.get(self.path_idx)
    }

    pub fn path_iter(&self) -> PathIterFrom {
        self.path.iter_from(self.path_idx)
    }
}
