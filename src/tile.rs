use toml;
use enum_primitive::FromPrimitive;
use cgmath::Vector2;
use image::{self, RgbaImage};

use tile_desc::TileDesc;
use content::*;
use resources::{self, TILE_SHEET_SPEC, TILE_SHEET_IMAGE};
use simple_file;

pub const NUM_TILE_CHANNELS: usize = 5;
pub const OVERLAY_CHANNEL: usize = 4;
pub type TileCoord = Vector2<i8>;

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: usize,
    pub sprite: TileCoord,
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

#[derive(Debug)]
pub struct TileResolver {
    tiles: Vec<Tile>,
    overlays: Vec<TileCoord>,
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
        Self::from_desc(&toml::from_str(s).expect("Failed to parse tile description"))
    }

    pub fn from_desc(tile_desc: &TileDesc) -> Self {
        let mut resolver = TileResolver::new(tile_desc.tile_size);

        for i in 0..NUM_OVERLAYS {
            if let Some(overlay_type) = OverlayType::from_usize(i) {
                let coord = tile_desc.overlays.get(&overlay_type.to_str().to_string())
                    .expect(&format!("Couldn't find overlay for {:?}", overlay_type));
                resolver.overlays.push(Vector2::new(coord[0] as i8, coord[1] as i8));
            }
        }

        for i in 0..NUM_TILES {
            if let Some(tile_type) = TileType::from_usize(i) {
                let channels = tile_desc.tiles.get(&tile_type.to_str().to_string())
                    .expect(&format!("Couldn't find tile for {:?}", tile_type));
                let mut tile = Tile::new();
                for j in 0..NUM_TILE_CHANNELS {
                    if let Some(coord) = channels.get(&format!("{}", j)) {
                        tile.channels.push(Channel {
                            id: j,
                            sprite: Vector2::new(coord[0] as i8, coord[1] as i8),
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

    pub fn resolve_overlay(&self, overlay_type: OverlayType) -> TileCoord {
        self.overlays[overlay_type as usize]
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }
}

pub fn read_tiles() -> (RgbaImage, TileDesc) {
    let tile_path = resources::res_path(TILE_SHEET_IMAGE);
    let img = image::open(tile_path).expect("failed to open image").to_rgba();

    let tile_desc: TileDesc = simple_file::read_toml(&resources::res_path(TILE_SHEET_SPEC))
        .expect("Failed to read tile spec");

    (img, tile_desc)
}
