use std::ops::{AddAssign, SubAssign};

use crate::Direction;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Position<T: Clone> {
    pub row: T,
    pub col: T,
}

impl<T> From<(T, T)> for Position<T>
where
    T: AddAssign + SubAssign + Clone,
{
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> Position<T>
where
    T: AddAssign + SubAssign + Clone,
{
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }

    pub fn move_to(&mut self, direction: &Direction, steps: T) -> &Self {
        use Direction::*;
        match direction {
            Up => self.row -= steps,
            Down => self.row += steps,
            Left => self.col -= steps,
            Right => self.col += steps,
        }
        self
    }
}
