use content::TileType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplexTile {
    Wall {
        front: TileType,
        top: TileType,
    },
    Simple(TileType),
}

impl ComplexTile {
    pub fn is_wall(&self) -> bool {
        match self {
            &ComplexTile::Wall { .. } => true,
            _ => false,
        }
    }
}
