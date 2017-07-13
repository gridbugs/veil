/// Convert a coordinate in -1..1 space with top-left (-1, 1)
/// into a coordinate in 0..1 space with top-left (0, 0)
pub fn cart_to_cg((x, y): (f32, f32)) -> (f32, f32) {
    (x / 2.0 + 0.5, 0.5 - y / 2.0)
}
