use grid_search::SearchEnv;

pub struct BehaviourEnv {
    pub search_env: SearchEnv,
}

impl BehaviourEnv {
    pub fn new(width: usize, height: usize) -> Self {
        BehaviourEnv {
            search_env: SearchEnv::new(width, height),
        }
    }
}
