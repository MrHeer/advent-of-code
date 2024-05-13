#![feature(binary_heap_into_iter_sorted)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
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
                if brick.is_intersect(&settled_brick) {
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
        self.highest() + 1 == other.lowest() && self.is_intersect(other)
    }

    fn is_intersect(&self, other: &Self) -> bool {
        let segment1 = create_segment(
            (self.start.x, self.start.y).into(),
            (self.end.x, self.end.y).into(),
        );
        let segment2 = create_segment(
            (other.start.x, other.start.y).into(),
            (other.end.x, other.end.y).into(),
        );

        match (segment1, segment2) {
            (Ok(segment), Ok(other_segment)) => segment.is_intersect(&other_segment),
            (Ok(segment), Err(point)) => segment.is_contains(&point),
            (Err(point), Ok(segment)) => segment.is_contains(&point),
            (Err(point), Err(other_point)) => point == other_point,
        }
    }
}

#[derive(PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

struct Segment {
    start: Point,
    end: Point,
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        let (x, y) = value;
        Self { x, y }
    }
}

fn create_segment(start: Point, end: Point) -> Result<Segment, Point> {
    if start == end {
        Err(start)
    } else {
        Ok(Segment { start, end })
    }
}

enum Orientation {
    Horizontal,
    Vertical,
}

fn create_range(start: usize, end: usize) -> RangeInclusive<usize> {
    let min = start.min(end);
    let max = start.max(end);
    min..=max
}

fn is_in_range(value: usize, start: usize, end: usize) -> bool {
    create_range(start, end).contains(&value)
}

fn is_overlap(range: RangeInclusive<usize>, other: RangeInclusive<usize>) -> bool {
    let min1 = range.start();
    let max1 = range.end();
    let min2 = other.start();
    let max2 = other.end();
    min1.max(min2) <= max1.min(max2)
}

use Orientation::*;

impl Segment {
    fn is_intersect(&self, other: &Self) -> bool {
        match (self.orientation().unwrap(), other.orientation().unwrap()) {
            (Horizontal, Horizontal) => {
                if self.start.y != other.start.y {
                    false
                } else {
                    is_overlap(
                        create_range(self.start.x, self.end.x),
                        create_range(other.start.x, other.end.x),
                    )
                }
            }
            (Vertical, Vertical) => {
                if self.start.x != other.start.x {
                    false
                } else {
                    is_overlap(
                        create_range(self.start.y, self.end.y),
                        create_range(other.start.y, other.end.y),
                    )
                }
            }
            (Vertical, Horizontal) => {
                is_in_range(self.start.x, other.start.x, other.end.x)
                    && is_in_range(other.start.y, self.start.y, self.end.y)
            }
            (Horizontal, Vertical) => {
                is_in_range(other.start.x, self.start.x, self.end.x)
                    && is_in_range(self.start.y, other.start.y, other.end.y)
            }
        }
    }

    fn is_contains(&self, point: &Point) -> bool {
        let Self { start, end } = self;
        match self.orientation().unwrap() {
            Horizontal => {
                if point.y != start.y {
                    false
                } else {
                    is_in_range(point.x, start.x, end.x)
                }
            }
            Vertical => {
                if point.x != start.x {
                    false
                } else {
                    is_in_range(point.y, start.y, end.y)
                }
            }
        }
    }

    fn orientation(&self) -> Option<Orientation> {
        let Self { start, end } = self;
        if start.x == end.x {
            return Some(Vertical);
        } else if start.y == end.y {
            return Some(Horizontal);
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let FallResult {
        bricks,
        support,
        supported,
    } = SandSlabs::from(input).fall();
    let is_safe = |brick: &Brick| -> bool {
        support.get(brick).map_or(true, |bricks| {
            bricks
                .iter()
                .all(|brick| supported.get(brick).unwrap().len() >= 2)
        })
    };

    Some(bricks.iter().filter(|brick| is_safe(brick)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
    fn test_intersect() {
        let a = Brick::from("1,0,1~1,2,1");
        let b = Brick::from("0,0,2~2,0,2");
        let c = Brick::from("0,2,3~2,2,3");
        let d = Brick::from("0,0,4~0,2,4");
        let e = Brick::from("2,0,5~2,2,5");
        let f = Brick::from("0,1,6~2,1,6");
        let g = Brick::from("1,1,8~1,1,9");

        assert!(a.is_intersect(&b));
        assert!(a.is_intersect(&c));
        assert!(b.is_intersect(&d));
        assert!(b.is_intersect(&e));
        assert!(c.is_intersect(&d));
        assert!(c.is_intersect(&e));
        assert!(d.is_intersect(&f));
        assert!(e.is_intersect(&f));
        assert!(f.is_intersect(&g));
    }
}
