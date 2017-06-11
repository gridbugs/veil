use std::path::Path;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use cgmath::Vector2;
use sdl2_frontend::tile::{TileResolver, OVERLAY_CHANNEL};
use sdl2_frontend::tile_buffer::TileBufferCell;
use sdl2_frontend::renderer_dimensions::RendererDimensions;
use sdl2_frontend::textures::GameTextures;
use simple_file;
use render_overlay::RenderOverlay;
use content::OverlayType;

const DIM_COEF: i32 = 32;
const INTENSITY_MAX: u8 = 255;
const INTENSITY_MIN: u8 = 96;
const INTENSITY_DIFF: u8 = INTENSITY_MAX - INTENSITY_MIN;

const INTENSITY_NUMERATOR: i32 = INTENSITY_DIFF as i32 * DIM_COEF;

pub struct GameRendererInternal<'a> {
    pub tile_resolver: TileResolver,
    pub canvas: &'a mut WindowCanvas,
}

fn delta_to_intensity(delta: Vector2<i32>) -> u8 {
    let length_squared = delta.x * delta.x + delta.y * delta.y;
    let intensity_delta = INTENSITY_NUMERATOR / (length_squared + DIM_COEF);
    INTENSITY_MIN + intensity_delta as u8
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

    pub fn draw_cell(&mut self, cell: &TileBufferCell, centre: Vector2<i32>, coord: Vector2<i32>,
                     dimensions: &RendererDimensions, textures: &mut GameTextures) {

        let texture = if cell.visible {
            let intensity = delta_to_intensity(coord - centre);
            textures.colour.set_color_mod(intensity, intensity, intensity);
            &mut textures.colour
        } else {
            textures.greyscale.set_color_mod(INTENSITY_MIN, INTENSITY_MIN, INTENSITY_MIN);
            &mut textures.greyscale
        };

        let dest_rect = dimensions.dest_rect(coord.x as u32, coord.y as u32);

        for (idx, channel) in cell.channels.iter().enumerate() {
            if idx == OVERLAY_CHANNEL {
                // overlay channel is always the last channel
                break;
            }
            if let &Some(source) = channel {
                self.canvas.copy(texture, source, dest_rect).expect("Failed to draw cell");
            }
        }

        if let Some(source) = cell.channels[OVERLAY_CHANNEL] {
            texture.set_color_mod(INTENSITY_MAX, INTENSITY_MAX, INTENSITY_MAX);
            self.canvas.copy(texture, source, dest_rect).expect("Failed to draw cell");
        }
    }

    pub fn draw_overlay(&mut self, dimensions: &RendererDimensions,
                        textures: &mut GameTextures, overlay: RenderOverlay) {

        let tile_mid = Some(*self.tile_resolver.resolve_overlay(OverlayType::AimLineMid));
        let tile_end = Some(*self.tile_resolver.resolve_overlay(OverlayType::AimLineEnd));

        let (mut traverse, end) = overlay.aim_line.split_end();

        textures.colour.set_color_mod(INTENSITY_MAX, INTENSITY_MAX, INTENSITY_MAX);

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
