enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum OverlayType {
    Blank,
    Death,
    AimLineMid,
    AimLineEnd,
    Veil,
    VeilCurrent,
    VeilNext,
}
}

impl OverlayType {
    pub fn to_str(self) -> &'static str {
        match self {
            OverlayType::Blank => "Blank",
            OverlayType::Death => "Death",
            OverlayType::AimLineMid => "AimLineMid",
            OverlayType::AimLineEnd => "AimLineEnd",
            OverlayType::Veil => "Veil",
            OverlayType::VeilCurrent => "VeilCurrent",
            OverlayType::VeilNext => "VeilNext",
        }
    }
}

pub const NUM_OVERLAYS: usize = 7;
