use std::path::Path;
use sdl2::render::WindowCanvas;
use sdl2_frontend::tile::TileResolver;
use sdl2_frontend::tile_buffer::TileBufferCell;
use sdl2_frontend::renderer_dimensions::RendererDimensions;
use sdl2_frontend::textures::GameTextures;
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

    pub fn draw_cell(&mut self, cell: &TileBufferCell, (x, y): (usize, usize),
                     dimensions: &RendererDimensions, textures: &GameTextures) {

        for channel in cell.channels.iter() {
            if let &Some(source) = channel {
                self.canvas.copy(&textures.colour, source, dimensions.dest_rect(x as u32, y as u32))
                    .expect("Failed to draw cell");
            }
        }
    }
}
