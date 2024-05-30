use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Not,
};

use advent_of_code::{Direction, Matrix, Position as P};

type Position = P<usize>;

advent_of_code::solution!(23);

struct Map {
    tiles: Matrix<Tile>,
    graph: Graph,
}

#[derive(PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

use Direction::*;
use Tile::*;

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Path,
            '#' => Forest,
            '^' => Slope(Up),
            '>' => Slope(Right),
            'v' => Slope(Down),
            '<' => Slope(Left),
            _ => panic!("Invalid tile"),
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self::new(Matrix::from(value))
    }
}

type Node = Position;

#[derive(PartialEq, Eq, Hash)]
struct Edge {
    a: Node,
    b: Node,
}

impl Edge {
    fn new(a: Node, b: Node) -> Self {
        if a == b {
            panic!("Edge should have two difference node.");
        }
        if a.row < b.row || (a.row == b.row && a.col < b.col) {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }
}

struct Graph {
    adjacencies: HashMap<Node, HashSet<Node>>,
    distances: HashMap<Edge, usize>,
}

impl Graph {
    fn contraction(&mut self) {
        let node = self
            .adjacencies
            .iter()
            .find(|(_, adjacency)| adjacency.len() == 2);
        if node.is_none() {
            return;
        }
        let node = *node.unwrap().0;
        let mut adjacency_iter = self.adjacencies.get(&node).unwrap().iter();
        let a = *adjacency_iter.next().unwrap();
        let b = *adjacency_iter.next().unwrap();

        let a_adjacency = self.adjacencies.get_mut(&a).unwrap();
        a_adjacency.remove(&node);
        a_adjacency.insert(b);
        let b_adjacency = self.adjacencies.get_mut(&b).unwrap();
        b_adjacency.remove(&node);
        b_adjacency.insert(a);
        self.adjacencies.remove(&node);

        let new_edge = Edge::new(a, b);
        let new_distance = self.get_distance(a, node) + self.get_distance(node, b);

        self.distances
            .entry(new_edge)
            .and_modify(|distance| *distance = (*distance).max(new_distance))
            .or_insert(new_distance);
        self.contraction();
    }

    fn get_distance(&self, a: Node, b: Node) -> usize {
        *self.distances.get(&Edge::new(a, b)).unwrap_or(&1)
    }
}

struct Hike(Vec<Position>);

impl Hike {
    fn new(pos: &Position) -> Self {
        Self(vec![*pos])
    }

    fn is_visited(&self, pos: &Position) -> bool {
        self.0.contains(pos)
    }

    fn position(&self) -> Position {
        *self.0.last().unwrap()
    }

    fn get_node(&self, index: usize) -> Node {
        self.0[index]
    }

    fn steps(&self, graph: &Graph) -> usize {
        let mut distance = 0;

        for i in 0..self.0.len() - 1 {
            distance += graph.get_distance(self.get_node(i), self.get_node(i + 1))
        }

        distance
    }

    fn append(&self, pos: &Position) -> Self {
        let mut cloned = self.0.clone();
        cloned.push(*pos);
        Self(cloned)
    }
}

impl Map {
    fn new(tiles: Matrix<Tile>) -> Self {
        let mut graph = Self::build_graph(&tiles);
        graph.contraction();
        Self { tiles, graph }
    }

    fn find_start(&self) -> Position {
        for col in 1..=self.tiles.cols {
            let position = (1, col).into();
            if self.tiles[position] == Path {
                return position;
            }
        }
        panic!("Could not find start position.");
    }

    fn find_goal(&self) -> Position {
        for col in 1..=self.tiles.cols {
            let position = (self.tiles.rows, col).into();
            if self.tiles[position] == Path {
                return position;
            }
        }
        panic!("Could not find goal position.");
    }

    fn get_neighbors(tiles: &Matrix<Tile>, position: &Position) -> Vec<Position> {
        let positions = match tiles[*position] {
            Path => tiles.adjacent_positions(position),
            Forest => panic!("You should not be Forest"),
            Slope(Up) => vec![*position.clone().move_to(&Up, 1)],
            Slope(Down) => vec![*position.clone().move_to(&Down, 1)],
            Slope(Left) => vec![*position.clone().move_to(&Left, 1)],
            Slope(Right) => vec![*position.clone().move_to(&Right, 1)],
        };
        positions
            .into_iter()
            .filter(|pos| tiles[*pos] != Forest)
            .collect()
    }

    fn build_graph(tiles: &Matrix<Tile>) -> Graph {
        let nodes = Vec::from_iter(
            tiles
                .indexes()
                .into_iter()
                .filter(|pos| tiles[*pos] != Forest),
        );

        let mut adjacencies = HashMap::new();
        nodes.iter().for_each(|node| {
            adjacencies.insert(*node, HashSet::from_iter(Self::get_neighbors(tiles, node)));
        });

        let distances = HashMap::new();

        Graph {
            adjacencies,
            distances,
        }
    }

    fn next_step(&self, hike: Hike) -> Vec<Hike> {
        self.graph
            .adjacencies
            .get(&hike.position())
            .unwrap()
            .iter()
            .filter_map(|pos| hike.is_visited(pos).not().then_some(hike.append(pos)))
            .collect()
    }

    fn find_all_hikes(&self) -> Vec<Hike> {
        let start = self.find_start();
        let goal = self.find_goal();
        let is_reach_goal = |hike: &Hike| hike.position() == goal;
        let mut stack = vec![Hike::new(&start)];
        let mut hikes = vec![];

        while let Some(hike) = stack.pop() {
            if is_reach_goal(&hike) {
                hikes.push(hike);
                continue;
            }
            stack.append(&mut self.next_step(hike))
        }

        hikes
    }

    fn longest_hike(&self) -> usize {
        self.find_all_hikes()
            .iter()
            .map(|hike| hike.steps(&self.graph))
            .max()
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Map::from(input).longest_hike())
}

fn covert_map(map: &Map) -> Map {
    let tiles = map.tiles.map(|tile| match tile {
        Path => Path,
        Forest => Forest,
        Slope(_) => Path,
    });
    Map::new(tiles)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(covert_map(&Map::from(input)).longest_hike())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
