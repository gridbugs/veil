use std::path::Path;

pub struct GameTextures {}

impl GameTextures {
    pub fn new<P: AsRef<Path>>(_tile_path: P) -> Self {
        GameTextures {}
    }
}
