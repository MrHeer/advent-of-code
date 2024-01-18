use advent_of_code::{number_of_interiors, Direction, Movable, Position as P};
use Direction::*;

type Position = P<isize>;

advent_of_code::solution!(18);

struct Command {
    direction: Direction,
    steps: usize,
}

struct Digger(Movable<isize>);

struct BigPlan;

impl Command {
    fn parse_command(value: &str) -> Self {
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

        Self { direction, steps }
    }

    fn from_color(color: &str) -> Self {
        let mut hex: Vec<char> = color.chars().skip(1).collect();
        let direction = match hex.pop().unwrap() {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => panic!("Invalid direction"),
        };
        let steps = usize::from_str_radix(&String::from_iter(hex), 16).unwrap();
        Self { direction, steps }
    }

    fn parse_crazy_command(value: &str) -> Self {
        let color = value
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .replace(['(', ')'], "");
        Self::from_color(&color)
    }
}

impl Digger {
    fn new() -> Self {
        Self(Movable::new((1, 1).into(), Right))
    }

    fn move_to(&mut self, direction: &Direction, steps: usize) -> Vec<Position> {
        (0..steps)
            .map(|_| self.0.turn_to(direction).move_forward(1).position)
            .collect()
    }

    fn move_with_command(&mut self, command: &Command) -> Vec<Position> {
        self.move_to(&command.direction, command.steps)
    }

    fn start_move(&mut self, commands: &[Command]) -> Vec<Position> {
        commands
            .iter()
            .flat_map(|command| self.move_with_command(command))
            .collect()
    }
}

impl BigPlan {
    fn get_big_plan(value: &str) -> Vec<Command> {
        value.lines().map(Command::parse_command).collect()
    }

    fn get_crazy_plan(value: &str) -> Vec<Command> {
        value.lines().map(Command::parse_crazy_command).collect()
    }
}

fn solve(circle: &[Position]) -> Option<usize> {
    let interiors = number_of_interiors(circle);
    Some(interiors + circle.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    let circle = Digger::new().start_move(&BigPlan::get_big_plan(input));
    solve(&circle)
}

pub fn part_two(input: &str) -> Option<usize> {
    let circle = Digger::new().start_move(&BigPlan::get_crazy_plan(input));
    solve(&circle)
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
        assert_eq!(result, Some(952408144115));
    }
}
