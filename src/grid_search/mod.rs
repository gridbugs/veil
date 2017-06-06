use std::collections::VecDeque;
use std::result;
use std::slice;
use grid:: StaticGrid;
use direction::Direction;
use cgmath::Vector2;
use best::BestMapNonEmpty;
use coord::LookupCoord;

#[derive(Debug)]
pub enum Error {
    InvalidGridSize,
    NoPath,
    CantEnterDestination,
    DestinationOutsideKnowledge,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub struct Step {
    direction: Direction,
    to_coord: Vector2<i32>,
}

impl Step {
    pub fn direction(&self) -> Direction { self.direction }
    pub fn from_coord(&self) -> Vector2<i32> { self.to_coord - self.direction.vector() }
    pub fn to_coord(&self) -> Vector2<i32> { self.to_coord }
}

#[derive(Debug)]
pub struct Path {
    steps: Vec<Step>,
}

pub type PathIterFrom<'a> = slice::Iter<'a, Step>;

impl Path {
    pub fn new() -> Self {
        Path {
            steps: Vec::new(),
        }
    }

    pub fn get(&self, idx: usize) -> Option<Step> {
        self.steps.get(idx).cloned()
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn destination(&self) -> Option<Vector2<i32>> {
        self.steps.last().map(|step| step.to_coord())
    }

    pub fn source(&self) -> Option<Vector2<i32>> {
        self.steps.first().map(|step| step.from_coord())
    }

    pub fn iter_from(&self, idx: usize) -> PathIterFrom {
        self.steps.split_at(idx).1.iter()
    }

    pub fn first(&self) -> Option<Step> {
        self.steps.first().cloned()
    }

    pub fn contains(&self, idx: usize) -> bool {
        idx >= self.len()
    }
}

#[derive(Debug)]
struct Node {
    seen_seq: u64,
    entry_direction: Option<Direction>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            seen_seq: 0,
            entry_direction: None,
        }
    }
}

#[derive(Debug)]
pub struct SearchEnv {
    queue: VecDeque<Vector2<i32>>,
    node_grid: StaticGrid<Node>,
    seq: u64,
}

impl SearchEnv {
    pub fn new(width: usize, height: usize) -> Self {
        SearchEnv {
            queue: VecDeque::new(),
            node_grid: StaticGrid::new_default(width, height),
            seq: 0,
        }
    }

    fn clear(&mut self) {
        self.queue.clear();
        self.seq += 1;
    }

    fn see_first(&mut self, idx: Vector2<i32>) -> Result<()> {
        if let Some(c) = self.node_grid.get_mut(idx) {
            c.seen_seq = self.seq;
            c.entry_direction = None;
            Ok(())
        } else {
            Err(Error::InvalidGridSize)
        }
    }

    // sees the given index if it wasn't already seen, returning true iff it
    // was already seen or invalid
    fn see_unless_seen_or_invalid(&mut self, idx: Vector2<i32>, direction: Direction) -> bool {
        if let Some(cell) = self.node_grid.get_mut(idx) {
            if cell.seen_seq == self.seq {
                true
            } else {
                cell.seen_seq = self.seq;
                cell.entry_direction = Some(direction);
                false
            }
        } else {
            true
        }
    }

    fn construct_path(&self, mut idx: Vector2<i32>, path: &mut Path) -> Result<()> {
        path.steps.clear();
        loop {
            let cell = self.node_grid.get(idx).ok_or(Error::InvalidGridSize)?;

            if let Some(entry_direction) = cell.entry_direction {
                let step = Step {
                    direction: entry_direction,
                    to_coord: idx,
                };
                idx += entry_direction.opposite().vector();
                path.steps.push(step);
            } else {
                path.steps.reverse();
                return Ok(());
            }
        }
    }
}

pub fn bfs_best<Dirs, Grid, Cell, ScoreFn, Score, CanEnterFn>(
        env: &mut SearchEnv,
        knowledge: &Grid,
        start: Vector2<i32>,
        directions: Dirs,
        score: ScoreFn,
        can_enter: CanEnterFn,
        path: &mut Path) -> Result<()>
    where Dirs: Copy + IntoIterator<Item=Direction>,
          Grid: LookupCoord<Item=Cell>,
          Score: Ord + ::std::fmt::Debug,
          ScoreFn: Fn(&Cell) -> Score,
          CanEnterFn: Fn(&Cell) -> bool,
{
    env.clear();

    env.queue.push_back(start);
    env.see_first(start)?;

    let mut best = if let Some(c) = knowledge.lookup_coord(start) {
        BestMapNonEmpty::new(score(c), start)
    } else {
        return Err(Error::InvalidGridSize);
    };

    while let Some(current_coord) = env.queue.pop_front() {
        for direction in directions {
            let coord = current_coord + direction.vector();

            if env.see_unless_seen_or_invalid(coord, direction) {
                continue;
            }

            if let Some(knowledge_cell) = knowledge.lookup_coord(coord) {

                if can_enter(knowledge_cell) {
                    best.insert_gt(score(knowledge_cell), coord);
                } else {
                    continue;
                }

            } else {
                continue;
            }

            env.queue.push_back(coord);
        }
    }

    env.construct_path(best.into_value(), path)
}

pub fn bfs_coord<Dirs, Grid, Cell, CanEnterFn>(
        env: &mut SearchEnv,
        knowledge: &Grid,
        start: Vector2<i32>,
        directions: Dirs,
        dest: Vector2<i32>,
        can_enter: CanEnterFn,
        path: &mut Path) -> Result<()>
    where Dirs: Copy + IntoIterator<Item=Direction>,
          Grid: LookupCoord<Item=Cell>,
          CanEnterFn: Fn(&Cell) -> bool,
{
    env.clear();

    env.queue.push_back(start);
    env.see_first(start)?;

    while let Some(current_coord) = env.queue.pop_front() {
        for direction in directions {
            let coord = current_coord + direction.vector();

            if env.see_unless_seen_or_invalid(coord, direction) {
                continue;
            }

            if let Some(knowledge_cell) = knowledge.lookup_coord(coord) {

                if can_enter(knowledge_cell) {
                    if coord == dest {
                        return env.construct_path(dest, path);
                    }
                } else {
                    if coord == dest {
                        return Err(Error::CantEnterDestination);
                    }
                    continue;
                }

            } else {
                if coord == dest {
                    return Err(Error::DestinationOutsideKnowledge);
                }
                continue;
            }

            env.queue.push_back(coord);
        }
    }

    Err(Error::NoPath)
}

pub fn bfs_predicate<Dirs, Grid, Cell, PredFn, CanEnterFn>(
        env: &mut SearchEnv,
        knowledge: &Grid,
        start: Vector2<i32>,
        directions: Dirs,
        pred: PredFn,
        can_enter: CanEnterFn,
        path: &mut Path) -> Result<()>
    where Dirs: Copy + IntoIterator<Item=Direction>,
          Grid: LookupCoord<Item=Cell>,
          PredFn: Fn(&Cell) -> bool,
          CanEnterFn: Fn(&Cell) -> bool,
{
    env.clear();

    env.queue.push_back(start);
    env.see_first(start)?;

    while let Some(current_coord) = env.queue.pop_front() {
        for direction in directions {
            let coord = current_coord + direction.vector();

            if env.see_unless_seen_or_invalid(coord, direction) {
                continue;
            }

            if let Some(knowledge_cell) = knowledge.lookup_coord(coord) {

                if can_enter(knowledge_cell) {
                    if pred(knowledge_cell) {
                        return env.construct_path(coord, path);
                    }
                } else {
                    continue;
                }

            } else {
                continue;
            }

            env.queue.push_back(coord);
        }
    }

    Err(Error::NoPath)
}
