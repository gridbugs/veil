use cgmath::Vector3;
use content::ActionType;

pub enum External {
    Quit,
}

pub enum DebugAction {
    ChangeVeilMin(f64),
    ChangeVeilMax(f64),
    ChangeVeilStep(Vector3<f64>),
    TogglePlayerOmniscient,
    ToggleDiminishingLighting,
    Wait,
}

pub enum MetaAction {
    Action(ActionType),
    External(External),
    Debug(DebugAction),
}
