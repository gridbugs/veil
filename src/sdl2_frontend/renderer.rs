use std::path::Path;
use sdl2_frontend::renderer_env::RendererEnv;
use sdl2_frontend::tile_buffer::TileBuffer;
use sdl2_frontend::textures::GameTextures;
use sdl2_frontend::renderer_dimensions::RendererDimensions;
use sdl2_frontend::renderer_internal::GameRendererInternal;
use knowledge::PlayerKnowledgeGrid;

pub struct GameRenderer<'a> {
    buffer: TileBuffer,
    internal: GameRendererInternal<'a>,
    textures: GameTextures<'a>,
    dimensions: RendererDimensions,
}

impl<'a> GameRenderer<'a> {
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(width: usize, height: usize, env: &'a mut RendererEnv,
                                               tile_path: P, tile_desc_path: Q) -> Self {

        let (window_width, window_height) = env.canvas.window().size();

        let buffer = TileBuffer::new(width, height);
        let internal = GameRendererInternal::new(tile_desc_path, &mut env.canvas);
        let textures = GameTextures::new(tile_path, &env.texture_creator);
        let dimensions = RendererDimensions::new(
            window_width,
            window_height,
            width as u32,
            height as u32);

        GameRenderer {
            buffer: buffer,
            internal: internal,
            textures: textures,
            dimensions: dimensions,
        }
    }

    pub fn update(&mut self, knowledge: &PlayerKnowledgeGrid, time: u64) {
        self.buffer.update(knowledge, &self.internal.tile_resolver, time);
    }

    pub fn draw(&mut self) {
        for (cell, coord) in izip!(self.buffer.iter(), self.buffer.coord_iter()) {
            self.internal.draw_cell(cell, coord, &self.dimensions, &self.textures);
        }
    }

    pub fn publish(&mut self) {
        self.internal.canvas.present();
    }
}
