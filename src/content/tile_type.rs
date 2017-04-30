enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TileType {
    StoneFloor,
    WallFront,
    WallTop,
    Player,
    Rain,
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
        }
    }
}

pub const NUM_TILES: usize = 5;
