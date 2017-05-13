use cgmath::Vector2;
use straight_line::*;

#[test]
fn finite_relative() {
    let mut traverse = FiniteAbsoluteLineTraverse::new_between(
        Vector2::new(0, 0),
        Vector2::new(2, 2),
    );

    assert_eq!(traverse.next(), Some(Vector2::new(0, 0)));
    assert_eq!(traverse.next(), Some(Vector2::new(1, 1)));
    assert_eq!(traverse.next(), Some(Vector2::new(2, 2)));
    assert_eq!(traverse.next(), None);
}
