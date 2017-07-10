use grid::StaticGridIdx;
use cgmath::Vector2;

impl StaticGridIdx for Vector2<i32> {
    fn wrap_to_index(self, width: usize) -> usize {
        (self.y as usize) * width + (self.x as usize)
    }
    fn is_valid(self, width: usize) -> bool {
        self.x >= 0 && self.y >= 0 && (self.x as usize) < width
    }
}

pub trait IntoCoord {
    fn into_coord(self) -> Vector2<i32>;
}

pub trait LookupCoord {
    type Item;
    fn lookup_coord(&self, coord: Vector2<i32>) -> Option<&Self::Item>;
}
