use std::path::Path;
use cgmath::Vector2;
use sdl2_frontend::renderer_env::RendererEnv;
use sdl2_frontend::tile_buffer::TileBuffer;
use sdl2_frontend::textures::GameTextures;
use sdl2_frontend::renderer_dimensions::RendererDimensions;
use sdl2_frontend::renderer_internal::GameRendererInternal;
use knowledge::PlayerKnowledgeGrid;
use render_overlay::RenderOverlay;
use renderer::GameRenderer;

pub struct SdlGameRenderer<'a> {
    buffer: TileBuffer,
    internal: GameRendererInternal<'a>,
    textures: GameTextures<'a>,
    dimensions: RendererDimensions,
    player_coord: Vector2<i32>,
}

impl<'a> SdlGameRenderer<'a> {
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

        SdlGameRenderer {
            buffer: buffer,
            internal: internal,
            textures: textures,
            dimensions: dimensions,
            player_coord: Vector2::new(0, 0),
        }
    }
}

impl<'a> GameRenderer for SdlGameRenderer<'a> {
    fn clear(&mut self) {
        self.internal.clear();
    }

    fn update_player_position(&mut self, player_coord: Vector2<i32>) {
        self.player_coord = player_coord;
    }

    fn update(&mut self, knowledge: &PlayerKnowledgeGrid, time: u64) {
        self.buffer.update(knowledge, &self.internal.tile_resolver, time);
    }

    fn draw(&mut self) {
        for (cell, coord) in izip!(self.buffer.iter(), self.buffer.coord_iter()) {
            self.internal.draw_cell(cell, self.player_coord, coord, &self.dimensions, &mut self.textures);
        }
    }

    fn draw_overlay(&mut self, overlay: RenderOverlay) {
        self.internal.draw_overlay(&self.dimensions, &mut self.textures, overlay);
    }

    fn publish(&mut self) {
        self.internal.canvas.present();
    }
}
