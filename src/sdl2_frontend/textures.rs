use std::path::Path;
use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;

pub struct GameTextures {
    pub colour: Texture,
}

impl GameTextures {
    pub fn new<P: AsRef<Path>>(renderer: &Renderer, tile_path: P) -> Self {
        let tile_texture = renderer.load_texture(&tile_path).expect("Failed to load texture");
        GameTextures {
            colour: tile_texture,
        }
    }
}
