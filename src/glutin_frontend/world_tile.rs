use glutin_frontend::tile_map::{TileMapData, UpdateTileMapData, ShaderTemplateInfo};
use game_policy::tile_buffer::TileBufferCell;
use game_policy::tile::{NUM_TILE_CHANNELS, OVERLAY_CHANNEL};

const TILE_STATUS_IDX: usize = 3;
const STATUS_BITS_PER_CHANNEL: usize = 2;
const CHANNEL_STATUS_BITS: usize = NUM_TILE_CHANNELS * STATUS_BITS_PER_CHANNEL;
const TILE_STATUS_VISIBLE: u32 = 1 << (CHANNEL_STATUS_BITS + 0);
const CHANNEL_PRESENT_OFFSET: usize = 0;
const CHANNEL_DIMINISH_OFFSET: usize = 1;

pub fn shader_template_info() -> ShaderTemplateInfo<'static> {
    btreemap!{
        "NUM_TILE_CHANNELS" => NUM_TILE_CHANNELS as u32,
        "TILE_STATUS_IDX" => TILE_STATUS_IDX as u32,
        "TILE_STATUS_VISIBLE" => TILE_STATUS_VISIBLE,
        "STATUS_BITS_PER_CHANNEL" => STATUS_BITS_PER_CHANNEL as u32,
        "CHANNEL_PRESENT_OFFSET" => CHANNEL_PRESENT_OFFSET as u32,
        "CHANNEL_DIMINISH_OFFSET" => CHANNEL_DIMINISH_OFFSET as u32,
    }
}

fn channel_mask(channel_idx: usize) -> u32 {
    if channel_idx % 2 == 0 {
        0xffff0000
    } else {
        0x0000ffff
    }
}

fn channel_shift(channel_idx: usize) -> u32 {
    (channel_idx as u32 % 2) * 16
}

fn set_visible(cell: &mut TileMapData) {
    let mut current = cell.data[TILE_STATUS_IDX].to_bits();
    current |= TILE_STATUS_VISIBLE;
    cell.data[TILE_STATUS_IDX] = f32::from_bits(current);
}

fn clear_visible(cell: &mut TileMapData) {
    let mut current = cell.data[TILE_STATUS_IDX].to_bits();
    current &= !TILE_STATUS_VISIBLE;
    cell.data[TILE_STATUS_IDX] = f32::from_bits(current);
}

fn set_channel_present(cell: &mut TileMapData, channel_idx: usize) {
    let mut current = cell.data[TILE_STATUS_IDX].to_bits();
    current |= 1 << ((channel_idx * STATUS_BITS_PER_CHANNEL + CHANNEL_PRESENT_OFFSET) as u32);
    cell.data[TILE_STATUS_IDX] = f32::from_bits(current);
}

fn clear_channel_present(cell: &mut TileMapData, channel_idx: usize) {
    let mut current = cell.data[TILE_STATUS_IDX].to_bits();
    current &= !(1 << ((channel_idx * STATUS_BITS_PER_CHANNEL + CHANNEL_PRESENT_OFFSET) as u32));
    cell.data[TILE_STATUS_IDX] = f32::from_bits(current);
}

fn set_channel_diminish(cell: &mut TileMapData, channel_idx: usize) {
    let mut current = cell.data[TILE_STATUS_IDX].to_bits();
    current |= 1 << ((channel_idx * STATUS_BITS_PER_CHANNEL + CHANNEL_DIMINISH_OFFSET) as u32);
    cell.data[TILE_STATUS_IDX] = f32::from_bits(current);
}

fn clear_channel_diminish(cell: &mut TileMapData, channel_idx: usize) {
    let mut current = cell.data[TILE_STATUS_IDX].to_bits();
    current &= !(1 << ((channel_idx * STATUS_BITS_PER_CHANNEL + CHANNEL_DIMINISH_OFFSET) as u32));
    cell.data[TILE_STATUS_IDX] = f32::from_bits(current);
}

fn set_channel(cell: &mut TileMapData, channel_idx: usize, x: u8, y: u8) {
    let idx = channel_idx / 2;
    let current = cell.data[idx].to_bits();
    let masked = current & channel_mask(channel_idx);
    let result = masked | ((x as u32 | (y as u32) << 8) << channel_shift(channel_idx));
    cell.data[idx] = f32::from_bits(result);
}

pub fn init_tile_map_data(data: &mut [TileMapData]) {
    for cell in data.iter_mut() {
        for i in 0..NUM_TILE_CHANNELS {
            if i != OVERLAY_CHANNEL {
                set_channel_diminish(cell, i);
            }
        }
    }
}

impl<'a> UpdateTileMapData for &'a TileBufferCell {
    fn update(&self, idx: usize, data: &mut [TileMapData]) {
        let cell = &mut data[idx];
        for channel_idx in 0..NUM_TILE_CHANNELS {
            if let Some(coord) = self.channels[channel_idx] {
                set_channel(cell, channel_idx, coord.x as u8, coord.y as u8);
                set_channel_present(cell, channel_idx);
            } else {
                clear_channel_present(cell, channel_idx);
            }
            if self.visible {
                set_visible(cell);
            } else {
                clear_visible(cell);
            }
        }
    }
}
