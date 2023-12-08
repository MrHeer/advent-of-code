use std::ops::Range;

use itertools::Itertools;

advent_of_code::solution!(5);

struct Rule {
    dest_start: u64,
    source_range: Range<u64>,
}

struct MapRange {
    mapped: Vec<Range<u64>>,
    non_mapped: Vec<Range<u64>>,
}

impl Rule {
    fn new(rule_text: &str) -> Rule {
        let mut iter = rule_text
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        let dest_start = iter.next().unwrap();
        let source_start = iter.next().unwrap();
        let range = iter.next().unwrap();
        Rule {
            dest_start,
            source_range: source_start..source_start + range,
        }
    }

    fn map(&self, x: u64) -> Option<u64> {
        match x {
            x if self.source_range.contains(&x) => {
                Some(x - self.source_range.start + self.dest_start)
            }
            _ => None,
        }
    }

    fn map_range(&self, range: &Range<u64>) -> MapRange {
        let Range { start, end } = *range;
        let Range {
            start: source_start,
            end: source_end,
        } = self.source_range;
        match (
            start < source_start && source_start < end,
            start < source_end && source_end < end,
        ) {
            (true, true) => MapRange {
                mapped: vec![Range {
                    start: self.map(source_start).unwrap(),
                    end: self.map(source_end - 1).unwrap() + 1,
                }],
                non_mapped: vec![
                    Range {
                        start,
                        end: source_start,
                    },
                    Range {
                        start: source_end,
                        end: end,
                    },
                ],
            },
            (true, false) => MapRange {
                mapped: vec![Range {
                    start: self.map(source_start).unwrap(),
                    end: self.map(end - 1).unwrap() + 1,
                }],
                non_mapped: vec![Range {
                    start,
                    end: source_start,
                }],
            },
            (false, true) => MapRange {
                mapped: vec![Range {
                    start: self.map(start).unwrap(),
                    end: self.map(source_end - 1).unwrap() + 1,
                }],
                non_mapped: vec![Range {
                    start: source_end,
                    end: end,
                }],
            },
            (false, false) => {
                if end <= source_start || start >= source_end {
                    MapRange {
                        mapped: vec![],
                        non_mapped: vec![range.clone()],
                    }
                } else {
                    MapRange {
                        mapped: vec![Range {
                            start: self.map(start).unwrap(),
                            end: self.map(end - 1).unwrap() + 1,
                        }],
                        non_mapped: vec![],
                    }
                }
            }
        }
    }
}

struct Map {
    rules: Vec<Rule>,
}

impl Map {
    fn new(map_text: &str) -> Map {
        Map {
            rules: map_text.lines().map(Rule::new).collect(),
        }
    }

    fn _map(&self, x: u64) -> Option<u64> {
        use itertools::FoldWhile::{Continue, Done};
        self.rules
            .iter()
            .fold_while(None, |_, rule| match rule.map(x) {
                Some(x) => Done(Some(x)),
                None => Continue(None),
            })
            .into_inner()
    }

    fn map(&self, x: u64) -> u64 {
        self._map(x).unwrap_or(x)
    }

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let (mut mapped, mut non_mapped) = (vec![], vec![range.clone()]);
        self.rules.iter().for_each(|rule| {
            let ranges: Vec<Range<u64>> = non_mapped.drain(..).collect();
            ranges.iter().for_each(|range| {
                let mut map_result = rule.map_range(range);
                mapped.append(&mut map_result.mapped);
                non_mapped.append(&mut map_result.non_mapped);
            });
        });
        [mapped, non_mapped].concat()
    }
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn new(almanac: &str) -> Almanac {
        let mut iter = almanac.split("\n\n");
        let seeds = Almanac::get_seeds(iter.next().unwrap());
        let maps = iter.map(Almanac::get_map).collect();
        Almanac { seeds, maps }
    }

    fn get_seeds(seeds_text: &str) -> Vec<u64> {
        seeds_text
            .split(':')
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn get_map(map_text: &str) -> Map {
        let map_text = map_text.split_once('\n').unwrap().1;
        Map::new(map_text)
    }

    fn get_lowest_location(&self, seeds: impl Iterator<Item = u64>) -> Option<u64> {
        seeds
            .map(|x| self.maps.iter().fold(x, |mapped_x, map| map.map(mapped_x)))
            .min()
    }

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        self.maps.iter().fold(vec![range.clone()], |v, map| {
            v.iter().flat_map(|range| map.map_range(range)).collect()
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = Almanac::new(input);
    let iter = almanac.seeds.iter().map(|a| *a);
    almanac.get_lowest_location(iter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = Almanac::new(input);
    almanac
        .seeds
        .iter()
        .tuples()
        .map(|(&start, &range)| start..start + range)
        .flat_map(|range| almanac.map_range(&range))
        .map(|range| range.start)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
