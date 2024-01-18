use advent_of_code::{Direction, Matrix, Movable, Position as P};
use std::{collections::HashSet, vec};
type Position = P<usize>;

advent_of_code::solution!(16);

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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Beam(Movable<usize>);

struct Contraption {
    tiles: Matrix<Tile>,
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
        let movable = Movable::new((row, col).into(), direction);
        Self(movable)
    }

    fn position(&self) -> &Position {
        &self.0.position
    }

    fn direction(&self) -> &Direction {
        &self.0.direction
    }

    fn turn_to(mut self, direction: Direction) -> Self {
        self.0.turn_to(&direction);
        self
    }

    fn move_forward(mut self) -> Self {
        self.0.move_forward(1);
        self
    }

    fn move_in_mirror(self, mirror: &Mirror) -> Self {
        use Mirror::*;
        match (self.direction(), mirror) {
            (Left, BackSlash) | (Right, Slash) => self.turn_to(Up),
            (Left, Slash) | (Right, BackSlash) => self.turn_to(Down),
            (Up, BackSlash) | (Down, Slash) => self.turn_to(Left),
            (Up, Slash) | (Down, BackSlash) => self.turn_to(Right),
        }
        .move_forward()
    }

    fn move_in_splitter(self, splitter: &Splitter) -> Vec<Self> {
        use Splitter::*;
        let beams = match (self.direction(), splitter) {
            (Up, Horizontal) | (Down, Horizontal) => {
                vec![self.turn_to(Left), self.turn_to(Right)]
            }
            (Left, Vertical) | (Right, Vertical) => {
                vec![self.turn_to(Up), self.turn_to(Down)]
            }
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
        Self {
            tiles: Matrix::from(value),
        }
    }
}

impl Contraption {
    fn is_valid_beam(&self, beam: &Beam) -> bool {
        self.tiles.is_valid_position(beam.position())
    }

    fn move_beam(&self, beam: &Beam, visited: &mut HashSet<Beam>) -> Option<Vec<Beam>> {
        if !self.is_valid_beam(beam) {
            // the beam in out of the tiles.
            return None;
        }
        if visited.contains(beam) {
            // the beam visited this tile, cycle start.
            return None;
        }
        visited.insert(*beam);

        let tile = &self.tiles[*beam.position()];
        let beams = beam.move_in_tile(tile);

        Some(beams)
    }

    fn energizes(&self, beam: &Beam) -> u32 {
        let mut beams = vec![*beam];
        let mut visited = HashSet::new();

        while let Some(beam) = beams.pop() {
            if let Some(mut moved_beams) = self.move_beam(&beam, &mut visited) {
                beams.append(&mut moved_beams);
            }
        }

        let mut visited_positions = HashSet::new();
        visited.iter().for_each(|beam| {
            visited_positions.insert(beam.position());
        });

        visited_positions.len() as u32
    }

    fn get_all_edge_beams(&self) -> Vec<Beam> {
        let mut beams = vec![];

        (1..=self.tiles.rows).for_each(|row| {
            beams.push(Beam::new(row, 1, Right));
        });

        (1..=self.tiles.rows).for_each(|row| {
            beams.push(Beam::new(row, self.tiles.cols, Left));
        });

        (1..=self.tiles.cols).for_each(|col| {
            beams.push(Beam::new(1, col, Down));
        });

        (1..=self.tiles.cols).for_each(|col| {
            beams.push(Beam::new(self.tiles.rows, col, Up));
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
