#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_next_direction_valid(&self, next: &Direction) -> bool {
        match self {
            Direction::Up => *next != Direction::Down,
            Direction::Down => *next != Direction::Up,
            Direction::Left => *next != Direction::Right,
            Direction::Right => *next != Direction::Left,
        }
    }
}
