use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

struct Card {
    id: u32,
    win_set: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(card_line: &str) -> Self {
        let mut iter = card_line.split(&[':', '|'][..]).map(|x| x.trim());

        let id = iter
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let mut next_numbers = || {
            iter.next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        };

        let win_numbers = next_numbers();
        let numbers = next_numbers();

        let win_set = HashSet::from_iter(win_numbers.into_iter());

        Self {
            id,
            win_set,
            numbers,
        }
    }

    fn get_win_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|&x| self.win_set.contains(x))
            .count() as u32
    }

    fn get_points(&self) -> u32 {
        match self.get_win_count() {
            0 => 0,
            count => 2_u32.pow((count - 1) as u32),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Card::new)
            .map(|card| card.get_points())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input.lines().map(Card::new).collect();
    let mut instances: HashMap<u32, u32> =
        HashMap::from_iter(cards.iter().map(|card| (card.id, 1)));

    cards.iter().for_each(|card| {
        let count = card.get_win_count();
        (1..count + 1).for_each(|i| {
            let copies = instances[&card.id];
            if let Some(counter) = instances.get_mut(&(card.id + i)) {
                *counter += copies;
            }
        });
    });

    Some(instances.iter().map(|(_, copies)| copies).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
