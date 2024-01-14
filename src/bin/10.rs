use advent_of_code::{Matrix, Position};
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

struct Grid {
    tiles: Matrix<Tile>,
    start: Position,
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
                    start_position = Some((row, col).into());
                }
                row_tiles.push(tile);
            });
            tiles.push(row_tiles);
        });

        Self {
            tiles: tiles.into(),
            start: start_position.unwrap(),
        }
    }

    fn get_tile(&self, pos: &Position) -> &Tile {
        &self.tiles[*pos]
    }

    fn get_pipe_adjacent_positions(&self, pos: &Position) -> Vec<Position> {
        let Position { row, col } = *pos;
        let tile = self.get_tile(pos);

        let positions = match tile {
            Tile::Pipe(pipe) => match pipe {
                NorthSouth => vec![(row - 1, col), (row + 1, col)],
                EastWest => vec![(row, col - 1), (row, col + 1)],
                NorthEast => vec![(row - 1, col), (row, col + 1)],
                NorthWest => vec![(row - 1, col), (row, col - 1)],
                SouthWest => vec![(row + 1, col), (row, col - 1)],
                SouthEast => vec![(row + 1, col), (row, col + 1)],
            },
            Ground => vec![],
            Start => vec![
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1),
            ],
        }
        .into_iter()
        .map(Position::from);

        positions
            .into_iter()
            .filter(|pos| self.tiles.is_valid_position(pos))
            .collect()
    }

    fn is_connect(&self, pos: &Position, other_pos: &Position) -> bool {
        assert!(pos != other_pos, "There are same position.");
        let adjacent_positions = self.get_pipe_adjacent_positions(pos);
        let other_adjacent_positions = self.get_pipe_adjacent_positions(other_pos);
        if adjacent_positions.contains(other_pos) && other_adjacent_positions.contains(pos) {
            return true;
        }
        false
    }

    fn get_connected_pipe_positions(&self, pos: &Position) -> Vec<Position> {
        self.get_pipe_adjacent_positions(pos)
            .into_iter()
            .filter(|adjacent_pos| self.is_connect(pos, adjacent_pos))
            .collect()
    }

    fn get_giant_loop(&self) -> Vec<Position> {
        let mut giant_loop = vec![];

        for pos in self.get_connected_pipe_positions(&self.start) {
            giant_loop.clear();
            let mut current_pos = self.start;
            let mut next = Some(pos);
            while let Some(next_pos) = next {
                let prev_pos = current_pos;
                current_pos = next_pos;

                if current_pos == self.start {
                    break;
                }
                giant_loop.push(current_pos);

                next = self
                    .get_connected_pipe_positions(&current_pos)
                    .into_iter()
                    .filter(|pos| *pos != prev_pos)
                    .next();
            }
        }

        giant_loop
    }

    // Shoelace Formula - https://en.m.wikipedia.org/wiki/Shoelace_formula
    fn get_area_of_loop(&self) -> usize {
        let mut giant_loop = self.get_giant_loop();
        giant_loop.insert(0, self.start);
        giant_loop.push(self.start);
        let mut area: i32 = 0;

        for i in 0..giant_loop.len() - 1 {
            let Position { row: y_i, col: x_i } = giant_loop[i];
            let Position {
                row: y_i_plus_1,
                col: x_i_plus_1,
            } = giant_loop[i + 1];

            area += (y_i as i32 + y_i_plus_1 as i32) * (x_i as i32 - x_i_plus_1 as i32) / 2;
        }

        area.abs() as usize
    }

    // Pick's Theorem - https://en.m.wikipedia.org/wiki/Pick%27s_theorem
    fn get_number_of_interior_points(&self) -> usize {
        let area = self.get_area_of_loop();
        let number_of_boundary_points = self.get_giant_loop().len() + 1;

        area + 1 - number_of_boundary_points / 2
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some((Grid::new(input).get_giant_loop().len() as f32 / 2.).ceil() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(Grid::new(input).get_number_of_interior_points() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }
}
