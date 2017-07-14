use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TileDesc {
    pub tile_size: u32,
    pub overlays: HashMap<String, [u32; 2]>,
    pub tiles: HashMap<String, HashMap<String, [u32; 2]>>,
}
