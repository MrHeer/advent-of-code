use std::collections::{HashMap, VecDeque};

use advent_of_code::{Matrix, Position as P};

advent_of_code::solution!(21);

type Position = P<isize>;

fn cycle_to_range(num: isize, min: isize, max: isize) -> isize {
    let size = max - min + 1;
    if num < min {
        max - (min - num - 1) % size
    } else if num > max {
        min + (num - max - 1) % size
    } else {
        num
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Start,
    Plot,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            'S' => Start,
            '.' => Plot,
            '#' => Rock,
            _ => panic!("Invalid tile: {}", value),
        }
    }
}

struct Garden {
    grid: Matrix<Tile>,
}

impl From<&str> for Garden {
    fn from(map: &str) -> Self {
        Self {
            grid: Matrix::from(map),
        }
    }
}

impl Garden {
    fn find_start(&self) -> Option<Position> {
        for row in 1..=self.grid.rows {
            for col in 1..=self.grid.cols {
                let pos = (row, col).into();
                if self.grid[pos] == Tile::Start {
                    return Some((row as isize, col as isize).into());
                }
            }
        }
        None
    }

    fn get_index(&self, pos: &Position) -> P<usize> {
        P {
            row: cycle_to_range(pos.row, 1, self.grid.rows as isize) as usize,
            col: cycle_to_range(pos.col, 1, self.grid.cols as isize) as usize,
        }
    }

    fn get_tile(&self, pos: &Position) -> Tile {
        self.grid[self.get_index(pos)]
    }

    fn get_distance_map(&self, steps: usize) -> HashMap<Position, usize> {
        let mut frontier = VecDeque::<(Position, usize)>::new();
        let mut distance_map = HashMap::new();
        frontier.push_back((self.find_start().unwrap(), 0));

        while let Some((pos, distance)) = frontier.pop_front() {
            if distance_map.contains_key(&pos) {
                continue;
            }

            distance_map.insert(pos, distance);

            pos.adjacent_positions()
                .into_iter()
                .filter(|next_pos| {
                    !distance_map.contains_key(next_pos) && self.get_tile(next_pos) != Tile::Rock
                })
                .for_each(|next_pos| {
                    if distance < steps {
                        frontier.push_back((next_pos, distance + 1));
                    }
                });
        }

        distance_map
    }

    fn marked_count(steps: usize, distance_map: &HashMap<Position, usize>) -> usize {
        distance_map
            .values()
            .filter(|&&v| v <= steps && v % 2 == steps % 2)
            .count()
    }

    fn steps(&self) -> usize {
        let steps = 64;
        let map = self.get_distance_map(steps);
        Garden::marked_count(steps, &map)
    }

    // The garden plots and rocks are set up so that the map repeats infinitely
    // in every direction.
    fn crazy_steps(&self) -> usize {
        let map = self.get_distance_map(328);

        // Exploiting some nice properties of the input it reduces to quadratic
        // interpolation over 3 points: k * 131 + 65 for k = 0, 1, 2
        // https://en.wikipedia.org/wiki/Newton_polynomial
        let (x0, y0) = (65_f64, Garden::marked_count(65, &map) as f64);
        let (x1, y1) = (196_f64, Garden::marked_count(196, &map) as f64);
        let (x2, y2) = (327_f64, Garden::marked_count(327, &map) as f64);

        let a0 = y0;
        let a1 = (y1 - y0) / (x1 - x0);
        let a2 = (y2 - y1) / ((x2 - x1) * (x2 - x0)) - a1 / (x2 - x0);

        let newton_polynomial = |x: f64| -> f64 { a0 + a1 * (x - x0) + a2 * (x - x0) * (x - x1) };

        newton_polynomial(26501365_f64) as usize
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Garden::from(input).steps())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Garden::from(input).crazy_steps())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3591));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(598044246091826));
    }

    #[test]
    fn test_cycle_to_range() {
        assert_eq!(cycle_to_range(-4, -1, 1), -1);
        assert_eq!(cycle_to_range(-3, -1, 1), 0);
        assert_eq!(cycle_to_range(-2, -1, 1), 1);
        assert_eq!(cycle_to_range(-1, -1, 1), -1);
        assert_eq!(cycle_to_range(0, -1, 1), 0);
        assert_eq!(cycle_to_range(1, -1, 1), 1);
        assert_eq!(cycle_to_range(2, -1, 1), -1);
        assert_eq!(cycle_to_range(3, -1, 1), 0);
        assert_eq!(cycle_to_range(4, -1, 1), 1);
    }
}
