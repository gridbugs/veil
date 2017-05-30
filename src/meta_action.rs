use content::ActionType;

pub enum External {
    Quit,
}

pub enum MetaAction {
    Action(ActionType),
    External(External),
}
