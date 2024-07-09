use advent_of_code::{Matrix, Position as P};
use std::collections::{HashMap, HashSet};
type Position = P<usize>;

advent_of_code::solution!(3);

struct Number {
    value: u32,
    positions: Vec<Position>,
}

struct Engine {
    schematic: Matrix<char>,
    numbers: Vec<Number>,
}

impl Engine {
    fn new(schematic_text: &str) -> Self {
        let (mut schematic, mut numbers) = (vec![], vec![]);
        let (mut row, mut col) = (0, 0);
        schematic_text.lines().for_each(|line| {
            row += 1;
            col = 0;
            let mut chars = vec![];
            let mut number = String::new();
            let mut positions = vec![];
            line.chars().for_each(|c| {
                col += 1;
                chars.push(c);

                if c.is_ascii_digit() {
                    number.push(c);
                    positions.push((row, col).into())
                } else if !number.is_empty() {
                    numbers.push(Self::make_number_and_clear(&mut number, &mut positions));
                }
            });
            if !number.is_empty() {
                numbers.push(Self::make_number_and_clear(&mut number, &mut positions));
            }
            schematic.push(chars);
        });
        Self {
            schematic: schematic.into(),
            numbers,
        }
    }

    fn make_number_and_clear(number: &mut String, positions: &mut Vec<Position>) -> Number {
        let num = Number {
            value: number.parse().unwrap(),
            positions: std::mem::take(positions),
        };
        number.clear();
        num
    }

    fn get_char(&self, pos: &Position) -> char {
        self.schematic[*pos]
    }

    fn is_symbol(&self, pos: &Position) -> bool {
        let c = self.get_char(pos);
        !(c.is_ascii_digit() || c == '.')
    }

    fn is_star(&self, pos: &Position) -> bool {
        let c = self.get_char(pos);
        c == '*'
    }

    fn is_part_number(&self, number: &Number) -> bool {
        for adjacent_pos in self.get_number_adjacent_position(number) {
            if self.is_symbol(&adjacent_pos) {
                return true;
            }
        }

        false
    }

    fn get_adjacent_positions(&self, pos: &Position) -> Vec<Position> {
        let Position { row, col } = *pos;
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .map(|(diff_row, diff_col)| (row as i32 + diff_row, col as i32 + diff_col))
        .map(|(row, col)| (row as usize, col as usize).into())
        .into_iter()
        .filter(|pos| self.schematic.is_valid_position(pos))
        .collect()
    }

    fn get_number_adjacent_position(&self, number: &Number) -> Vec<Position> {
        let mut adjacent_positions = HashSet::new();

        number
            .positions
            .iter()
            .flat_map(|pos| self.get_adjacent_positions(pos))
            .for_each(|adjacent_pos| {
                adjacent_positions.insert(adjacent_pos);
            });

        Vec::from_iter(adjacent_positions)
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|num| self.is_part_number(num))
            .map(|num| num.value)
            .collect()
    }

    fn get_star_map(&self) -> HashMap<Position, Vec<u32>> {
        let mut star_map: HashMap<P<usize>, Vec<u32>> = HashMap::new();

        self.numbers.iter().for_each(|number| {
            self.get_number_adjacent_position(number)
                .into_iter()
                .filter(|adjacent_pos| self.is_star(adjacent_pos))
                .for_each(|star_pos| {
                    star_map.entry(star_pos).or_default().push(number.value);
                });
        });

        star_map
    }

    fn get_gear_ratios(&self) -> Vec<u32> {
        let star_map = self.get_star_map();

        let gear_iter = star_map
            .keys()
            .filter(|star_pos| star_map.get(star_pos).unwrap().len() == 2);

        let get_radio = |gear_pos| star_map.get(gear_pos).unwrap().iter().product();

        gear_iter.map(get_radio).collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(Engine::new(input).get_part_numbers().iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(Engine::new(input).get_gear_ratios().iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
