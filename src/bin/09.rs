advent_of_code::solution!(9);

struct History(Vec<i32>);

impl History {
    fn new(history: &str) -> Self {
        Self(
            history
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        )
    }
    fn extrapolate(&self) -> i32 {
        let mut values = self.0.clone();
        let mut result = 0;

        while values.iter().all(|x| *x == 0) == false {
            for i in 0..values.len() - 1 {
                values[i] = values[i + 1] - values[i];
            }
            result += values.pop().unwrap();
        }

        result
    }

    fn extrapolate_backward(&self) -> i32 {
        let mut values = self.0.clone();
        let mut sign = 1;
        let mut result = 0;

        while values.iter().all(|x| *x == 0) == false {
            result += sign * values.first().unwrap();
            sign *= -1;
            for i in 0..values.len() - 1 {
                values[i] = values[i + 1] - values[i];
            }
            values.pop();
        }

        result
    }
}

fn parse_histories(histories: &str) -> Vec<History> {
    histories.lines().map(History::new).collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(parse_histories(input).iter().map(|h| h.extrapolate()).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        parse_histories(input)
            .iter()
            .map(|h| h.extrapolate_backward())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
