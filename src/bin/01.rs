use regex::Regex;

advent_of_code::solution!(1);

fn find_first_digit(haystack: &str, re: &Regex) -> Option<u32> {
    let digit_or_name = re.captures(haystack)?.get(0)?.as_str();
    let digit = match digit_or_name {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => digit_or_name.parse().unwrap(),
    };
    Some(digit)
}

fn find_last_digit(haystack: &str, re: &Regex) -> Option<u32> {
    let reversed_haystack = haystack.chars().rev().collect::<String>();
    let digit_or_name = re.captures(&reversed_haystack)?.get(0)?.as_str();
    let digit = match digit_or_name {
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => digit_or_name.parse().unwrap(),
    };
    Some(digit)
}

fn get_calibration_value(first_digit: Option<u32>, last_digit: Option<u32>) -> Option<u32> {
    match (first_digit, last_digit) {
        (Some(first_digit), Some(last_digit)) => {
            let value = first_digit * 10 + last_digit;
            Some(value)
        }
        _ => None,
    }
}

fn solve(input: &str, re: &Regex, reversed_re: &Regex) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let first_digit = find_first_digit(line, re);
            let last_digit = find_last_digit(line, reversed_re);
            get_calibration_value(first_digit, last_digit).unwrap_or_default()
        })
        .reduce(|sum, value| sum + value)
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d)").unwrap();
    solve(input, &re, &re)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reversed_re = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    solve(input, &re, &reversed_re)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
