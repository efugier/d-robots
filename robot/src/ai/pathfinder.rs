use crate::ai::CellState;
use ndarray::Array2;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

// A Node is a possible position on the grid
#[derive(Clone, PartialEq, Debug)]
struct Node {
    cost: f32,
    xy: (usize, usize),
    parent: usize,
}

impl Eq for Node {}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap()
            .then_with(|| self.xy.cmp(&other.xy))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// A* algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn find_path(
    start: (u32, u32),
    map_seen: &Array2<CellState>,
    dest: (u32, u32),
) -> Vec<(u32, u32)> {
    let start = (start.0 as usize, start.1 as usize);
    let dest = (dest.0 as usize, dest.1 as usize);
    if dest.0 >= map_seen.shape()[0] || dest.1 >= map_seen.shape()[1] {
        log::error!("[Pathfinding] destination point {:?} is out of map", dest);
    }
    let moves = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut path = Vec::new();
    let mut parents = Vec::new();

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = map_seen.map(|_| usize::MAX);

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    let first_node = Node {
        cost: 0.,
        xy: start,
        parent: 0,
    };
    heap.push(first_node.clone());
    parents.push(first_node.clone());
    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(Node { cost, xy, parent }) = heap.pop() {
        // Destination reached
        if xy == dest {
            path.push((xy.0 as u32, xy.1 as u32));
            let mut prev = &parents[parent];
            while *prev != first_node {
                path.push((prev.xy.0 as u32, prev.xy.1 as u32));
                prev = &parents[prev.parent];
            }
            return path;
        }

        parents.push(Node { cost, xy, parent });

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for mv in &moves {
            let new = (xy.0 as i32 + mv.0, xy.1 as i32 + mv.1);
            if new.0 < map_seen.shape()[0] as i32
                && new.1 < map_seen.shape()[1] as i32
                && new.0 >= 0
                && new.1 >= 0
                && map_seen[(new.0 as usize, new.1 as usize)] != CellState::Blocked
            {
                let new_cost = dist[xy] as f32
                    + 1.
                    + ((new.0 as f32 - dest.0 as f32).powf(2.)
                        + (new.1 as f32 - dest.1 as f32).powf(2.))
                    .sqrt();

                let new = (new.0 as usize, new.1 as usize);
                let next = Node {
                    cost: new_cost,
                    xy: new,
                    parent: parents.len() - 1,
                };

                // If so, add it to the frontier and continue
                if dist[xy] + 1 < dist[new] {
                    heap.push(next);
                    // Update, we have now found a better way
                    dist[new] = dist[xy] + 1;
                }
            }
        }
    }

    // Destination not reachable
    log::error!(
        "[Pathfinding] destination point {:?} is unreachable from {:?}",
        dest,
        start
    );
    path
}
