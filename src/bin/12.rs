use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(12);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

use Condition::*;

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("Could not resolve condition"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Record {
    conditions: Vec<Condition>,
    damaged_account: Vec<usize>,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();

        let conditions = parts.next().unwrap().chars().map(Condition::from).collect();
        let damaged_account = parts
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        Record::new(conditions, damaged_account)
    }
}

impl From<String> for Record {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl Record {
    fn new(conditions: Vec<Condition>, damaged_account: Vec<usize>) -> Self {
        Self {
            conditions,
            damaged_account,
        }
    }

    fn possible_arrangements(&self, cache: &mut HashMap<Record, usize>) -> usize {
        if let Some(&count) = cache.get(self) {
            return count;
        }

        if self.damaged_account.is_empty() {
            let count = match self.conditions.iter().any(|&c| c == Damaged) {
                true => 0,
                false => 1,
            };

            cache.insert(self.clone(), count);
            return count;
        }

        let needed_space =
            self.damaged_account.iter().sum::<usize>() + self.damaged_account.len() - 1;
        if self.conditions.len() < needed_space {
            cache.insert(self.clone(), 0);
            return 0;
        }

        let first = self.conditions[0];
        if first == Operational {
            let count = Self::new(self.conditions[1..].to_vec(), self.damaged_account.clone())
                .possible_arrangements(cache);
            cache.insert(self.clone(), count);
            return count;
        }

        let damaged = self.damaged_account[0] as usize;
        let are_all_non_operational = self.conditions[..damaged].iter().all(|c| *c != Operational);
        let end = (damaged + 1).min(self.conditions.len());

        let mut count = 0;

        if are_all_non_operational
            && ((self.conditions.len() > damaged && self.conditions[damaged] != Damaged)
                || self.conditions.len() <= damaged)
        {
            count += Self::new(
                self.conditions[end..].to_vec(),
                self.damaged_account[1..].to_vec(),
            )
            .possible_arrangements(cache);
        }

        if first == Unknown {
            count += Self::new(self.conditions[1..].to_vec(), self.damaged_account.clone())
                .possible_arrangements(cache);
        }

        cache.insert(self.clone(), count);

        count
    }
}

fn solve(records: Vec<Record>) -> Option<usize> {
    let mut cache = HashMap::new();

    Some(
        records
            .iter()
            .map(|c| c.possible_arrangements(&mut cache))
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let records = input.lines().map(Record::from).collect();
    solve(records)
}

fn unfold(input: &str) -> String {
    let mut parts = input.split_whitespace();

    let conditions = parts.next().unwrap();
    let conditions = (0..5).map(|_| conditions).join("?");

    let damaged_account = parts.next().unwrap();
    let damaged_account = (0..5).map(|_| damaged_account).join(",");

    format!("{} {}", conditions, damaged_account)
}

pub fn part_two(input: &str) -> Option<usize> {
    let records = input.lines().map(unfold).map(Record::from).collect();
    solve(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
