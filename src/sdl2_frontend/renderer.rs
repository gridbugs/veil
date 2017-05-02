use std::path::Path;
use sdl2::VideoSubsystem;
use sdl2::render::Renderer;
use sdl2_frontend::tile_buffer::TileBuffer;
use sdl2_frontend::textures::GameTextures;
use sdl2_frontend::tile::TileResolver;
use simple_file;

pub struct GameRenderer {
    buffer: TileBuffer,
    textures: GameTextures,
    internal: GameRendererInternal,
}

impl GameRenderer {
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(width: usize, height: usize, video: &VideoSubsystem,
                                               tile_path: P, tile_desc_path: Q) -> Self {

        let window = video.window("Veil", (width * 16) as u32, (height * 16) as u32).build().expect("Failed to create window");
        let renderer = window.renderer().build().expect("Failed to initialize renderer");

        let textures = GameTextures::new(&renderer, tile_path);
        let internal = GameRendererInternal::new(renderer, tile_desc_path);

        GameRenderer {
            buffer: TileBuffer::new(width, height),
            textures: textures,
            internal: internal,
        }
    }
}

struct GameRendererInternal {
    renderer: Renderer<'static>,
    tile_resolver: TileResolver,
}

impl GameRendererInternal {
    fn new<P: AsRef<Path>>(renderer: Renderer<'static>, tile_desc_path: P) -> Self {
        let tile_desc_str = simple_file::read_string(tile_desc_path)
            .expect("Failed to open tile description");
        let tile_resolver = TileResolver::from_str(&tile_desc_str);
        GameRendererInternal {
            renderer: renderer,
            tile_resolver: tile_resolver,
        }
    }
}
