use crate::{area_of_circle, Position};

// Pick's Theorem - https://en.m.wikipedia.org/wiki/Pick%27s_theorem
pub fn number_of_interiors(circle: &[Position<isize>]) -> usize {
    let area = area_of_circle(circle);
    let number_of_boundary_points = circle.len();

    area + 1 - number_of_boundary_points / 2
}
