use std::collections::{HashMap, HashSet};

advent_of_code::solution!(25);

struct Graph {
    adjacencies: HashMap<String, HashSet<String>>,
}

impl From<&str> for Graph {
    fn from(value: &str) -> Self {
        value.lines().fold(
            Graph {
                adjacencies: HashMap::new(),
            },
            |mut graph, line| {
                let (source, targets) = line.split_once(": ").unwrap();
                targets.split_whitespace().for_each(|target| {
                    graph
                        .adjacencies
                        .entry(source.to_string())
                        .or_default()
                        .insert(target.to_string());
                    graph
                        .adjacencies
                        .entry(target.to_string())
                        .or_default()
                        .insert(source.to_string());
                });
                graph
            },
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = Graph::from(input);
    graph.adjacencies.iter().for_each(|(source, targets)| {
        targets.iter().for_each(|target| {
            println!("{source} --- {target}");
        });
    });
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
