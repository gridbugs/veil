use limits::LimitsRect;
use cgmath::Vector2;

#[test]
fn saturate() {
    let l = LimitsRect::new_rect(0, 0, 2, 2);
    let v = Vector2::new(-1, 3);
    let s = l.saturate(v);

    assert_eq!(s, Vector2::new(0, 2));
}
