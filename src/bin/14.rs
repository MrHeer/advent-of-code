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
struct Puzzle {
    tiles: Vec<Vec<Tile>>,
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

impl From<&str> for Puzzle {
    fn from(value: &str) -> Self {
        let tiles = value
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Self { tiles }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row {
                write!(f, "{}", tile)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Puzzle {
    fn rows(&self) -> u32 {
        self.tiles.len() as u32
    }

    fn transpose(&self) -> Self {
        let tiles = (0..self.tiles[0].len())
            .map(|col| self.tiles.iter().map(|p| p[col]).collect())
            .collect();

        Self { tiles }
    }

    fn slide_to_head_in_row(row: &Vec<Tile>) -> Vec<Tile> {
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

    fn slide_to_tail_in_row(row: &Vec<Tile>) -> Vec<Tile> {
        let mut reversed_row = row.clone();
        reversed_row.reverse();
        let mut reversed_slide_row = Self::slide_to_head_in_row(&reversed_row);
        reversed_slide_row.reverse();
        reversed_slide_row
    }

    fn slide_to_north(&self) -> Self {
        self.transpose().slide_to_west().transpose()
    }

    fn slide_to_west(&self) -> Self {
        let tiles = self.tiles.iter().map(Self::slide_to_head_in_row).collect();
        Self { tiles }
    }

    fn slide_to_south(&self) -> Self {
        self.transpose().slide_to_east().transpose()
    }

    fn slide_to_east(&self) -> Self {
        let tiles = self.tiles.iter().map(Self::slide_to_tail_in_row).collect();
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
            .iter()
            .enumerate()
            .map(|(index, row)| {
                row.iter().filter(|&tile| *tile == Rounded).count() as u32
                    * (self.rows() - index as u32)
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(Puzzle::from(input).slide_to_north().load())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut puzzle = Puzzle::from(input);
    let mut index_map = HashMap::new();
    let mut index: u32 = 0;
    loop {
        index_map.insert(puzzle.clone(), index);
        puzzle = puzzle.cycle();
        index += 1;
        if index_map.contains_key(&puzzle) {
            break;
        }
    }
    let cycle_start = *index_map.get(&puzzle).unwrap();
    let cycle_length = index - cycle_start;

    for _ in 0..(1_000_000_000 - cycle_start) % cycle_length {
        puzzle = puzzle.cycle();
    }

    return Some(puzzle.load());
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
