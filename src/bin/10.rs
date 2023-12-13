advent_of_code::solution!(10);

#[derive(PartialEq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(PartialEq)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

use std::vec;

use self::Pipe::*;
use Tile::*;

impl Tile {
    fn new(ch: &char) -> Option<Self> {
        match ch {
            '|' => Some(Pipe(NorthSouth)),
            '-' => Some(Pipe(EastWest)),
            'L' => Some(Pipe(NorthEast)),
            'J' => Some(Pipe(NorthWest)),
            '7' => Some(Pipe(SouthWest)),
            'F' => Some(Pipe(SouthEast)),
            '.' => Some(Ground),
            'S' => Some(Start),
            _ => None,
        }
    }
}

#[derive(PartialEq)]
struct Position {
    row: u32,
    col: u32,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: Position,
    row: u32,
    col: u32,
}

impl Grid {
    fn new(grid: &str) -> Self {
        let (mut row, mut col) = (0, 0);
        let mut start_position = None;
        let mut tiles = vec![];
        grid.lines().for_each(|line| {
            row += 1;
            col = 0;
            let mut row_tiles = vec![];
            line.chars().for_each(|ch| {
                col += 1;
                let tile = Tile::new(&ch).unwrap();
                if start_position.is_none() && tile == Start {
                    start_position = Some(Position { row, col });
                }
                row_tiles.push(tile);
            });
            tiles.push(row_tiles);
        });

        Self {
            tiles,
            start: start_position.unwrap(),
            row,
            col,
        }
    }

    fn is_valid(&self, pos: &Position) -> bool {
        let Position { row, col } = *pos;
        1 <= row && row <= self.row && 1 <= col && col <= self.col
    }

    fn assert_position(&self, pos: &Position) {
        assert!(self.is_valid(pos), "pos is not valid.");
    }

    fn get_tile(&self, pos: &Position) -> &Tile {
        self.assert_position(pos);
        let Position { row, col } = *pos;
        &self.tiles[(row - 1) as usize][(col - 1) as usize]
    }

    fn get_adjacent_position(&self, pos: &Position) -> Vec<Position> {
        self.assert_position(pos);
        let Position { row, col } = *pos;
        let tile = self.get_tile(pos);

        let positions = match tile {
            Tile::Pipe(pipe) => match pipe {
                NorthSouth => vec![
                    Position { row: row - 1, col },
                    Position { row: row + 1, col },
                ],
                EastWest => vec![
                    Position {
                        row: row,
                        col: col - 1,
                    },
                    Position {
                        row: row,
                        col: col + 1,
                    },
                ],
                NorthEast => vec![
                    Position { row: row - 1, col },
                    Position {
                        row: row,
                        col: col + 1,
                    },
                ],
                NorthWest => vec![
                    Position { row: row - 1, col },
                    Position {
                        row: row,
                        col: col - 1,
                    },
                ],
                SouthWest => vec![
                    Position { row: row + 1, col },
                    Position {
                        row: row,
                        col: col - 1,
                    },
                ],
                SouthEast => vec![
                    Position { row: row + 1, col },
                    Position {
                        row: row,
                        col: col + 1,
                    },
                ],
            },
            Ground => vec![],
            Start => vec![
                Position { row: row - 1, col },
                Position { row: row + 1, col },
                Position {
                    row: row,
                    col: col - 1,
                },
                Position {
                    row: row,
                    col: col + 1,
                },
            ],
        };

        positions
            .into_iter()
            .filter(|pos| self.is_valid(pos))
            .collect()
    }

    fn is_connect(&self, pos: &Position, other_pos: &Position) -> bool {
        assert!(pos != other_pos, "There are same position.");
        let adjacent_positions = self.get_adjacent_position(pos);
        let other_adjacent_positions = self.get_adjacent_position(other_pos);
        if adjacent_positions.contains(other_pos) && other_adjacent_positions.contains(pos) {
            return true;
        }
        false
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
