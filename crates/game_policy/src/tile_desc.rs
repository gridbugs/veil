use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TileDesc {
    tile_size: u32,
    pub tile_scale: u32,
    pub overlays: HashMap<String, [u32; 2]>,
    pub tiles: HashMap<String, HashMap<String, [u32; 2]>>,
}

impl TileDesc {
    pub fn tile_size_original(&self) -> u32 { self.tile_size }
    pub fn tile_size_scaled(&self) -> u32 { self.tile_scale * self.tile_size }
}
