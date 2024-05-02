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

impl Position<isize> {
    pub fn adjacent_positions(&self) -> Vec<Self> {
        [
            *self.clone().move_to(&Direction::Up, 1),
            *self.clone().move_to(&Direction::Down, 1),
            *self.clone().move_to(&Direction::Left, 1),
            *self.clone().move_to(&Direction::Right, 1),
        ]
        .into_iter()
        .collect()
    }
}

impl Position<usize> {
    pub fn adjacent_positions(&self) -> Vec<Self> {
        [
            *self.clone().move_to(&Direction::Up, 1),
            *self.clone().move_to(&Direction::Down, 1),
            *self.clone().move_to(&Direction::Left, 1),
            *self.clone().move_to(&Direction::Right, 1),
        ]
        .into_iter()
        .collect()
    }
}
