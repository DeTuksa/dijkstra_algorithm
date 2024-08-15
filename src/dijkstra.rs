use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, usize};

use crate::graph::Graph;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct State {
    cost: usize,
    node: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(graph: &Graph, start: usize, goal: usize) -> Option<(usize, Vec<usize>)> {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut previous: HashMap<usize, usize> = HashMap::new();

    distances.insert(start, 0);
    heap.push(State { cost: 0, node: start });

    while let Some(State {cost, node}) = heap.pop() {
        if node == goal {
            let mut path = Vec::new();
            let mut current_node = goal;
            while current_node != start {
                path.push(current_node);
                current_node = *previous.get(&current_node)?;
            }
            path.push(start);
            path.reverse();
            return Some((cost, path));
        }

        if cost > *distances.get(&node).unwrap_or(&usize::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.nodes.get(&node) {
            for &(neighbor, weight) in neighbors {
                let next = State {
                    cost: cost + weight,
                    node: neighbor
                };

                if next.cost < *distances.get(&neighbor).unwrap_or(&usize::MAX) {
                    heap.push(next);
                    distances.insert(neighbor, next.cost);
                    previous.insert(neighbor, node);
                }
            }
        }
    }
    None
}