pub struct StaticGrid<T> {
    items: Vec<T>,
    width: usize,
    height: usize,
    size: usize,
}

pub trait StaticGridIdx: Copy {
    fn wrap_to_index(self, width: usize) -> usize;
    fn is_valid(self, width: usize) -> bool;
}

impl StaticGridIdx for (usize, usize) {
    fn wrap_to_index(self, width: usize) -> usize {
        self.1 * width + self.0
    }
    fn is_valid(self, width: usize) -> bool {
        self.0 < width
    }
}

impl StaticGridIdx for (isize, isize) {
    fn wrap_to_index(self, width: usize) -> usize {
        (self.1 as usize) * width + (self.0 as usize)
    }
    fn is_valid(self, width: usize) -> bool {
        self.0 >= 0 && self.1 >= 0 && (self.0 as usize) < width
    }
}

impl<T> StaticGrid<T> {
    fn new_with_capacity(width: usize, height: usize) -> Self {
        let size = width * height;
        StaticGrid {
            items: Vec::with_capacity(size),
            width: width,
            height: height,
            size: size,
        }
    }
}

impl<T: Default> StaticGrid<T> {
    pub fn new_default(width: usize, height: usize) -> Self {
        let mut grid = Self::new_with_capacity(width, height);
        for _ in 0..grid.size {
            grid.items.push(Default::default());
        }
        grid
    }
}

impl<T: Copy> StaticGrid<T> {
    pub fn new_copy(width: usize, height: usize, item: T) -> Self {
        let mut grid = Self::new_with_capacity(width, height);
        for _ in 0..grid.size {
            grid.items.push(item);
        }
        grid
    }
}

impl<T> StaticGrid<T> {
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    fn wrap_to_index<I: StaticGridIdx>(&self, idx: I) -> usize {
        idx.wrap_to_index(self.width)
    }
    fn is_valid<I: StaticGridIdx>(&self, idx: I) -> bool {
        idx.is_valid(self.width)
    }

    pub fn get<I: StaticGridIdx>(&self, idx: I) -> Option<&T> {
        if self.is_valid(idx) {
            self.get_valid(idx)
        } else {
            None
        }
    }
    pub fn get_mut<I: StaticGridIdx>(&mut self, idx: I) -> Option<&mut T> {
        if self.is_valid(idx) {
            self.get_valid_mut(idx)
        } else {
            None
        }
    }

    pub fn get_valid<I: StaticGridIdx>(&self, idx: I) -> Option<&T> {
        self.items.get(self.wrap_to_index(idx))
    }
    pub fn get_valid_mut<I: StaticGridIdx>(&mut self, idx: I) -> Option<&mut T> {
        let wrapped_idx = self.wrap_to_index(idx);
        self.items.get_mut(wrapped_idx)
    }

    pub unsafe fn get_unchecked<I: StaticGridIdx>(&self, idx: I) -> &T {
        self.items.get_unchecked(self.wrap_to_index(idx))
    }

    pub unsafe fn get_unchecked_mut<I: StaticGridIdx>(&mut self, idx: I) -> &mut T {
        let wrapped_idx = self.wrap_to_index(idx);
        self.items.get_unchecked_mut(wrapped_idx)
    }
}
