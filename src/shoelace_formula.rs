use crate::Position;

// Shoelace Formula - https://en.m.wikipedia.org/wiki/Shoelace_formula
pub fn area_of_circle(circle: &[Position<isize>]) -> usize {
    let circle = &[circle, &[circle[0]]].concat();
    let mut area: isize = 0;

    (0..circle.len() - 1).for_each(|index| {
        let Position { row: y_i, col: x_i } = circle[index];
        let Position {
            row: y_i_plus_1,
            col: x_i_plus_1,
        } = circle[index + 1];

        area += (y_i + y_i_plus_1) * (x_i - x_i_plus_1) / 2;
    });

    area.unsigned_abs()
}
