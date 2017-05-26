use std::path::Path;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2_frontend::tile::TileResolver;
use sdl2_frontend::tile_buffer::TileBufferCell;
use sdl2_frontend::renderer_dimensions::RendererDimensions;
use sdl2_frontend::textures::GameTextures;
use simple_file;
use render_overlay::RenderOverlay;
use content::OverlayType;

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

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn draw_cell(&mut self, cell: &TileBufferCell, (x, y): (usize, usize),
                     dimensions: &RendererDimensions, textures: &GameTextures) {

        let texture = if cell.visible {
            &textures.colour
        } else {
            &textures.greyscale
        };

        let dest_rect = dimensions.dest_rect(x as u32, y as u32);

        for channel in cell.channels.iter() {
            if let &Some(source) = channel {
                self.canvas.copy(texture, source, dest_rect).expect("Failed to draw cell");
            }
        }
    }

    pub fn draw_overlay(&mut self, dimensions: &RendererDimensions,
                        textures: &GameTextures, overlay: RenderOverlay) {

        let tile_mid = Some(*self.tile_resolver.resolve_overlay(OverlayType::AimLineMid));
        let tile_end = Some(*self.tile_resolver.resolve_overlay(OverlayType::AimLineEnd));

        let (mut traverse, end) = overlay.aim_line.split_end();

        let dest_rect = dimensions.dest_rect(end.x as u32, end.y as u32);
        self.canvas.copy(&textures.colour, tile_end, Some(dest_rect)).expect("Failed to draw cell");

        // skip the start
        traverse.step_in_place();

        for coord in traverse {
            let dest_rect = dimensions.dest_rect(coord.x as u32, coord.y as u32);
            self.canvas.copy(&textures.colour, tile_mid, Some(dest_rect)).expect("Failed to draw cell");
        }
    }
}
