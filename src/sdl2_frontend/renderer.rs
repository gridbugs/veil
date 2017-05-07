use std::path::Path;
use sdl2_frontend::renderer_env::RendererEnv;
use sdl2_frontend::tile_buffer::TileBuffer;
use sdl2_frontend::textures::GameTextures;
use sdl2_frontend::renderer_internal::GameRendererInternal;

pub struct GameRenderer<'a> {
    buffer: TileBuffer,
    internal: GameRendererInternal<'a>,
    textures: GameTextures<'a>,
}

impl<'a> GameRenderer<'a> {
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(width: usize, height: usize, env: &'a mut RendererEnv,
                                               tile_path: P, tile_desc_path: Q) -> Self {

        let buffer = TileBuffer::new(width, height);
        let internal = GameRendererInternal::new(tile_desc_path, &mut env.canvas);
        let textures = GameTextures::new(tile_path, &env.texture_creator);

        GameRenderer {
            buffer: buffer,
            internal: internal,
            textures: textures,
        }
    }
}
