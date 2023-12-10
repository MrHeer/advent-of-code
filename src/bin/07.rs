use std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
    slice::Iter,
};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, Clone, Copy)]
struct Hand([char; 5]);

impl Index<usize> for Hand {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Hand {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Hand {
    fn iter(&self) -> Iter<'_, char> {
        self.0.iter()
    }

    fn map<F>(&self, f: F) -> Hand
    where
        F: FnMut(char) -> char,
    {
        Hand(self.0.map(f))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Eq)]
struct CamelCard {
    hand: Hand,
    hand_type: HandType,
    map: [(u32, char); 5],
    bid: u32,
}

const JOKER: char = 'J';
const MAX_LABEL: char = 'A';
const MIN_LABEL: char = 'X';
const EMPTY: char = ' ';

fn label_to_rank(label: &char) -> Option<u32> {
    match *label {
        MAX_LABEL => Some(14),
        'K' => Some(13),
        'Q' => Some(12),
        JOKER => Some(11),
        'T' => Some(10),
        '9' => Some(9),
        '8' => Some(8),
        '7' => Some(7),
        '6' => Some(6),
        '5' => Some(5),
        '4' => Some(4),
        '3' => Some(3),
        '2' => Some(2),
        MIN_LABEL => Some(1),
        _ => None,
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for index in 0..5 {
            let self_rank = label_to_rank(&self.0[index]).unwrap();
            let other_rank = label_to_rank(&other.0[index]).unwrap();
            match self_rank.partial_cmp(&other_rank) {
                Some(Ordering::Greater) => return Some(Ordering::Greater),
                Some(Ordering::Less) => return Some(Ordering::Less),
                _ => continue,
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        for index in 0..5 {
            let self_rank = label_to_rank(&self.0[index]).unwrap();
            let other_rank = label_to_rank(&other.0[index]).unwrap();
            match self_rank.cmp(&other_rank) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => continue,
            }
        }
        Ordering::Equal
    }
}

impl CamelCard {
    fn new(camel_card: &str) -> Self {
        let mut iter = camel_card.split_ascii_whitespace();
        let mut hand = Hand([EMPTY; 5]);
        iter.next()
            .unwrap()
            .chars()
            .enumerate()
            .for_each(|(i, c)| hand[i] = c);
        let bid = iter.next().unwrap().parse().unwrap();
        Self::from(hand, bid)
    }

    fn from(hand: Hand, bid: u32) -> Self {
        let map = Self::get_count_map(&hand);
        let hand_type = Self::hand_type(&map);
        Self {
            hand,
            hand_type,
            map,
            bid,
        }
    }

    fn get_count_map(hand: &Hand) -> [(u32, char); 5] {
        let mut map = [(0, EMPTY); 5];
        hand.iter()
            .counts()
            .iter()
            .map(|(&ch, count)| (*count as u32, *ch))
            .sorted_by(|a, b| match a.0.cmp(&b.0) {
                Ordering::Equal => label_to_rank(&a.1).cmp(&label_to_rank(&b.1)),
                order => order,
            })
            .rev()
            .enumerate()
            .for_each(|(i, pair)| map[i] = pair);
        map
    }

    fn hand_type(map: &[(u32, char); 5]) -> HandType {
        let mut count_arr = [0, 0, 0, 0, 0];
        map.iter()
            .map(|pair| pair.0)
            .enumerate()
            .for_each(|(i, count)| count_arr[i] = count as u32);
        match count_arr {
            [5, 0, 0, 0, 0] => HandType::FiveKind,
            [4, 1, 0, 0, 0] => HandType::FourKind,
            [3, 2, 0, 0, 0] => HandType::FullHouse,
            [3, 1, 1, 0, 0] => HandType::ThreeKind,
            [2, 2, 1, 0, 0] => HandType::TwoPair,
            [2, 1, 1, 1, 0] => HandType::OnePair,
            _ => HandType::High,
        }
    }

    fn map_x_to_y(hand: &Hand, x: char, y: char) -> Hand {
        hand.map(|c| if c == x { y } else { c })
    }

    fn strongest(&self) -> Self {
        let Self {
            hand,
            hand_type: _,
            map,
            bid,
        } = *self;

        let map_to_char = map
            .iter()
            .map(|(_, c)| *c)
            .filter(|&c| c != 'J' && c != EMPTY)
            .next()
            .unwrap_or(MAX_LABEL);
        let strongest_hand = Self::map_x_to_y(&hand, JOKER, map_to_char);
        let hand_type = Self::from(strongest_hand, bid).hand_type;

        Self {
            hand: Self::map_x_to_y(&hand, JOKER, MIN_LABEL),
            hand_type,
            map,
            bid: self.bid,
        }
    }
}

impl PartialEq for CamelCard {
    fn eq(&self, other: &Self) -> bool {
        match self.hand_type.eq(&other.hand_type) {
            true => true,
            false => self.hand.eq(&other.hand),
        }
    }
}

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => self.hand.partial_cmp(&other.hand),
            order => order,
        }
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.hand.cmp(&other.hand),
            other => other,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(CamelCard::new)
            .sorted()
            .enumerate()
            .map(|(rank, card)| (rank as u32 + 1) * card.bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(CamelCard::new)
            .map(|card| card.strongest())
            .sorted()
            .enumerate()
            .map(|(rank, card)| (rank as u32 + 1) * card.bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
