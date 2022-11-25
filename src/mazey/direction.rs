#[derive(Debug)]
pub enum Direction {
    None,
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn to_door_mask(&self) -> u8 {
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
}
