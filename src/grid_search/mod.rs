use std::collections::VecDeque;
use std::result;
use std::slice;
use knowledge::PlayerKnowledgeGrid;
use grid:: StaticGrid;
use direction::Direction;
use cgmath::Vector2;
use best::BestMapNonEmpty;
use invert_ord::InvertOrd;

#[derive(Debug)]
pub enum Error {
    InvalidGridSize,
    AlreadyAtDestination,
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

    fn see(&mut self, idx: Vector2<i32>, direction: Direction) -> Result<()> {
        if let Some(c) = self.node_grid.get_mut(idx) {
            c.seen_seq = self.seq;
            c.entry_direction = Some(direction);
            Ok(())
        } else {
            Err(Error::InvalidGridSize)
        }
    }

    fn is_seen(&self, idx: Vector2<i32>) -> Result<bool> {
        self.node_grid.get(idx).map(|n| n.seen_seq == self.seq).ok_or(Error::InvalidGridSize)
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

pub fn bfs<D>(env: &mut SearchEnv, knowledge: &PlayerKnowledgeGrid, start: Vector2<i32>, directions: D, path: &mut Path) -> Result<()>
    where D: Copy + IntoIterator<Item=Direction>
{
    env.clear();

    env.queue.push_back(start);
    env.see_first(start)?;

    let mut best = if let Some(knowledge_cell) = knowledge.get(start) {
        BestMapNonEmpty::new(InvertOrd::new(knowledge_cell.last_updated), start)
    } else {
        return Err(Error::InvalidGridSize);
    };

    while let Some(current_coord) = env.queue.pop_front() {
        for (direction, coord) in izip!(directions, env.node_grid.neighbour_coord_iter(current_coord, directions)) {
            if env.is_seen(coord)? {
                continue;
            }

            env.see(coord, direction)?;

            if let Some(knowledge_cell) = knowledge.get(coord) {
                if knowledge_cell.solid && knowledge_cell.door.is_none() {
                    continue;
                }

                best.insert(InvertOrd::new(knowledge_cell.last_updated), coord);
            } else {
                continue;
            }

            env.queue.push_back(coord);
        }
    }

    let dest = best.into_value();
    if dest == start {
        return Err(Error::AlreadyAtDestination);
    }

    env.construct_path(dest, path)
}
