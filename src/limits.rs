use std::cmp;
use cgmath::Vector2;

pub trait LimitsRect {
    fn x_min(&self) -> i32;
    fn x_max(&self) -> i32;
    fn y_min(&self) -> i32;
    fn y_max(&self) -> i32;

    fn saturate(&self, mut v: Vector2<i32>) -> Vector2<i32> {
        v.x = cmp::max(self.x_min(), cmp::min(self.x_max(), v.x));
        v.y = cmp::max(self.y_min(), cmp::min(self.y_max(), v.y));
        v
    }
}
