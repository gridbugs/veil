use rand::StdRng;
use entity_store::EntityStoreChange;
use entity_id_allocator::EntityIdAllocator;
use schedule::{Schedule, ScheduleEntry};
use content::ActionType;
use policy::GamePolicy;
use observation::shadowcast::ShadowcastEnv;
use reaction::Reaction;

pub struct GameEnv {
    pub id_allocator: EntityIdAllocator,
    pub change: EntityStoreChange,
    pub rng: StdRng,
    pub action_schedule: Schedule<ActionType>,
    pub policy: GamePolicy,
    pub shadowcast: ShadowcastEnv,
    pub reactions: Vec<Reaction>,
    pub action_schedule_entries: Vec<ScheduleEntry<ActionType>>,
    pub time: u64,
}

impl GameEnv {
    pub fn new() -> Self {
        GameEnv {
            id_allocator: EntityIdAllocator::new(),
            change: EntityStoreChange::new(),
            rng: StdRng::new().expect("Failed to init rng"),
            action_schedule: Schedule::new(),
            policy: GamePolicy::new(),
            shadowcast: ShadowcastEnv::new(),
            reactions: Vec::new(),
            action_schedule_entries: Vec::new(),
            time: 1,
        }
    }
}
