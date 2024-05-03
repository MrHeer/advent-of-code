use std::ops::{AddAssign, SubAssign};

use num::One;

use crate::Direction;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Position<T> {
    pub row: T,
    pub col: T,
}

impl<T> From<(T, T)> for Position<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> Position<T> {
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }
}

impl<T> Position<T>
where
    T: AddAssign + SubAssign,
{
    pub fn move_to(&mut self, direction: &Direction, steps: T) -> &mut Self {
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

impl<T> Position<T>
where
    T: AddAssign + SubAssign + One + Copy,
{
    pub fn adjacent_positions(&self) -> Vec<Self> {
        use Direction::*;
        [
            *self.clone().move_to(&Up, One::one()),
            *self.clone().move_to(&Down, One::one()),
            *self.clone().move_to(&Left, One::one()),
            *self.clone().move_to(&Right, One::one()),
        ]
        .into_iter()
        .collect()
    }
}
