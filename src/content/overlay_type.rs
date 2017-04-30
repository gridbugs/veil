enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum OverlayType {
    Blank,
    Death,
    PostEldrich,
    Eldrich,
    PreEldrich,
}
}

impl OverlayType {
    pub fn to_str(self) -> &'static str {
        match self {
            OverlayType::Blank => "Blank",
            OverlayType::Death => "Death",
            OverlayType::PostEldrich => "PostEldrich",
            OverlayType::Eldrich => "Eldrich",
            OverlayType::PreEldrich => "PreEldrich",
        }
    }
}

pub const NUM_OVERLAYS: usize = 5;
