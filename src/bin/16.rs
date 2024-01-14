use std::{collections::HashSet, vec};

advent_of_code::solution!(16);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

enum Mirror {
    Slash,
    BackSlash,
}

enum Splitter {
    Horizontal,
    Vertical,
}

enum Tile {
    Empty,
    Mirror(Mirror),
    Splitter(Splitter),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Beam {
    position: Position,
    direction: Direction,
}

struct Contraption {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
}

impl From<char> for Mirror {
    fn from(value: char) -> Self {
        use Mirror::*;
        match value {
            '/' => Slash,
            '\\' => BackSlash,
            _ => panic!("Could not resolve the mirror."),
        }
    }
}

impl From<char> for Splitter {
    fn from(value: char) -> Self {
        use Splitter::*;
        match value {
            '-' => Horizontal,
            '|' => Vertical,
            _ => panic!("Could not resolve the splitter."),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            '.' => Empty,
            '/' | '\\' => Mirror(value.into()),
            '-' | '|' => Splitter(value.into()),
            _ => panic!("Could not resolve the tile."),
        }
    }
}

use Direction::*;

impl Default for Beam {
    fn default() -> Self {
        Self::new(1, 1, Right)
    }
}

impl Beam {
    fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            position: Position { row, col },
            direction,
        }
    }

    fn move_forward(mut self) -> Self {
        match self.direction {
            Up => self.position.row -= 1,
            Down => self.position.row += 1,
            Left => self.position.col -= 1,
            Right => self.position.col += 1,
        }

        self
    }

    fn move_in_mirror(self, mirror: &Mirror) -> Self {
        use Mirror::*;
        match (self.direction, mirror) {
            (Left, BackSlash) | (Right, Slash) => self.change_direction(Up),
            (Left, Slash) | (Right, BackSlash) => self.change_direction(Down),
            (Up, BackSlash) | (Down, Slash) => self.change_direction(Left),
            (Up, Slash) | (Down, BackSlash) => self.change_direction(Right),
        }
        .move_forward()
    }

    fn change_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    fn move_in_splitter(self, splitter: &Splitter) -> Vec<Self> {
        use Splitter::*;
        let beams = match (self.direction, splitter) {
            (Up, Horizontal) | (Down, Horizontal) => vec![
                self.clone().change_direction(Left),
                self.clone().change_direction(Right),
            ],
            (Left, Vertical) | (Right, Vertical) => vec![
                self.clone().change_direction(Up),
                self.clone().change_direction(Down),
            ],
            _ => vec![self],
        };

        beams.into_iter().map(|beam| beam.move_forward()).collect()
    }

    fn move_in_tile(self, tile: &Tile) -> Vec<Self> {
        use Tile::*;
        match tile {
            Empty => vec![self.move_forward()],
            Mirror(mirror) => vec![self.move_in_mirror(mirror)],
            Splitter(splitter) => self.move_in_splitter(splitter),
        }
    }
}

impl From<&str> for Contraption {
    fn from(value: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = value
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        let rows = tiles.len();
        let cols = tiles.first().map(|row| row.len()).unwrap_or_default();

        Self { tiles, rows, cols }
    }
}

impl Contraption {
    fn get_tile(&self, position: &Position) -> &Tile {
        let Position { row, col } = *position;
        &self.tiles[row - 1][col - 1]
    }

    fn is_valid_position(&self, position: &Position) -> bool {
        let Position { row, col } = *position;
        1 <= row && row <= self.rows && 1 <= col && col <= self.cols
    }

    fn is_valid_beam(&self, beam: &Beam) -> bool {
        self.is_valid_position(&beam.position)
    }

    fn move_beam(&self, beam: &Beam, visited: &mut HashSet<Beam>) -> Option<Vec<Beam>> {
        if self.is_valid_beam(beam) == false {
            // the beam in out of the tiles.
            return None;
        }
        if visited.contains(beam) {
            // the beam visited this tile, cycle start.
            return None;
        }
        visited.insert(*beam);

        let tile = self.get_tile(&beam.position);
        let beams = beam.move_in_tile(&tile);

        Some(beams)
    }

    fn energizes(&self, beam: &Beam) -> u32 {
        let mut beams = vec![beam.clone()];
        let mut visited = HashSet::new();

        while let Some(beam) = beams.pop() {
            if let Some(mut moved_beams) = self.move_beam(&beam, &mut visited) {
                beams.append(&mut moved_beams);
            }
        }

        let mut visited_positions = HashSet::new();
        visited.iter().for_each(|beam| {
            visited_positions.insert(beam.position);
        });

        visited_positions.iter().count() as u32
    }

    fn get_all_edge_beams(&self) -> Vec<Beam> {
        let mut beams = vec![];

        (1..=self.rows).for_each(|row| {
            beams.push(Beam::new(row, 1, Right));
        });

        (1..=self.rows).for_each(|row| {
            beams.push(Beam::new(row, self.cols, Left));
        });

        (1..=self.cols).for_each(|col| {
            beams.push(Beam::new(1, col, Down));
        });

        (1..=self.cols).for_each(|col| {
            beams.push(Beam::new(self.rows, col, Up));
        });

        beams
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(Contraption::from(input).energizes(&Beam::default()))
}

pub fn part_two(input: &str) -> Option<u32> {
    let contraption = Contraption::from(input);
    contraption
        .get_all_edge_beams()
        .iter()
        .map(|beam| contraption.energizes(beam))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
