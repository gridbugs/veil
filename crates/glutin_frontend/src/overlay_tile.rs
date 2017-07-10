use game_policy::tile::TileCoord;

use tile_map::{TileMapData, UpdateTileMapData, ShaderTemplateInfo};

const TILE_STATUS_IDX: usize = 3;
const TILE_STATUS_ENABLED: u32 = 1 << 0;

pub struct OverlayCoord(pub TileCoord);

pub fn shader_template_info() -> ShaderTemplateInfo<'static> {
    btreemap!{
        "TILE_STATUS_IDX" => TILE_STATUS_IDX as u32,
        "TILE_STATUS_ENABLED" => TILE_STATUS_ENABLED,
    }
}

impl UpdateTileMapData for OverlayCoord {
    fn update(&self, idx: usize, data: &mut [TileMapData]) {
        let cell = &mut data[idx];
        cell.data[TILE_STATUS_IDX] = f32::from_bits(TILE_STATUS_ENABLED);
        cell.data[0] = f32::from_bits(((self.0.x as u32) | ((self.0.y as u32) << 8)) as u32);
    }
}

pub fn clear_tile_map_data(data: &mut [TileMapData]) {
    for cell in data.iter_mut() {
        cell.data[TILE_STATUS_IDX] = f32::from_bits(0);
    }
}
