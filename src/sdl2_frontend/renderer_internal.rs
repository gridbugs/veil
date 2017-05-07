use std::path::Path;
use sdl2_frontend::tile::TileResolver;
use simple_file;

pub struct GameRendererInternal {
    pub tile_resolver: TileResolver,
}

impl GameRendererInternal {
    pub fn new<P: AsRef<Path>>(tile_desc_path: P) -> Self {
        let tile_desc_str = simple_file::read_string(tile_desc_path)
            .expect("Failed to open tile description");
        let tile_resolver = TileResolver::from_str(&tile_desc_str);
        GameRendererInternal {
            tile_resolver: tile_resolver,
        }
    }
}
