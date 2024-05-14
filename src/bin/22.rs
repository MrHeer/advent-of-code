#![feature(binary_heap_into_iter_sorted)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    ops::RangeInclusive,
};

advent_of_code::solution!(22);

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y, z } = self;
        write!(f, "{},{},{}", x, y, z)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Brick {
    start: Coordinate,
    end: Coordinate,
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { start, end } = self;
        write!(f, "{}~{}", start, end)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.lowest().cmp(&other.lowest())
    }
}

struct SandSlabs {
    bricks: Vec<Brick>,
}

impl From<(usize, usize, usize)> for Coordinate {
    fn from(value: (usize, usize, usize)) -> Self {
        let (x, y, z) = value;
        Self { x, y, z }
    }
}

impl From<&str> for Coordinate {
    fn from(value: &str) -> Self {
        let mut iter = value.split(',').map(|x| x.parse().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Self::from((x, y, z))
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let mut iter = value.split('~').map(Coordinate::from);
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        Self { start, end }
    }
}

impl From<&str> for SandSlabs {
    fn from(value: &str) -> Self {
        let bricks = value.lines().map(Brick::from).collect();
        Self { bricks }
    }
}

struct FallResult {
    bricks: Vec<Brick>,
    support: HashMap<Brick, Vec<Brick>>,
    supported: HashMap<Brick, Vec<Brick>>,
}

#[derive(Eq, PartialEq, Clone)]
struct HeapState {
    brick: Brick,
}

impl From<Brick> for HeapState {
    fn from(value: Brick) -> Self {
        Self { brick: value }
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.brick.highest().cmp(&other.brick.highest())
    }
}

impl SandSlabs {
    fn fall(&self) -> FallResult {
        let mut bricks = self.bricks.clone();
        let mut settled = BinaryHeap::new();
        let mut support = HashMap::new();
        let mut supported = HashMap::new();
        bricks.sort();

        fn fall_to_settle(brick: &Brick, settled: &BinaryHeap<HeapState>) -> Brick {
            let mut new_settled_brick = None;
            for HeapState {
                brick: settled_brick,
            } in settled.clone().into_iter_sorted()
            {
                if brick.is_intersect_in_xy(&settled_brick) {
                    let diff = brick.lowest() - settled_brick.highest() - 1;
                    new_settled_brick = Some(brick.fall(diff));
                    break;
                }
            }
            new_settled_brick.unwrap_or(brick.fall(brick.lowest() - 1))
        }

        let mut update_dependencies = |brick: &Brick, settled: &BinaryHeap<HeapState>| {
            settled.iter().for_each(
                |HeapState {
                     brick: settled_brick,
                 }| {
                    if settled_brick.is_support(brick) {
                        support
                            .entry(settled_brick.clone())
                            .and_modify(|v: &mut Vec<Brick>| v.push(brick.clone()))
                            .or_insert(vec![brick.clone()]);
                        supported
                            .entry(brick.clone())
                            .and_modify(|v: &mut Vec<Brick>| v.push(settled_brick.clone()))
                            .or_insert(vec![settled_brick.clone()]);
                    }
                },
            );
        };

        bricks.iter().for_each(|brick| {
            let settled_brick = fall_to_settle(brick, &settled);
            update_dependencies(&settled_brick, &settled);
            settled.push(HeapState::from(settled_brick));
        });

        FallResult {
            bricks: settled.into_iter().map(|state| state.brick).collect(),
            support,
            supported,
        }
    }
}

impl Brick {
    fn fall(&self, diff: usize) -> Self {
        let mut brick = self.clone();
        brick.start.z -= diff;
        brick.end.z -= diff;
        brick
    }

    fn highest(&self) -> usize {
        self.start.z.max(self.end.z)
    }

    fn lowest(&self) -> usize {
        self.start.z.min(self.end.z)
    }

    fn is_support(&self, other: &Self) -> bool {
        self.highest() + 1 == other.lowest() && self.is_intersect_in_xy(other)
    }

    fn is_intersect_in_xy(&self, other: &Self) -> bool {
        is_overlap(
            create_range(self.start.x, self.end.x),
            create_range(other.start.x, other.end.x),
        ) && is_overlap(
            create_range(self.start.y, self.end.y),
            create_range(other.start.y, other.end.y),
        )
    }
}

fn create_range(start: usize, end: usize) -> RangeInclusive<usize> {
    let min = start.min(end);
    let max = start.max(end);
    min..=max
}

fn is_overlap(range: RangeInclusive<usize>, other: RangeInclusive<usize>) -> bool {
    let min1 = range.start();
    let max1 = range.end();
    let min2 = other.start();
    let max2 = other.end();
    min1.max(min2) <= max1.min(max2)
}

pub fn part_one(input: &str) -> Option<usize> {
    let FallResult {
        bricks,
        support,
        supported,
    } = SandSlabs::from(input).fall();
    let is_safe = |brick: &Brick| -> bool {
        support.get(brick).map_or(true, |supports| {
            supports
                .iter()
                .all(|brick| supported.get(brick).unwrap().len() >= 2)
        })
    };

    Some(bricks.iter().filter(|brick| is_safe(brick)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let FallResult {
        bricks,
        support,
        supported,
    } = SandSlabs::from(input).fall();

    let should_falling = |brick: &Brick, fallings: &HashSet<&Brick>| {
        supported
            .get(brick)
            .unwrap()
            .iter()
            .all(|brick| fallings.contains(brick))
    };

    let falling_count = |disintegrated: &Brick| -> usize {
        let mut fallings = HashSet::new();
        let mut disintegrated_queue = vec![disintegrated];
        while let Some(disintegrated) = disintegrated_queue.pop() {
            fallings.insert(disintegrated);
            if let Some(supports) = support.get(disintegrated) {
                supports
                    .iter()
                    .filter(|brick| should_falling(brick, &fallings))
                    .for_each(|disintegrated| disintegrated_queue.push(disintegrated));
            }
        }

        fallings.len() - 1
    };

    Some(bricks.iter().map(falling_count).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_intersect_in_xy() {
        let a = Brick::from("1,0,1~1,2,1");
        let b = Brick::from("0,0,2~2,0,2");
        let c = Brick::from("0,2,3~2,2,3");
        let d = Brick::from("0,0,4~0,2,4");
        let e = Brick::from("2,0,5~2,2,5");
        let f = Brick::from("0,1,6~2,1,6");
        let g = Brick::from("1,1,8~1,1,9");

        assert!(a.is_intersect_in_xy(&b));
        assert!(a.is_intersect_in_xy(&c));
        assert!(b.is_intersect_in_xy(&d));
        assert!(b.is_intersect_in_xy(&e));
        assert!(c.is_intersect_in_xy(&d));
        assert!(c.is_intersect_in_xy(&e));
        assert!(d.is_intersect_in_xy(&f));
        assert!(e.is_intersect_in_xy(&f));
        assert!(f.is_intersect_in_xy(&g));
    }
}
