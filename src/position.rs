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

    pub fn move_to(&self, direction: &Direction) -> Self {
        let Position { mut row, mut col } = *self;

        use Direction::*;
        match direction {
            Up => row -= 1,
            Down => row += 1,
            Left => col -= 1,
            Right => col += 1,
        }
        Position::new(row, col)
    }
}
