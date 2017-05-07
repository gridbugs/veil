use sdl2::rect::Rect;

#[derive(Debug)]
pub struct RendererDimensions {
    pub dest_tile_size: u32,
    pub game_area: Rect,
}

impl RendererDimensions {
    pub fn new(window_width_px: u32, window_height_px: u32,
               window_width_tiles: u32, window_height_tiles: u32) -> Self {

        let mut game_area_height = window_height_px;
        let mut game_area_width =
            (window_width_tiles * game_area_height) / window_height_tiles;

        if game_area_width > window_width_px {
            game_area_width = window_width_px;
            game_area_height = (window_height_tiles * game_area_width) / window_width_tiles;
        }

        let dest_tile_size = game_area_width / window_width_tiles;

        let game_area = Rect::new(
            ((window_width_px - game_area_width) / 2) as i32,
            ((window_height_px - game_area_height) / 2) as i32,
            game_area_width,
            game_area_height,
        );

        RendererDimensions {
            dest_tile_size: dest_tile_size,
            game_area: game_area,
        }
    }

    pub fn dest_rect(&self, x: u32, y: u32) -> Rect {
        Rect::new(
            self.game_area.x + (x * self.dest_tile_size) as i32,
            self.game_area.y + (y * self.dest_tile_size) as i32,
            self.dest_tile_size,
            self.dest_tile_size
        )
    }
}
