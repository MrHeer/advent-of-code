use matrix::{
    equation,
    line::{Intersection, Line},
    linear_system, vector, Solution, Vector,
};

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

struct Hailstone {
    position: Vector<3>,
    velocity: Vector<3>,
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let mut iter = value.split('@');

        fn into_vector(value: &str) -> Vector<3> {
            let mut iter = value.split(',').map(|x| x.trim().parse::<f64>().unwrap());
            vector([
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ])
        }

        let position = into_vector(iter.next().unwrap());
        let velocity = into_vector(iter.next().unwrap());
        Self { position, velocity }
    }
}

impl Hailstone {
    fn get_line_in_xy(&self) -> Line {
        let px = self.position[0];
        let py = self.position[1];
        let vx = self.velocity[0];
        let vy = self.velocity[1];
        let normal_vector = vector([vy, -vx]);
        let constant_term = vy * px - vx * py;
        equation(normal_vector, constant_term)
    }

    fn intersect(&self, other: &Self) -> Intersection {
        let line = self.get_line_in_xy();
        let other_line = other.get_line_in_xy();
        line.intersect(&other_line)
    }

    fn position_at(&self, time: &Time) -> Vector<3> {
        self.position + self.velocity * time.nanosecond
    }

    fn collide(&self, other: &Self) -> Option<(Time, Time, Vector<3>)> {
        let time = match self.intersect(other) {
            Intersection::Some(position) => Some((
                Time::new((position[0] - self.position[0]) / self.velocity[0]),
                Time::new((position[0] - other.position[0]) / other.velocity[0]),
            )),
            _ => None,
        };
        time.map(|(time, other_time)| {
            let position = self.position_at(&time);
            (time, other_time, position)
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::from).collect();
    let mut intersections = 0;
    (0..hailstones.len() - 1).for_each(|i| {
        (i + 1..hailstones.len()).for_each(|j| {
            if let Some((time, other_time, position)) = hailstones[i].collide(&hailstones[j]) {
                if time.nanosecond > 0.
                    && other_time.nanosecond > 0.
                    && position[0] >= 200000000000000.
                    && position[1] >= 200000000000000.
                    && position[0] <= 400000000000000.
                    && position[1] <= 400000000000000.
                {
                    intersections += 1;
                }
            }
        })
    });
    Some(intersections)
}

pub fn part_two(input: &str) -> Option<usize> {
    // (p - p[i]) x (v - v[i]) = 0
    // https://typst.app/project/rDEObAUN5ac0NsNYoFnSI7
    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::from).collect();
    let v01 = hailstones[0].velocity - hailstones[1].velocity;
    let p10 = hailstones[1].position - hailstones[0].position;
    let c01 = hailstones[1].position.cross(&hailstones[1].velocity)
        - hailstones[0].position.cross(&hailstones[0].velocity);
    let v12 = hailstones[1].velocity - hailstones[2].velocity;
    let p21 = hailstones[2].position - hailstones[1].position;
    let c12 = hailstones[2].position.cross(&hailstones[2].velocity)
        - hailstones[1].position.cross(&hailstones[1].velocity);

    let system = linear_system([
        equation(vector([0., -v01[2], v01[1], 0., -p10[2], p10[1]]), c01[0]),
        equation(vector([v01[2], 0., -v01[0], p10[2], 0., -p10[0]]), c01[1]),
        equation(vector([-v01[1], v01[0], 0., -p10[1], p10[0], 0.]), c01[2]),
        equation(vector([0., -v12[2], v12[1], 0., -p21[2], p21[1]]), c12[0]),
        equation(vector([v12[2], 0., -v12[0], p21[2], 0., -p21[0]]), c12[1]),
        equation(vector([-v12[1], v12[0], 0., -p21[1], p21[0], 0.]), c12[2]),
    ]);

    match system.compute_solution() {
        Solution::Some(s) => Some(s[0] as usize + s[1] as usize + s[2] as usize),
        _ => None,
    }
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
        assert_eq!(result, Some(47));
    }
}
