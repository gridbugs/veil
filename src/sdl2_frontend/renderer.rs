use std::path::Path;
use sdl2::VideoSubsystem;
use sdl2_frontend::tile_buffer::TileBuffer;
use sdl2_frontend::textures::GameTextures;
use sdl2_frontend::renderer_internal::GameRendererInternal;

pub struct GameRenderer {
    buffer: TileBuffer,
    textures: GameTextures,
    internal: GameRendererInternal,
}

impl GameRenderer {
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(width: usize, height: usize, video: &VideoSubsystem,
                                               tile_path: P, tile_desc_path: Q) -> Self {

        video.window("Veil", (width * 16) as u32, (height * 16) as u32).build().expect("Failed to create window");

        let textures = GameTextures::new(tile_path);
        let internal = GameRendererInternal::new(tile_desc_path);

        GameRenderer {
            buffer: TileBuffer::new(width, height),
            textures: textures,
            internal: internal,
        }
    }
}
