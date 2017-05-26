// Implementation of Bressenham's algorithm

use cgmath::Vector2;
use vector_index::VectorIndex;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Octant {
    major_sign: i8,
    minor_sign: i8,
    major_axis: VectorIndex,
    minor_axis: VectorIndex,
}

impl Octant {
    fn choose(delta: Vector2<i32>) -> Self {
        let (major_axis, minor_axis) = if delta.x.abs() > delta.y.abs() {
            (VectorIndex::X, VectorIndex::Y)
        } else {
            (VectorIndex::Y, VectorIndex::X)
        };

        let major_sign = if major_axis.get(delta) < 0 { -1 } else { 1 };
        let minor_sign = if minor_axis.get(delta) < 0 { -1 } else { 1 };

        Octant {
            major_sign: major_sign,
            minor_sign: minor_sign,
            major_axis: major_axis,
            minor_axis: minor_axis,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InfiniteRelativeLineTraverse {
    octant: Octant,
    major_delta_abs: i32,
    minor_delta_abs: i32,
    accumulator: i32,
}

impl InfiniteRelativeLineTraverse {
    pub fn new(delta: Vector2<i32>) -> Self {
        let octant = Octant::choose(delta);
        InfiniteRelativeLineTraverse {
            major_delta_abs: octant.major_axis.get(delta).abs(),
            minor_delta_abs: octant.minor_axis.get(delta).abs(),
            accumulator: 0,
            octant: octant,
        }
    }

    pub fn step_in_place(&mut self) -> Vector2<i32> {
        let mut coord = Vector2::new(0, 0);

        // a single step of bresenham's algorithm
        self.accumulator += self.minor_delta_abs;

        self.octant.major_axis.set(&mut coord, self.octant.major_sign);

        if self.accumulator > (self.major_delta_abs) / 2 {
            self.accumulator -= self.major_delta_abs;
            self.octant.minor_axis.set(&mut coord, self.octant.minor_sign);
        }

        Vector2::new(coord.x as i32, coord.y as i32)
    }

    pub fn step(mut self) -> (Vector2<i32>, Self) {
        let coord = self.step_in_place();
        (coord, self)
    }

    pub fn reset_in_place(&mut self) {
        self.accumulator = 0;
    }

    pub fn reset(mut self) -> Self {
        self.reset_in_place();
        self
    }
}

impl Iterator for InfiniteRelativeLineTraverse {
    type Item = Vector2<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.step_in_place())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FiniteRelativeLineTraverse {
    infinite: InfiniteRelativeLineTraverse,
    count: u32,
    length: u32,
}

impl FiniteRelativeLineTraverse {
    pub fn new(delta: Vector2<i32>) -> Self {
        let infinite = InfiniteRelativeLineTraverse::new(delta);
        assert!(infinite.major_delta_abs >= 0, "Illegal negative absolute delta");

        FiniteRelativeLineTraverse {
            infinite: infinite,
            count: 0,
            // add 1 as lines are inclusive
            length: (infinite.major_delta_abs as u32) + 1,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.count == self.length
    }

    pub fn step_in_place(&mut self) -> Option<Vector2<i32>> {
        if self.is_complete() {
            None
        } else {
            let coord = self.infinite.step_in_place();
            self.count += 1;
            Some(coord)
        }
    }

    pub fn step(mut self) -> Option<(Vector2<i32>, Self)> {
        if let Some(coord) = self.step_in_place() {
            Some((coord, self))
        } else {
            None
        }
    }

    pub fn reset_in_place(&mut self) {
        self.infinite.reset_in_place();
        self.count = 0;
    }

    pub fn reset(mut self) -> Self {
        self.reset_in_place();
        self
    }

    pub fn exclude_end(self) -> Self {
        FiniteRelativeLineTraverse {
            infinite: self.infinite,
            count: self.count,
            length: self.length.saturating_sub(1),
        }
    }

    pub fn end(self) -> Vector2<i32> {
        let mut ret = Vector2::new(0, 0);

        let minor = self.infinite.minor_delta_abs * self.infinite.octant.minor_sign as i32;
        self.infinite.octant.minor_axis.set(&mut ret, minor);

        let major = self.infinite.major_delta_abs * self.infinite.octant.major_sign as i32;
        self.infinite.octant.major_axis.set(&mut ret, major);

        ret
    }

    pub fn split_end(self) -> (Self, Vector2<i32>) {
        (self.exclude_end(), self.end())
    }
}

impl Iterator for FiniteRelativeLineTraverse {
    type Item = Vector2<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        self.step_in_place()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InfiniteAbsoluteLineTraverse {
    relative: InfiniteRelativeLineTraverse,
    current: Vector2<i32>,
}

impl InfiniteAbsoluteLineTraverse {
    pub fn new_offset(start: Vector2<i32>, delta: Vector2<i32>) -> Self {
        InfiniteAbsoluteLineTraverse {
            relative: InfiniteRelativeLineTraverse::new(delta),
            current: start,
        }
    }

    pub fn new_between(start: Vector2<i32>, end: Vector2<i32>) -> Self {
        Self::new_offset(start, end - start)
    }

    pub fn step_in_place(&mut self) -> Vector2<i32> {
        let ret = self.current;
        self.current += self.relative.step_in_place();
        ret
    }

    pub fn step(mut self) -> (Vector2<i32>, Self) {
        let coord = self.step_in_place();
        (coord, self)
    }

    pub fn current(&self) -> Vector2<i32> {
        self.current
    }

    pub fn reset_in_place(&mut self, start: Vector2<i32>) {
        self.relative.reset_in_place();
        self.current = start;
    }

    pub fn reset(mut self, start: Vector2<i32>) -> Self {
        self.reset_in_place(start);
        self
    }
}

impl Iterator for InfiniteAbsoluteLineTraverse {
    type Item = Vector2<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.step_in_place())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FiniteAbsoluteLineTraverse {
    relative: FiniteRelativeLineTraverse,
    current: Vector2<i32>,
}

impl FiniteAbsoluteLineTraverse {
    pub fn new_offset(start: Vector2<i32>, delta: Vector2<i32>) -> Self {
        FiniteAbsoluteLineTraverse {
            relative: FiniteRelativeLineTraverse::new(delta),
            current: start,
        }
    }

    pub fn new_between(start: Vector2<i32>, end: Vector2<i32>) -> Self {
        Self::new_offset(start, end - start)
    }

    pub fn step_in_place(&mut self) -> Option<Vector2<i32>> {
        if let Some(delta) = self.relative.step_in_place() {
            let ret = self.current;
            self.current += delta;
            Some(ret)
        } else {
            None
        }
    }

    pub fn step(mut self) -> Option<(Vector2<i32>, Self)> {
        if let Some(coord) = self.step_in_place() {
            Some((coord, self))
        } else {
            None
        }
    }

    pub fn current(&self) -> Vector2<i32> {
        self.current
    }

    pub fn is_complete(&self) -> bool {
        self.relative.is_complete()
    }

    pub fn reset_in_place(&mut self, start: Vector2<i32>) {
        self.relative.reset_in_place();
        self.reset_position_in_place(start);
    }

    pub fn reset_position_in_place(&mut self, start: Vector2<i32>) {
        self.current = start;
    }

    pub fn reset(mut self, start: Vector2<i32>) -> Self {
        self.reset_in_place(start);
        self
    }

    pub fn reset_position(mut self, start: Vector2<i32>) -> Self {
        self.reset_position_in_place(start);
        self
    }

    pub fn exclude_end(self) -> Self {
        FiniteAbsoluteLineTraverse {
            relative: self.relative.exclude_end(),
            current: self.current,
        }
    }

    pub fn end(self) -> Vector2<i32> {
        self.current + self.relative.end()
    }

    pub fn split_end(self) -> (Self, Vector2<i32>) {
        (self.exclude_end(), self.end())
    }
}

impl Iterator for FiniteAbsoluteLineTraverse {
    type Item = Vector2<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        self.step_in_place()
    }
}
