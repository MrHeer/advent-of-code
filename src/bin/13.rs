use advent_of_code::Matrix;

advent_of_code::solution!(13);

#[derive(Clone, Copy, PartialEq)]
enum Pattern {
    Ash,
    Rocks,
}

struct Patterns {
    patterns: Matrix<Pattern>,
}

use Pattern::*;

impl From<char> for Pattern {
    fn from(value: char) -> Self {
        match value {
            '.' => Ash,
            '#' => Rocks,
            _ => panic!("Could not resolve pattern."),
        }
    }
}

impl From<&str> for Patterns {
    fn from(value: &str) -> Self {
        Self {
            patterns: Matrix::from(value),
        }
    }
}

fn hamming_distance<T>(a: &[T], b: &[T]) -> usize
where
    T: PartialEq,
{
    if a.len() != b.len() {
        panic!("patterns must be of equal length.");
    }
    a.iter().zip(b.iter()).filter(|(a, b)| a != b).count()
}

impl Patterns {
    fn len(&self) -> usize {
        self.patterns.rows
    }

    fn get(&self, index: usize) -> Vec<Pattern> {
        self.patterns.get_row(index).clone()
    }

    fn distance(&self, index: usize) -> usize {
        let mut distance = 0;
        let max_matches = (self.len() - (index + 1)).min(index + 1);
        (0..max_matches).for_each(|shift| {
            distance += hamming_distance(&self.get(index - shift), &self.get(index + shift + 1));
        });
        distance
    }

    fn get_reflection_position(&self, distance: usize) -> Option<u32> {
        (0..self.len() - 1)
            .find(|index| self.distance(*index) == distance)
            .map(|x| x as u32 + 1)
    }

    fn transpose(&self) -> Self {
        Self {
            patterns: self.patterns.transpose(),
        }
    }

    fn summarize(&self, distance: usize) -> u32 {
        self.get_reflection_position(distance).unwrap_or_default() * 100
            + self
                .transpose()
                .get_reflection_position(distance)
                .unwrap_or_default()
    }
}

fn solve(input: &str, distance: usize) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(Patterns::from)
            .map(|p| p.summarize(distance))
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 0)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
