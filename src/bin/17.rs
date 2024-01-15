use std::collections::HashSet;

use advent_of_code::{Direction, Matrix, Position};

advent_of_code::solution!(17);

struct Crucible {
    position: Position,
    direction: Direction,
    forward_times: usize,
}

type Path = HashSet<Position>;

struct Puzzle {
    blocks: Matrix<u32>,
}

impl Default for Crucible {
    fn default() -> Self {
        Crucible::new(1, 1, Direction::Right)
    }
}

impl Crucible {
    fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            position: (row, col).into(),
            direction,
            forward_times: 0,
        }
    }

    fn move_forward(self) -> Self {
        // self.position = match self.direction {
        //     Direction::Up => todo!(),
        //     Direction::Down => todo!(),
        //     Direction::Left => todo!(),
        //     Direction::Right => todo!(),
        // }
        // self.forward_times += 1;

        self
    }
}

impl From<&str> for Puzzle {
    fn from(value: &str) -> Self {
        Self {
            blocks: Matrix::from(value),
        }
    }
}

impl Puzzle {
    fn find_path(&self, start: &Position, end: &Position) -> Vec<Path> {
        vec![]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
