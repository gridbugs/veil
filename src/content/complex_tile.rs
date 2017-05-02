use content::TileType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ComplexTile {
    Wall {
        front: TileType,
        top: TileType,
    },
    Simple(TileType),
}
