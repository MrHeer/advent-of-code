advent_of_code::solution!(2);

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_set(set: &str) -> Set {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    let cubes = set.split(',');

    cubes.for_each(|cube| {
        let mut info = cube.split_ascii_whitespace();
        let count = info.next().unwrap().parse().unwrap();
        let color = info.next().unwrap();
        match color {
            "red" => red = count,
            "green" => green = count,
            "blue" => blue = count,
            _ => panic!("other color"),
        }
    });

    Set { red, green, blue }
}

fn parse_line(line: &str) -> Game {
    let mut id_and_sets = line.split(": ");

    let id = id_and_sets
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let sets = id_and_sets
        .next()
        .unwrap()
        .split(';')
        .map(parse_set)
        .collect();

    Game { id, sets }
}

fn is_possible(game: &Game) -> bool {
    for set in game.sets.iter() {
        let &Set { red, green, blue } = set;
        if red > 12 || green > 13 || blue > 14 {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter(is_possible)
            .map(|game| game.id)
            .sum(),
    )
}

fn fewest_set(game: &Game) -> Set {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    game.sets.iter().for_each(|set| {
        if red < set.red {
            red = set.red;
        }
        if blue < set.blue {
            blue = set.blue;
        }
        if green < set.green {
            green = set.green;
        }
    });

    Set { red, green, blue }
}

fn multiply(set: &Set) -> u32 {
    set.red * set.green * set.blue
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .map(|game| fewest_set(&game))
            .map(|set| multiply(&set))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
