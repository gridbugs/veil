use std::cmp;
use cgmath::Vector2;

pub struct LimitsRect {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32,
}

impl LimitsRect {
    pub fn new(x_min: i32, y_min: i32, x_max: i32, y_max: i32) -> LimitsRect {
        assert!(x_min <= x_max);
        assert!(y_min <= y_max);

        LimitsRect {
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
        }
    }

    pub fn new_rect(x_min: i32, y_min: i32, width: u32, height: u32) -> LimitsRect {
        Self::new(x_min, y_min, x_min + width as i32, y_min + height as i32)
    }

    pub fn new_from_vector(v: Vector2<i32>, width: u32, height: u32) -> LimitsRect {
        Self::new_rect(v.x, v.y, width, height)
    }

    pub fn saturate(&self, mut v: Vector2<i32>) -> Vector2<i32> {
        v.x = cmp::max(self.x_min, cmp::min(self.x_max, v.x));
        v.y = cmp::max(self.y_min, cmp::min(self.y_max, v.y));
        v
    }
}
