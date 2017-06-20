use std::collections::HashMap;
use toml;
use enum_primitive::FromPrimitive;

use tile;
use rect::Rect;
use content::*;

pub const NUM_TILE_CHANNELS: usize = 5;
pub const OVERLAY_CHANNEL: usize = 4;

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: usize,
    pub sprite: Rect,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub channels: Vec<Channel>,
}

impl Tile {
    fn new() -> Self {
        Tile {
            channels: Vec::new(),
        }
    }
}


#[derive(Deserialize)]
struct TileDesc {
    tile_size: u32,
    overlays: HashMap<String, [u32; 2]>,
    tiles: HashMap<String, HashMap<String, [u32; 2]>>,
}

impl TileDesc {
    fn rect(&self, x: u32, y: u32) -> Rect {
        Rect::new((x * self.tile_size) as i32, (y * self.tile_size) as i32, self.tile_size, self.tile_size)
    }
}

#[derive(Debug)]
pub struct TileResolver {
    tiles: Vec<Tile>,
    overlays: Vec<Rect>,
    tile_size: u32,
}

impl TileResolver {
    fn new(tile_size: u32) -> Self {
        TileResolver {
            tiles: Vec::new(),
            overlays: Vec::new(),
            tile_size: tile_size,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let tile_desc: TileDesc = toml::from_str(s).expect("Failed to parse tile description");
        let mut resolver = TileResolver::new(tile_desc.tile_size);

        for i in 0..NUM_OVERLAYS {
            if let Some(overlay_type) = OverlayType::from_usize(i) {
                let coord = tile_desc.overlays.get(&overlay_type.to_str().to_string())
                    .expect(&format!("Couldn't find overlay for {:?}", overlay_type));
                resolver.overlays.push(tile_desc.rect(coord[0], coord[1]));
            }
        }

        for i in 0..NUM_TILES {
            if let Some(tile_type) = TileType::from_usize(i) {
                let channels = tile_desc.tiles.get(&tile_type.to_str().to_string())
                    .expect(&format!("Couldn't find tile for {:?}", tile_type));
                let mut tile = Tile::new();
                for j in 0..tile::NUM_TILE_CHANNELS {
                    if let Some(coord) = channels.get(&format!("{}", j)) {
                        tile.channels.push(Channel {
                            id: j,
                            sprite: tile_desc.rect(coord[0], coord[1]),
                        });
                    }
                }
                resolver.tiles.push(tile);
            }
        }

        resolver
    }

    pub fn resolve_tile(&self, tile_type: TileType) -> &Tile {
        &self.tiles[tile_type as usize]
    }

    pub fn resolve_overlay(&self, overlay_type: OverlayType) -> &Rect {
        &self.overlays[overlay_type as usize]
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }
}
