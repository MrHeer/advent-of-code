use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
};

pub trait Bound {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<Cost, Node> {
    cost: Cost,
    node: Node,
}

impl<Cost, Node> Ord for State<Cost, Node>
where
    Cost: Ord,
    Node: Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl<Cost, Node> PartialOrd for State<Cost, Node>
where
    Cost: Ord,
    Node: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra_search<Cost, Node, F, G, H>(
    starts: Vec<Node>,
    mut is_reach_goal: F,
    mut get_cost: G,
    mut get_neighbors: H,
) -> Option<Cost>
where
    Cost: Bound + Ord + Add<Cost, Output = Cost> + Copy,
    Node: Eq + Hash + Copy,
    F: FnMut(&Node) -> bool,
    G: FnMut(&Node, &Node) -> Cost,
    H: FnMut(&Node) -> Vec<Node>,
{
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    starts.into_iter().for_each(|node| {
        dist.entry(node).or_insert(Cost::min_value());
        heap.push(State {
            cost: Cost::min_value(),
            node,
        });
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, node }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if is_reach_goal(&node) {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > *dist.get(&node).unwrap_or(&Cost::max_value()) {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neighbor in get_neighbors(&node) {
            let next = State {
                cost: cost + get_cost(&node, &neighbor),
                node: neighbor,
            };

            // If so, add it to the frontier and continue
            if next.cost < *dist.get(&next.node).unwrap_or(&Cost::max_value()) {
                heap.push(next);
                // Relaxation, we have now found a better way
                *dist.entry(next.node).or_insert(Cost::max_value()) = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}
