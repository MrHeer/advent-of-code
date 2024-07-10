use rand::Rng;
use std::collections::HashMap;

advent_of_code::solution!(25);

const SPLIT: char = '-';

#[derive(Clone)]
struct Graph {
    adjacencies: HashMap<String, Vec<String>>,
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
                        .push(target.to_string());
                    graph
                        .adjacencies
                        .entry(target.to_string())
                        .or_default()
                        .push(source.to_string());
                });
                graph
            },
        )
    }
}

impl Graph {
    fn contract_edge(&mut self, src: &str, dst: &str) {
        fn rebind(graph: &mut Graph, old_node: &str, new_node: &str) {
            let neighbors = graph.adjacencies.get(old_node).unwrap().clone();
            neighbors.iter().for_each(|neighbor| {
                graph
                    .adjacencies
                    .get_mut(neighbor)
                    .unwrap()
                    .iter_mut()
                    .for_each(|node| {
                        if node == old_node {
                            *node = new_node.to_string()
                        }
                    });
            });
        }

        self.adjacencies
            .get_mut(src)
            .unwrap()
            .retain(|node| node != dst);
        self.adjacencies
            .get_mut(dst)
            .unwrap()
            .retain(|node| node != src);
        let new_vertex = format!("{}{}{}", src, SPLIT, dst);
        self.adjacencies
            .insert(new_vertex.clone(), Default::default());
        rebind(self, src, &new_vertex);
        rebind(self, dst, &new_vertex);

        let mut src_neighbors = self.adjacencies.remove(src).unwrap();
        let mut dst_neighbors = self.adjacencies.remove(dst).unwrap();

        let new_neighbors = self.adjacencies.get_mut(&new_vertex).unwrap();
        new_neighbors.append(&mut src_neighbors);
        new_neighbors.append(&mut dst_neighbors);
    }

    fn find_min_cut(&mut self) -> usize {
        let mut rng = rand::thread_rng();

        while self.adjacencies.len() > 2 {
            let random_vertex = self
                .adjacencies
                .keys()
                .nth(rng.gen_range(0..self.adjacencies.len()))
                .unwrap()
                .clone();
            let neighbors = self.adjacencies.get(&random_vertex).unwrap();
            let random_neighbor = neighbors
                .get(rng.gen_range(0..neighbors.len()))
                .unwrap()
                .clone();

            self.contract_edge(&random_vertex, &random_neighbor);
        }

        self.adjacencies.values().next().unwrap().len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = Graph::from(input);
    loop {
        let mut cloned_graph = graph.clone();
        if cloned_graph.find_min_cut() == 3 {
            return Some(
                cloned_graph
                    .adjacencies
                    .keys()
                    .map(|node| node.split(SPLIT).count())
                    .product(),
            );
        }
    }
}

pub fn part_two(_input: &str) -> Option<usize> {
    Some(50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }
}
