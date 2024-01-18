use advent_of_code::{number_of_interiors, Direction, Movable, Position as P};
use Direction::*;

type Position = P<isize>;

advent_of_code::solution!(18);

struct Command {
    direction: Direction,
    steps: usize,
    color: String,
}

struct Digger(Movable<isize>);

struct BigPlan {
    digger: Digger,
    commands: Vec<Command>,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        let direction = parts
            .next()
            .map(|d| match d {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => panic!("Invalid direction"),
            })
            .unwrap();

        let steps = parts.next().unwrap().parse().unwrap();

        let color = parts.next().unwrap().replace(['(', ')'], "");

        Self {
            direction,
            steps,
            color,
        }
    }
}

impl Digger {
    fn move_to(&mut self, direction: &Direction, steps: usize) -> Vec<Position> {
        (0..steps)
            .map(|_| self.0.turn_to(direction).move_forward(1).position)
            .collect()
    }

    fn move_with_command(&mut self, command: &Command) -> Vec<Position> {
        self.move_to(&command.direction, command.steps)
    }
}

impl From<&str> for BigPlan {
    fn from(value: &str) -> Self {
        let commands = value.lines().map(Command::from).collect();
        Self {
            digger: Digger(Movable::new((1, 1).into(), Right)),
            commands,
        }
    }
}

impl BigPlan {
    fn start_move(&mut self) -> Vec<Position> {
        self.commands
            .iter()
            .flat_map(|command| self.digger.move_with_command(command))
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let circle = BigPlan::from(input).start_move();
    let interiors = number_of_interiors(&circle);
    Some((interiors + circle.len()) as u32)
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
