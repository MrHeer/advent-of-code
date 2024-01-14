#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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
}
