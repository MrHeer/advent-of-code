use num::integer::lcm;
use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Hash, PartialEq, Eq, Debug)]
struct Node([char; 3]);

const START: Node = Node(['A', 'A', 'A']);
const END: Node = Node(['Z', 'Z', 'Z']);

struct Pair {
    left: Node,
    right: Node,
}

enum Instruction {
    L,
    R,
}

use Instruction::{L, R};

struct Navigate {
    instructions: Vec<Instruction>,
    map: HashMap<Node, Pair>,
}

impl Navigate {
    fn new(navigate: &str) -> Self {
        let mut iter = navigate.split("\n\n");
        let instructions = Self::parse_instructions(iter.next().unwrap());
        let map = Self::parse_map(iter.next().unwrap());
        Self { instructions, map }
    }

    fn parse_instructions(instructions: &str) -> Vec<Instruction> {
        instructions
            .chars()
            .map(|ch| match ch {
                'L' => Some(L),
                'R' => Some(R),
                _ => None,
            })
            .map(|instruction| instruction.unwrap())
            .collect()
    }

    fn parse_map(map_text: &str) -> HashMap<Node, Pair> {
        let mut map = HashMap::new();
        map_text.lines().for_each(|info| {
            let mut iter = info.split(" = ");
            let node = Self::parse_node(iter.next().unwrap());
            let pair = Self::parse_pair(iter.next().unwrap());
            map.insert(node, pair);
        });
        map
    }

    fn parse_node(node: &str) -> Node {
        let mut chars = node.chars();
        Node([
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        ])
    }

    fn parse_pair(pair: &str) -> Pair {
        let mut iter = pair.get(1..9).unwrap().split(", ");
        Pair {
            left: Self::parse_node(iter.next().unwrap()),
            right: Self::parse_node(iter.next().unwrap()),
        }
    }

    fn next_node(&self, current: &Node, instruction: &Instruction) -> &Node {
        match instruction {
            L => &self.map[current].left,
            R => &self.map[current].right,
        }
    }

    fn navigate_to_end<F>(&self, node: &Node, is_end: F) -> u64
    where
        F: Fn(&Node) -> bool,
    {
        let mut step = 0;
        let mut current = node;

        for instruction in self.instructions.iter().cycle() {
            step += 1;
            current = self.next_node(current, instruction);
            if is_end(current) {
                return step;
            }
        }

        step
    }

    fn get_end_char(node: &Node) -> &char {
        &node.0[2]
    }

    fn is_start(node: &Node) -> bool {
        *Self::get_end_char(node) == 'A'
    }

    fn is_end(node: &Node) -> bool {
        *Self::get_end_char(node) == 'Z'
    }

    fn get_all_start_nodes(&self) -> Vec<&Node> {
        self.map
            .keys()
            .filter(|node| Self::is_start(node))
            .collect()
    }

    fn navigate_all_to_end_by_ghost(&self) -> u64 {
        self.get_all_start_nodes()
            .iter()
            .map(|n| self.navigate_to_end(n, Self::is_end))
            .fold(1, |steps, step| lcm(steps, step))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(Navigate::new(input).navigate_to_end(&START, |node| *node == END))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(Navigate::new(input).navigate_all_to_end_by_ghost())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
