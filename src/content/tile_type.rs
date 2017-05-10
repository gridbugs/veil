enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TileType {
    StoneFloor,
    WallFront,
    WallTop,
    Player,
    Rain,
    OpenDoorFront,
    ClosedDoorFront,
    OpenDoorTop,
    ClosedDoorTop,
}
}

impl TileType {
    pub fn to_str(self) -> &'static str {
        match self {
            TileType::StoneFloor => "StoneFloor",
            TileType::WallFront => "WallFront",
            TileType::WallTop => "WallTop",
            TileType::Player => "Player",
            TileType::Rain => "Rain",
            TileType::OpenDoorFront => "OpenDoorFront",
            TileType::ClosedDoorFront => "ClosedDoorFront",
            TileType::OpenDoorTop => "OpenDoorTop",
            TileType::ClosedDoorTop => "ClosedDoorTop",
        }
    }
}

pub const NUM_TILES: usize = 9;
