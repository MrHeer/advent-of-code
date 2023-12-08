use std::ops::Range;

advent_of_code::solution!(6);

struct Race {
    time: u64,
    distance: u64,
}

const TOLERANCE: f64 = 1e-10;

impl Race {
    fn win(&self) -> Option<Range<u64>> {
        let time = self.time as f64;
        let distance = self.distance as f64;
        let delta = time.powi(2) - 4. * distance;
        if delta > TOLERANCE {
            let start = ((time - delta.sqrt()) / 2. + 0.5).round() as u64;
            let end = ((time + delta.sqrt()) / 2. - 0.6).round() as u64 + 1;
            let range = Range { start, end };
            return Some(range);
        }
        None
    }
}

type Game = Vec<Race>;

fn parse_line(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
}

fn parse_game(game_text: &str) -> Game {
    let mut lines = game_text.lines();
    let time_iter = parse_line(lines.next().unwrap());
    let distance_iter = parse_line(lines.next().unwrap());
    time_iter
        .into_iter()
        .zip(distance_iter)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_game(input)
        .iter()
        .map(|race| race.win().and_then(|range| Some(range.count() as u64)))
        .product()
}

fn parse_number(line: &str) -> u64 {
    line.split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time = parse_number(lines.next().unwrap());
    let distance = parse_number(lines.next().unwrap());
    Race { time, distance }
        .win()
        .map(|range| range.count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
