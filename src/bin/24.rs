use matrix::{equation, line::Line, vector};

advent_of_code::solution!(24);

#[derive(Clone, Copy)]
struct Time {
    nanosecond: f64,
}

impl Time {
    fn new(nanosecond: f64) -> Self {
        Self { nanosecond }
    }
}

struct Position {
    x: f64,
    y: f64,
    z: f64,
}

impl From<(f64, f64, f64)> for Position {
    fn from(value: (f64, f64, f64)) -> Self {
        let (x, y, z) = value;
        Self { x, y, z }
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut iter = value.split(',').map(|x| x.trim().parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
            .into()
    }
}

struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

impl From<(f64, f64, f64)> for Velocity {
    fn from(value: (f64, f64, f64)) -> Self {
        let (x, y, z) = value;
        Self { x, y, z }
    }
}

impl From<&str> for Velocity {
    fn from(value: &str) -> Self {
        let mut iter = value.split(',').map(|x| x.trim().parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
            .into()
    }
}

struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let mut iter = value.split('@');
        let position = iter.next().unwrap().into();
        let velocity = iter.next().unwrap().into();
        Self { position, velocity }
    }
}

struct Position2D {
    x: f64,
    y: f64,
}

impl From<Position> for Position2D {
    fn from(value: Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

struct Velocity2D {
    x: f64,
    y: f64,
}

impl From<Velocity> for Velocity2D {
    fn from(value: Velocity) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

struct Hailstone2D {
    position: Position2D,
    velocity: Velocity2D,
}

impl From<Hailstone> for Hailstone2D {
    fn from(value: Hailstone) -> Self {
        Self {
            position: value.position.into(),
            velocity: value.velocity.into(),
        }
    }
}

enum Intersection<T> {
    None,
    Some(T),
    Infinity,
}

impl Hailstone2D {
    fn get_line(&self) -> Line {
        let Position2D { x: px, y: py } = self.position;
        let Velocity2D { x: vx, y: vy } = self.velocity;
        let normal_vector = vector([vy, -vx]);
        let constant_term = vy * px - vx * py;
        equation(normal_vector, constant_term)
    }

    fn intersect(&self, other: &Self) -> Intersection<Position2D> {
        let line = self.get_line();
        let other_line = other.get_line();
        match line.intersect(&other_line) {
            matrix::line::Intersection::Some(vector) => Intersection::Some(Position2D {
                x: vector[0],
                y: vector[1],
            }),
            matrix::line::Intersection::None => Intersection::None,
            matrix::line::Intersection::Infinity(_) => Intersection::Infinity,
        }
    }

    fn position(&self, time: &Time) -> Position2D {
        Position2D {
            x: self.position.x + time.nanosecond * self.velocity.x,
            y: self.position.y + time.nanosecond * self.velocity.y,
        }
    }

    fn collide(&self, other: &Self) -> Option<(Time, Time, Position2D)> {
        let time = match self.intersect(other) {
            Intersection::Some(position) => Some((
                Time::new((position.x - self.position.x) / self.velocity.x),
                Time::new((position.x - other.position.x) / other.velocity.x),
            )),
            _ => None,
        };
        time.map(|(time, other_time)| {
            let position = self.position(&time);
            (time, other_time, position)
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones: Vec<Hailstone2D> = input
        .lines()
        .map(Hailstone::from)
        .map(Hailstone2D::from)
        .collect();
    let mut intersections = 0;
    (0..hailstones.len() - 1).for_each(|i| {
        (i + 1..hailstones.len()).for_each(|j| {
            if let Some((time, other_time, position)) = hailstones[i].collide(&hailstones[j]) {
                if time.nanosecond > 0.
                    && other_time.nanosecond > 0.
                    && position.x >= 200000000000000.
                    && position.y >= 200000000000000.
                    && position.x <= 400000000000000.
                    && position.y <= 400000000000000.
                {
                    intersections += 1;
                }
            }
        })
    });
    Some(intersections)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
