use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::Position;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Matrix<T> {
    cells: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> From<&str> for Matrix<T>
where
    T: From<char>,
{
    fn from(value: &str) -> Self {
        value
            .lines()
            .map(|line| line.chars().map(Into::into).collect())
            .collect::<Vec<Vec<T>>>()
            .into()
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let rows = value.len();
        let cols = value.first().map(|row| row.len()).unwrap_or_default();

        Self {
            cells: value,
            rows,
            cols,
        }
    }
}

impl<T> Index<Position> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        if self.is_valid_position(&index) {
            &self.cells[index.row - 1][index.col - 1]
        } else {
            panic!("Please give correct position.");
        }
    }
}

impl<T> IndexMut<Position> for Matrix<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        if self.is_valid_position(&index) {
            &mut self.cells[index.row - 1][index.col - 1]
        } else {
            panic!("Please give correct position.");
        }
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.row_iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Matrix<T> {
    pub fn is_valid_position(&self, position: &Position) -> bool {
        let Position { row, col } = *position;
        1 <= row && row <= self.rows && 1 <= col && col <= self.cols
    }

    pub fn get_row(&self, index: usize) -> &Vec<T> {
        &self.cells[index]
    }

    pub fn row_iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.cells.iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.row_iter().flat_map(|row| row.iter())
    }

    pub fn map<U, F>(&self, mut f: F) -> Matrix<U>
    where
        F: FnMut(&T) -> U,
    {
        self.row_iter()
            .map(|row| row.iter().map(&mut f).collect())
            .collect::<Vec<Vec<U>>>()
            .into()
    }
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn transpose(&self) -> Self {
        let cells = (0..self.cols)
            .map(|col| self.row_iter().map(|row| row[col]).collect())
            .collect();

        Self {
            cells,
            rows: self.rows,
            cols: self.cols,
        }
    }
}
