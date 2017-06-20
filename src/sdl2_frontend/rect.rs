use rect;
use sdl2;

impl From<rect::Rect> for sdl2::rect::Rect {
    fn from(r: rect::Rect) -> Self {
        sdl2::rect::Rect::new(r.x, r.y, r.width, r.height)
    }
}

impl From<rect::Rect> for Option<sdl2::rect::Rect> {
    fn from(r: rect::Rect) -> Self {
        Some(sdl2::rect::Rect::from(r))
    }
}
