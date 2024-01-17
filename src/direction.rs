#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        self.turn_back().turn_left()
    }

    pub fn turn_back(&self) -> Self {
        self.turn_left().turn_left()
    }
}
