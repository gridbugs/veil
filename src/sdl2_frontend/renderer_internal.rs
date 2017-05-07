use std::path::Path;
use sdl2::render::WindowCanvas;
use sdl2_frontend::tile::TileResolver;
use simple_file;

pub struct GameRendererInternal<'a> {
    pub tile_resolver: TileResolver,
    pub canvas: &'a mut WindowCanvas,
}

impl<'a> GameRendererInternal<'a> {
    pub fn new<P: AsRef<Path>>(tile_desc_path: P, canvas: &'a mut WindowCanvas) -> Self {

        let tile_desc_str = simple_file::read_string(tile_desc_path)
            .expect("Failed to open tile description");
        let tile_resolver = TileResolver::from_str(&tile_desc_str);

        GameRendererInternal {
            tile_resolver: tile_resolver,
            canvas: canvas,
        }
    }
}
