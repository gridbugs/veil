use std::mem;
use std::slice;
use rand::Rng;
use grid::{StaticGrid, static_grid};
use perlin::{PerlinGrid, PerlinWrapType};
use cgmath::Vector2;

const ZOOM: usize = 10;
const ZOOM_F: f64 = ZOOM as f64;

pub type CoordIter = static_grid::CoordIter;
pub struct Iter<'a> {
    current: static_grid::Iter<'a, bool>,
    next: static_grid::Iter<'a, bool>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = VeilCell;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.next() {
            if let Some(next) = self.next.next() {
                return Some(VeilCell {
                    current: *current,
                    next: *next,
                })
            }
        }

        None
    }
}

pub struct RowIter<'a> {
    current: static_grid::Rows<'a, bool>,
    next: static_grid::Rows<'a, bool>,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Row<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.next() {
            if let Some(next) = self.next.next() {
                return Some(Row {
                    current: current.iter(),
                    next: next.iter(),
                });
            }
        }

        None
    }
}

pub struct Row<'a> {
    current: slice::Iter<'a, bool>,
    next: slice::Iter<'a, bool>,
}

impl<'a> Iterator for Row<'a> {
    type Item = VeilCell;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.next() {
            if let Some(next) = self.next.next() {
                return Some(VeilCell {
                    current: *current,
                    next: *next,
                });
            }
        }

        None
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VeilCell {
    pub current: bool,
    pub next: bool,
}

pub struct VeilState {
    current: StaticGrid<bool>,
    next: StaticGrid<bool>,
    perlin: PerlinGrid,
    dx: f64,
    dy: f64,
}

impl VeilState {
    pub fn new<R: Rng>(width: usize, height: usize, rng: &mut R) -> Self {

        let mut perlin = PerlinGrid::new((width - 1) / ZOOM + 1,
                                         (height - 1) / ZOOM + 1,
                                         PerlinWrapType::Regenerate,
                                         rng);

        let mut current = StaticGrid::new_default(width, height);
        let mut next = StaticGrid::new_default(width, height);

        let dx = 1.0 / ZOOM_F;
        let dy = 1.0 / ZOOM_F;

        record(dx, dy, &perlin, &mut current);
        mutate(&mut perlin, rng);
        record(dx, dy, &perlin, &mut next);

        VeilState {
            current: current,
            next: next,
            perlin: perlin,
            dx: dx,
            dy: dy,
        }
    }

    pub fn get(&self, coord: Vector2<i32>) -> Option<VeilCell> {
        self.current.get(coord).and_then(|current| {
            self.next.get(coord).map(|next| {
                VeilCell {
                    current: *current,
                    next: *next,
                }
            })
        })
    }

    pub fn rows(&self) -> RowIter {
        RowIter {
            current: self.current.rows(),
            next: self.next.rows(),
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            current: self.current.iter(),
            next: self.next.iter(),
        }
    }

    pub fn coord_iter(&self) -> CoordIter {
        self.current.coord_iter()
    }

    pub fn step<R: Rng>(&mut self, rng: &mut R) {
        mem::swap(&mut self.current, &mut self.next);
        mutate(&mut self.perlin, rng);
        record(self.dx, self.dy, &self.perlin, &mut self.next);
    }
}

fn is_veil(noise: f64) -> bool {
    noise < -0.2 || noise > 0.2
}

fn mutate<R: Rng>(perlin: &mut PerlinGrid, rng: &mut R) {
    perlin.scroll(rng, 0.2, 0.1);
    perlin.mutate(rng, 0.1);
}

fn record(dx: f64, dy: f64, perlin: &PerlinGrid, grid: &mut StaticGrid<bool>) {
    for (coord, cell) in izip!(grid.coord_iter(), grid.iter_mut()) {
        let x = dx * coord.x as f64;
        let y = dy * coord.y as f64;

        if let Some(noise) = perlin.noise(x, y) {
            *cell = is_veil(noise);
        }
    }
}
