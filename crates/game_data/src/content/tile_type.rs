enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TileType {
    StoneFloor,
    StoneFloorFront,
    WallFront,
    WallTop,
    Player,
    Undead,
    SuperUndead,
    Rain,
    Splash,
    OpenDoorFront,
    ClosedDoorFront,
    OpenDoorTop,
    ClosedDoorTop,
    Bullet,
    Page,
    Water1,
    Water2,
    WoodenFloor,
    WoodenPost,
    StoneWallFront,
    StoneWallTop,
}
}

impl TileType {
    pub fn to_str(self) -> &'static str {
        match self {
            TileType::StoneFloor => "StoneFloor",
            TileType::StoneFloorFront => "StoneFloorFront",
            TileType::WallFront => "WallFront",
            TileType::WallTop => "WallTop",
            TileType::Player => "Player",
            TileType::Undead => "Undead",
            TileType::SuperUndead => "SuperUndead",
            TileType::Rain => "Rain",
            TileType::Splash => "Splash",
            TileType::OpenDoorFront => "OpenDoorFront",
            TileType::ClosedDoorFront => "ClosedDoorFront",
            TileType::OpenDoorTop => "OpenDoorTop",
            TileType::ClosedDoorTop => "ClosedDoorTop",
            TileType::Bullet => "Bullet",
            TileType::Page => "Page",
            TileType::Water1 => "Water1",
            TileType::Water2 => "Water2",
            TileType::WoodenFloor => "WoodenFloor",
            TileType::WoodenPost => "WoodenPost",
            TileType::StoneWallFront => "StoneWallFront",
            TileType::StoneWallTop => "StoneWallTop",
        }
    }
}

pub const NUM_TILES: usize = 21;
