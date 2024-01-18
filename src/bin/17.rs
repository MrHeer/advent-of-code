use advent_of_code::{dijkstra::Bound, dijkstra_search, Direction, Matrix, Movable, Position as P};
use std::ops::Add;
use Direction::*;
type Position = P<usize>;

advent_of_code::solution!(17);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Crucible {
    movable: Movable<usize>,
    forward_times: usize,
    is_ultra: bool,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Cost(u32);

struct Solver {
    blocks: Matrix<Cost>,
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Cost {
    type Output = Cost;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Bound for Cost {
    fn min_value() -> Self {
        Self(u32::MIN)
    }

    fn max_value() -> Self {
        Self(u32::MAX)
    }
}

impl From<char> for Cost {
    fn from(value: char) -> Self {
        Self(value.to_digit(10).unwrap())
    }
}

impl Crucible {
    fn new(row: usize, col: usize, direction: Direction, is_ultra: bool) -> Self {
        let movable = Movable::new((row, col).into(), direction);
        Self {
            movable,
            forward_times: 0,
            is_ultra,
        }
    }

    fn create(position: &Position, direction: Direction) -> Self {
        let Position { row, col } = *position;
        Self::new(row, col, direction, false)
    }

    fn create_ultra(position: &Position, direction: Direction) -> Self {
        let Position { row, col } = *position;
        Self::new(row, col, direction, true)
    }

    fn position(&self) -> &Position {
        &self.movable.position
    }

    fn minimum_forward(&self) -> usize {
        match self.is_ultra {
            true => 4,
            false => 0,
        }
    }

    fn maximum_forward(&self) -> usize {
        match self.is_ultra {
            true => 10,
            false => 3,
        }
    }

    fn could_forward(&self) -> bool {
        self.forward_times < self.maximum_forward()
    }

    fn could_turnaround(&self) -> bool {
        self.forward_times >= self.minimum_forward()
    }

    fn move_forward(mut self) -> Option<Self> {
        if !self.could_forward() {
            return None;
        }
        self.movable.move_forward(1);
        self.forward_times += 1;
        Some(self)
    }

    fn turn_left(&mut self) -> &mut Self {
        self.movable.turn_left();
        self.forward_times = 0;
        self
    }

    fn turn_right(&mut self) -> &mut Self {
        self.movable.turn_right();
        self.forward_times = 0;
        self
    }

    fn move_left(mut self) -> Option<Self> {
        if !self.could_turnaround() {
            return None;
        }
        self.turn_left().move_forward()
    }

    fn move_right(mut self) -> Option<Self> {
        if !self.could_turnaround() {
            return None;
        }
        self.turn_right().move_forward()
    }

    fn moves(&self) -> Vec<Self> {
        [self.move_forward(), self.move_left(), self.move_right()]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl From<&str> for Solver {
    fn from(value: &str) -> Self {
        Self {
            blocks: Matrix::from(value),
        }
    }
}

impl Solver {
    fn is_valid(&self, crucible: &Crucible) -> bool {
        self.blocks.is_valid_position(crucible.position())
    }

    fn heat_loss(&self, crucible: &Crucible) -> Cost {
        self.blocks[*crucible.position()]
    }

    fn is_reach_goal(crucible: &Crucible, goal: &Position) -> bool {
        crucible.position() == goal
    }

    fn minimize_heat_loss(&self, start: &Position, goal: &Position, is_ultra: bool) -> Option<u32> {
        let starts: Vec<Crucible> = match is_ultra {
            true => vec![
                Crucible::create_ultra(start, Up),
                Crucible::create_ultra(start, Down),
                Crucible::create_ultra(start, Left),
                Crucible::create_ultra(start, Right),
            ],
            false => vec![
                Crucible::create(start, Up),
                Crucible::create(start, Down),
                Crucible::create(start, Left),
                Crucible::create(start, Right),
            ],
        };

        let is_reach_goal = |crucible: &Crucible| match is_ultra {
            true => crucible.could_turnaround() && Self::is_reach_goal(crucible, goal),
            false => Self::is_reach_goal(crucible, goal),
        };

        dijkstra_search(
            starts,
            is_reach_goal,
            |_, neighbor| self.heat_loss(neighbor),
            |node| {
                node.moves()
                    .into_iter()
                    .filter(|crucible| self.is_valid(crucible))
                    .collect()
            },
        )
        .map(|cost| cost.0)
    }
}

fn solve(input: &str, is_ultra: bool) -> Option<u32> {
    let solver = Solver::from(input);
    let blocks = &solver.blocks;
    let start = (1, 1).into();
    let goal = (blocks.rows, blocks.cols).into();
    solver.minimize_heat_loss(&start, &goal, is_ultra)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(94));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
