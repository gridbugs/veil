use cgmath::Vector2;
use grid::StaticGrid;
use direction::*;

#[test]
fn neighbour_coord_iter() {

    let grid = StaticGrid::new_copy(4, 4, ());

    let mut iter = grid.neighbour_coord_iter(Vector2::new(0, 0), CardinalDirections);
    assert_eq!(iter.next(), Some(Vector2::new(1, 0)));
    assert_eq!(iter.next(), Some(Vector2::new(0, 1)));
    assert_eq!(iter.next(), None);

    let mut iter = grid.neighbour_coord_iter(Vector2::new(1, 1), CardinalDirections);
    assert_eq!(iter.next(), Some(Vector2::new(1, 0)));
    assert_eq!(iter.next(), Some(Vector2::new(2, 1)));
    assert_eq!(iter.next(), Some(Vector2::new(1, 2)));
    assert_eq!(iter.next(), Some(Vector2::new(0, 1)));
    assert_eq!(iter.next(), None);

    let mut iter = grid.neighbour_coord_iter(Vector2::new(grid.width() as i32 - 1, grid.height() as i32 - 1), CardinalDirections);
    assert_eq!(iter.next(), Some(Vector2::new(grid.width() as i32 - 1, grid.height() as i32 - 2)));
    assert_eq!(iter.next(), Some(Vector2::new(grid.width() as i32 - 2, grid.height() as i32 - 1)));
    assert_eq!(iter.next(), None);
}
