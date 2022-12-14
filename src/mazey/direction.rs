#[derive(Debug)]
pub enum Direction {
    None,
    Up,
    Right,
    Down,
    Left,
}

pub const ALL_DIRECTIONS: [&Direction; 4] = [ &Direction::Up, &Direction::Right, &Direction::Down, &Direction::Left ];

impl Direction {
    pub fn to_door_mask(&self) -> u32 {
        match self {
            Direction::None => 0,
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 4,
            Direction::Right => 8
        }
    }

    pub fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        }
    }

    pub fn to_opposite(&self) -> &Self {
        match self {
            Direction::None => &Direction::None,
            Direction::Up => &Direction::Down,
            Direction::Down => &Direction::Up,
            Direction::Left => &Direction::Right,
            Direction::Right => &Direction::Left
        }
    }
}
