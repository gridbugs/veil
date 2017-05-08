use cgmath::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdinalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    pub fn vector(self) -> Vector2<i32> {
        match self {
            Direction::North => Vector2::new(0, -1),
            Direction::NorthEast => Vector2::new(1, -1),
            Direction::East => Vector2::new(1, 0),
            Direction::SouthEast => Vector2::new(1, 1),
            Direction::South => Vector2::new(0, 1),
            Direction::SouthWest => Vector2::new(-1, 1),
            Direction::West => Vector2::new(-1, 0),
            Direction::NorthWest => Vector2::new(-1, -1),
        }
    }

    pub fn left90(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::NorthEast => Direction::NorthWest,
            Direction::East => Direction::North,
            Direction::SouthEast => Direction::NorthEast,
            Direction::South => Direction::East,
            Direction::SouthWest => Direction::SouthEast,
            Direction::West => Direction::South,
            Direction::NorthWest => Direction::SouthWest,
        }
    }
}

impl CardinalDirection {
    pub fn direction(self) -> Direction {
        match self {
            CardinalDirection::North => Direction::North,
            CardinalDirection::East => Direction::East,
            CardinalDirection::South => Direction::South,
            CardinalDirection::West => Direction::West,
        }
    }

    pub fn opposite(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }

    pub fn vector(self) -> Vector2<i32> {
        match self {
            CardinalDirection::North => Vector2::new(0, -1),
            CardinalDirection::East => Vector2::new(1, 0),
            CardinalDirection::South => Vector2::new(0, 1),
            CardinalDirection::West => Vector2::new(-1, 0),
        }
    }

    pub fn left90(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::North,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::West => CardinalDirection::South,
        }
    }
}

impl OrdinalDirection {
    pub fn direction(self) -> Direction {
        match self {
            OrdinalDirection::NorthEast => Direction::NorthEast,
            OrdinalDirection::SouthEast => Direction::SouthEast,
            OrdinalDirection::SouthWest => Direction::SouthWest,
            OrdinalDirection::NorthWest => Direction::NorthWest,
        }
    }

    pub fn opposite(self) -> OrdinalDirection {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::SouthWest,
            OrdinalDirection::SouthEast => OrdinalDirection::NorthWest,
            OrdinalDirection::SouthWest => OrdinalDirection::NorthEast,
            OrdinalDirection::NorthWest => OrdinalDirection::SouthEast,
        }
    }

    pub fn vector(self) -> Vector2<i32> {
        match self {
            OrdinalDirection::NorthEast => Vector2::new(1, -1),
            OrdinalDirection::SouthEast => Vector2::new(1, 1),
            OrdinalDirection::SouthWest => Vector2::new(-1, 1),
            OrdinalDirection::NorthWest => Vector2::new(-1, -1),
        }
    }

    pub fn left90(self) -> OrdinalDirection {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::NorthWest,
            OrdinalDirection::SouthEast => OrdinalDirection::NorthEast,
            OrdinalDirection::SouthWest => OrdinalDirection::SouthEast,
            OrdinalDirection::NorthWest => OrdinalDirection::SouthWest,
        }
    }

    pub fn from_cardinals(a: CardinalDirection, b: CardinalDirection) -> Option<Self> {
        match a {
            CardinalDirection::North => {
                match b {
                    CardinalDirection::East => return Some(OrdinalDirection::NorthEast),
                    CardinalDirection::West => return Some(OrdinalDirection::NorthEast),
                    _ => return None,
                }
            }
            CardinalDirection::East => {
                match b {
                    CardinalDirection::North => return Some(OrdinalDirection::NorthEast),
                    CardinalDirection::South => return Some(OrdinalDirection::SouthEast),
                    _ => return None,
                }
            }
            CardinalDirection::South => {
                match b {
                    CardinalDirection::East => return Some(OrdinalDirection::SouthEast),
                    CardinalDirection::West => return Some(OrdinalDirection::SouthWest),
                    _ => return None,
                }
            }
            CardinalDirection::West => {
                match b {
                    CardinalDirection::North => return Some(OrdinalDirection::NorthWest),
                    CardinalDirection::South => return Some(OrdinalDirection::SouthWest),
                    _ => return None,
                }
            }
        }
    }
}
