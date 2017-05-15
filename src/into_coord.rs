use cgmath::Vector2;

pub trait IntoCoord {
    fn into_coord(self) -> Vector2<i32>;
}
