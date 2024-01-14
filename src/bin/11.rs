use advent_of_code::Position;
use std::collections::HashSet;

advent_of_code::solution!(11);

#[derive(PartialEq)]
enum Space {
    Empty,
    Galaxy,
}

use Space::*;

impl Space {
    fn new(ch: &char) -> Option<Self> {
        match ch {
            '.' => Some(Empty),
            '#' => Some(Galaxy),
            _ => None,
        }
    }
}

struct Image {
    galaxies: Vec<Position>,
    rows: HashSet<usize>,
    cols: HashSet<usize>,
}

impl Image {
    fn new(image: &str) -> Self {
        let mut galaxies = vec![];
        let (mut galaxy_rows, mut galaxy_cols) = (HashSet::new(), HashSet::new());
        let (mut row, mut col) = (0, 0);

        image.lines().for_each(|line| {
            row += 1;
            col = 0;
            line.chars().for_each(|ch| {
                col += 1;
                let space = Space::new(&ch).unwrap();
                if space == Galaxy {
                    galaxies.push((row, col).into());
                    galaxy_rows.insert(row);
                    galaxy_cols.insert(col);
                }
            });
        });

        Self {
            galaxies,
            rows: galaxy_rows,
            cols: galaxy_cols,
        }
    }

    fn get_shortest_path_length(
        &self,
        galaxy: &Position,
        other_galaxy: &Position,
        expansion: usize,
    ) -> usize {
        let Position {
            row: galaxy_row,
            col: galaxy_col,
        } = *galaxy;
        let Position {
            row: other_galaxy_row,
            col: other_galaxy_col,
        } = *other_galaxy;

        let mut steps = 0;

        let get_steps = |a: usize, b: usize, should_expand: &dyn Fn(&usize) -> bool| {
            let min = Ord::min(a, b);
            let max = Ord::max(a, b);
            (min + 1..=max)
                .map(|row| if should_expand(&row) { 1 } else { expansion })
                .sum::<usize>()
        };

        steps += get_steps(galaxy_row, other_galaxy_row, &|row| self.rows.contains(row));
        steps += get_steps(galaxy_col, other_galaxy_col, &|col| self.cols.contains(col));

        steps
    }
}

fn solve(input: &str, expansion: usize) -> Option<usize> {
    let image = Image::new(input);
    let galaxies = &image.galaxies;
    let galaxies_len = galaxies.len();
    Some(
        (0..galaxies_len - 1)
            .map(|i| {
                (i + 1..galaxies_len)
                    .map(|j| image.get_shortest_path_length(&galaxies[i], &galaxies[j], expansion))
                    .sum::<usize>()
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
