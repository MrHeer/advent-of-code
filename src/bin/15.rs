#![feature(ascii_char)]

use itertools::Itertools;
use std::hash::{Hash, Hasher};

advent_of_code::solution!(15);

struct StringHasher(usize);

#[derive(Clone, PartialEq, Eq)]
struct CustomizeString(String);

#[derive(Clone)]
struct Lens {
    label: CustomizeString,
    focal_length: usize,
}

#[derive(Clone)]
struct Box {
    lens_slots: Vec<Lens>,
}

struct LensMapIterItem<'a> {
    lens: &'a Lens,
    box_index: usize,
    slot_index: usize,
}

struct LensMapIter<'a> {
    boxes: &'a Vec<Box>,
    box_index: usize,
    slot_index: usize,
}

struct LensMap {
    boxes: Vec<Box>,
}

enum Operation {
    Insert(Lens),
    Remove(CustomizeString),
}

use Operation::*;

impl Hasher for StringHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        bytes
            .iter()
            .for_each(|byte| self.0 = (self.0 + *byte as usize) * 17 % 256);
    }
}

impl StringHasher {
    fn new() -> Self {
        StringHasher(0)
    }
}

impl Hash for CustomizeString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0
            .chars()
            .for_each(|ch| state.write(&[ch.as_ascii().unwrap().to_u8()]))
    }
}

impl From<&str> for CustomizeString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl CustomizeString {
    fn calc_hash(&self) -> usize {
        let mut hasher = StringHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if value.contains('=') {
            let mut parts = value.split('=');
            let label = CustomizeString::from(parts.next().unwrap());
            let focal_length = parts.next().unwrap().parse().unwrap();
            let lens = Lens {
                label,
                focal_length,
            };
            return Insert(lens);
        } else if value.contains('-') {
            let mut parts = value.split('-');
            let label = CustomizeString::from(parts.next().unwrap());
            return Remove(label);
        } else {
            panic!("Could not resolve operation.");
        }
    }
}

impl Lens {
    fn calc_hash(&self) -> usize {
        self.label.calc_hash()
    }
}

impl Box {
    fn new() -> Self {
        Self { lens_slots: vec![] }
    }
}

impl<'a> LensMapIter<'a> {
    fn new(map: &'a LensMap) -> Self {
        Self {
            boxes: &map.boxes,
            box_index: 0,
            slot_index: 0,
        }
    }
}

impl<'a> Iterator for LensMapIter<'a> {
    type Item = LensMapIterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.box_index > 255 {
                return None;
            }
            let lens_slots = &self.boxes[self.box_index].lens_slots;
            if self.slot_index >= lens_slots.len() {
                self.box_index += 1;
                self.slot_index = 0;
                continue;
            }
            let lens = &lens_slots[self.slot_index];
            let item = Self::Item {
                lens,
                box_index: self.box_index,
                slot_index: self.slot_index,
            };
            self.slot_index += 1;
            return Some(item);
        }
    }
}

impl LensMap {
    fn new() -> Self {
        Self {
            boxes: vec![Box::new(); 256],
        }
    }

    fn insert(&mut self, lens: Lens) {
        let box_index = lens.calc_hash();
        let lens_slots = &mut self.boxes[box_index].lens_slots;
        let slot_position = lens_slots
            .iter()
            .find_position(|inner_lens| inner_lens.label == lens.label);
        if let Some((slot_index, _)) = slot_position {
            lens_slots[slot_index] = lens;
        } else {
            lens_slots.push(lens);
        }
    }

    fn remove(&mut self, label: CustomizeString) {
        let box_index = label.calc_hash();
        let lens_slots = &mut self.boxes[box_index].lens_slots;
        let slot_position = lens_slots
            .iter()
            .find_position(|inner_lens| inner_lens.label == label);
        if let Some((slot_index, _)) = slot_position {
            lens_slots.remove(slot_index);
        }
    }

    fn apply_operation(&mut self, operation: Operation) {
        match operation {
            Insert(lens) => self.insert(lens),
            Remove(label) => self.remove(label),
        }
    }

    fn iter(&self) -> LensMapIter {
        LensMapIter::new(self)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(CustomizeString::from)
            .map(|s| s.calc_hash())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = LensMap::new();
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(Operation::from)
        .for_each(|op| map.apply_operation(op));
    Some(
        map.iter()
            .map(|item| (item.box_index + 1) * (item.slot_index + 1) * item.lens.focal_length)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
