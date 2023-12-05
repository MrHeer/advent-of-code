use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    row: u32,
    col: u32,
}

struct Number {
    value: u32,
    positions: Vec<Position>,
}

struct Engine {
    schematic: Vec<Vec<char>>,
    numbers: Vec<Number>,
    row: u32,
    col: u32,
}

impl Engine {
    pub fn new(schematic_text: &str) -> Engine {
        let (mut schematic, mut numbers, mut gears) = (vec![], vec![], vec![]);
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
                if c == '*' {
                    gears.push(Position { row, col })
                }
                if c.is_digit(10) {
                    number.push(c);
                    positions.push(Position { row, col })
                } else if number.is_empty() == false {
                    numbers.push(Engine::make_number_and_clear(&mut number, &mut positions));
                }
            });
            if number.is_empty() == false {
                numbers.push(Engine::make_number_and_clear(&mut number, &mut positions));
            }
            schematic.push(chars);
        });
        Engine {
            schematic,
            numbers,
            row,
            col,
        }
    }

    fn make_number_and_clear(number: &mut String, positions: &mut Vec<Position>) -> Number {
        let num = Number {
            value: number.parse().unwrap(),
            positions: positions.to_vec(),
        };
        number.clear();
        positions.clear();
        num
    }

    fn is_valid(&self, pos: &Position) -> bool {
        let &Position { row, col } = pos;
        row <= self.row && col <= self.col
    }

    fn assert_position(&self, pos: &Position) {
        assert!(self.is_valid(pos), "pos is not valid.");
    }

    fn get_char(&self, pos: &Position) -> char {
        self.assert_position(pos);
        let &Position { row, col } = pos;
        self.schematic[(row - 1) as usize][(col - 1) as usize]
    }

    fn is_symbol(&self, pos: &Position) -> bool {
        let c = self.get_char(pos);
        !(c.is_digit(10) || c == '.')
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

    fn get_adjacent_position(&self, pos: &Position) -> Vec<Position> {
        self.assert_position(pos);
        let &Position { row, col } = pos;
        let positions = match (row, col) {
            (1, 1) => vec![
                Position {
                    row: row,
                    col: col + 1,
                },
                Position {
                    row: row + 1,
                    col: col,
                },
                Position {
                    row: row + 1,
                    col: col + 1,
                },
            ],
            (1, col) => vec![
                Position {
                    row: row,
                    col: col - 1,
                },
                Position {
                    row: row,
                    col: col + 1,
                },
                Position {
                    row: row + 1,
                    col: col - 1,
                },
                Position {
                    row: row + 1,
                    col: col,
                },
                Position {
                    row: row + 1,
                    col: col + 1,
                },
            ],
            (row, 1) => vec![
                Position {
                    row: row - 1,
                    col: col,
                },
                Position {
                    row: row - 1,
                    col: col + 1,
                },
                Position {
                    row: row,
                    col: col + 1,
                },
                Position {
                    row: row + 1,
                    col: col,
                },
                Position {
                    row: row + 1,
                    col: col + 1,
                },
            ],
            _ => vec![
                Position {
                    row: row - 1,
                    col: col - 1,
                },
                Position {
                    row: row - 1,
                    col: col,
                },
                Position {
                    row: row - 1,
                    col: col + 1,
                },
                Position {
                    row: row,
                    col: col + 1,
                },
                Position {
                    row: row,
                    col: col - 1,
                },
                Position {
                    row: row + 1,
                    col: col - 1,
                },
                Position {
                    row: row + 1,
                    col: col,
                },
                Position {
                    row: row + 1,
                    col: col + 1,
                },
            ],
        };
        positions
            .into_iter()
            .filter(|pos| self.is_valid(pos))
            .collect()
    }

    fn get_number_adjacent_position(&self, number: &Number) -> Vec<Position> {
        let mut adjacent_positions = HashSet::new();

        number
            .positions
            .iter()
            .flat_map(|pos| self.get_adjacent_position(pos))
            .for_each(|adjacent_pos| {
                adjacent_positions.insert(adjacent_pos.clone());
            });

        Vec::from_iter(adjacent_positions.into_iter())
    }

    pub fn part_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|&num| self.is_part_number(num))
            .map(|num| num.value)
            .collect()
    }

    pub fn get_gear_ratio(&self) -> Option<u32> {
        let mut star_map = HashMap::new();
        self.numbers.iter().for_each(|number| {
            self.get_number_adjacent_position(number)
                .iter()
                .filter(|&adjacent_pos| self.is_star(adjacent_pos))
                .for_each(|&star_pos| {
                    let star_numbers = star_map.entry(star_pos).or_insert(vec![]);
                    star_numbers.push(number.value);
                });
        });

        let gear_iter = star_map
            .keys()
            .filter(|&star_pos| star_map.get(star_pos).unwrap().len() == 2);

        gear_iter
            .flat_map(|gear_pos| {
                star_map
                    .get(gear_pos)
                    .unwrap()
                    .to_owned()
                    .into_iter()
                    .reduce(|gear_ratio, value| gear_ratio * value)
            })
            .reduce(|sum, gear_ratio| sum + gear_ratio)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Engine::new(input)
        .part_numbers()
        .into_iter()
        .reduce(|sum, value| sum + value)
}

pub fn part_two(input: &str) -> Option<u32> {
    Engine::new(input).get_gear_ratio()
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
