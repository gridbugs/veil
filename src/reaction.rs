use content::ActionType;

pub struct Reaction {
    pub action: ActionType,
    pub delay: u64,
}

impl Reaction {
    pub fn new(action: ActionType, delay: u64) -> Self {
        Reaction {
            action: action,
            delay: delay,
        }
    }

    pub fn immediate(action: ActionType) -> Self {
        Self::new(action, 0)
    }
}
