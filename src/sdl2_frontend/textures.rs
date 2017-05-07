use std::path::Path;
use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;

pub struct GameTextures<'a> {
    pub colour: Texture<'a>,
}

impl<'a> GameTextures<'a> {
    pub fn new<P: AsRef<Path>>(tile_path: P, texture_creator: &'a TextureCreator<WindowContext>) -> Self {

        let texture = texture_creator.load_texture(tile_path)
            .expect("Failed to create texture");

        GameTextures {
            colour: texture,
        }
    }
}
