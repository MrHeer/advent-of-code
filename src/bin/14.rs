use advent_of_code::Matrix;
use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

advent_of_code::solution!(14);

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Rounded,
    Cube,
    Empty,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Solver {
    tiles: Matrix<Tile>,
}

use Tile::*;

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Rounded,
            '#' => Cube,
            '.' => Empty,
            _ => panic!("Could not resolve tile."),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Rounded => 'O',
            Cube => '#',
            Empty => '.',
        };
        f.write_char(ch)
    }
}

impl From<&str> for Solver {
    fn from(value: &str) -> Self {
        Self {
            tiles: Matrix::from(value),
        }
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.row_iter() {
            for tile in row {
                write!(f, "{}", tile)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Solver {
    fn transpose(&self) -> Self {
        Self {
            tiles: self.tiles.transpose(),
        }
    }

    fn slide_to_head_in_row(row: &[Tile]) -> Vec<Tile> {
        let mut slide_row = vec![Empty; row.len()];
        let mut rounded_position = 0;
        row.iter().enumerate().for_each(|(index, tile)| {
            if *tile == Rounded {
                slide_row[rounded_position] = Rounded;
                rounded_position += 1;
            }
            if *tile == Cube {
                slide_row[index] = Cube;
                rounded_position = index + 1;
            }
        });
        slide_row
    }

    fn slide_to_tail_in_row(row: &[Tile]) -> Vec<Tile> {
        let mut reversed_row = row.to_owned();
        reversed_row.reverse();
        let mut reversed_slide_row = Self::slide_to_head_in_row(&reversed_row);
        reversed_slide_row.reverse();
        reversed_slide_row
    }

    fn slide_to_north(&self) -> Self {
        self.transpose().slide_to_west().transpose()
    }

    fn slide_to_west(&self) -> Self {
        let tiles = self
            .tiles
            .row_iter()
            .map(|tiles| Self::slide_to_head_in_row(tiles))
            .collect::<Vec<Vec<Tile>>>()
            .into();
        Self { tiles }
    }

    fn slide_to_south(&self) -> Self {
        self.transpose().slide_to_east().transpose()
    }

    fn slide_to_east(&self) -> Self {
        let tiles = self
            .tiles
            .row_iter()
            .map(|tiles| Self::slide_to_tail_in_row(tiles))
            .collect::<Vec<Vec<Tile>>>()
            .into();
        Self { tiles }
    }

    fn cycle(&self) -> Self {
        self.slide_to_north()
            .slide_to_west()
            .slide_to_south()
            .slide_to_east()
    }

    fn load(&self) -> u32 {
        self.tiles
            .row_iter()
            .enumerate()
            .map(|(index, row)| {
                row.iter().filter(|&tile| *tile == Rounded).count() as u32
                    * (self.tiles.rows as u32 - index as u32)
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(Solver::from(input).slide_to_north().load())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut solver = Solver::from(input);
    let mut index_map = HashMap::new();
    let mut index: u32 = 0;
    loop {
        index_map.insert(solver.clone(), index);
        solver = solver.cycle();
        index += 1;
        if index_map.contains_key(&solver) {
            break;
        }
    }
    let cycle_start = *index_map.get(&solver).unwrap();
    let cycle_length = index - cycle_start;

    for _ in 0..(1_000_000_000 - cycle_start) % cycle_length {
        solver = solver.cycle();
    }

    Some(solver.load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
