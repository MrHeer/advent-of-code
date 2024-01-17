use crate::Direction;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn move_to(&mut self, direction: &Direction) -> &Self {
        use Direction::*;
        match direction {
            Up => self.row -= 1,
            Down => self.row += 1,
            Left => self.col -= 1,
            Right => self.col += 1,
        }
        self
    }
}
