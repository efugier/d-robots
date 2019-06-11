use crate::ai::CellState;
use ndarray::Array2;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

// A Node is a possible position on the grid
#[derive(Clone, PartialEq, Debug)]
struct Node {
    cost: f32,
    x: u32,
    y: u32,
    parent: Option<Box<Node>>,
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
            .then_with(|| self.x.cmp(&other.x))
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
    self_pos: (u32, u32),
    map_seen: &Array2<CellState>,
    dest: (u32, u32),
) -> Vec<(u32, u32)> {
    if dest.0 >= map_seen.shape()[0] as u32 || dest.1 >= map_seen.shape()[1] as u32 {
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

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = map_seen.map(|_| usize::MAX);

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[(self_pos.0 as usize, self_pos.1 as usize)] = 0;
    heap.push(Node {
        cost: 0.,
        x: self_pos.0,
        y: self_pos.1,
        parent: None,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(Node {
        cost,
        x: curr_x,
        y: curr_y,
        parent,
    }) = heap.pop()
    {
        // Destination reached
        if (curr_x, curr_y) == dest {
            path.push((curr_x as u32, curr_y as u32));
            let mut prev = parent;
            while let Some(curr) = prev {
                path.push((curr.x as u32, curr.y as u32));
                prev = curr.parent;
            }
            return path;
        }

        // Comparing costs as we may have already found a better way
        // Cost = effective distance from origin + square distance to destination
        // if cost
        //     > (dist[(curr_x as usize, curr_y as usize)]
        //         + ((curr_x - (dest.0 as i32)).pow(2) + (curr_y - (dest.1 as i32)).pow(2)) as usize)
        // {
        //     continue;
        // }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for mv in &moves {
            let new_x = curr_x as i32 + mv.0;
            let new_y = curr_y as i32 + mv.1;
            let new_cost = dist[(curr_x as usize, curr_y as usize)] as f32
                + 1.
                + ((new_x as f32 - dest.0 as f32).powf(2.)
                    + (new_y as f32 - dest.1 as f32).powf(2.))
                .sqrt();
            if new_x < map_seen.shape()[0] as i32
                && new_y < map_seen.shape()[1] as i32
                && new_x >= 0
                && new_y >= 0
                && map_seen[(new_x as usize, new_y as usize)] != CellState::Blocked
            {
                let next = Node {
                    cost: new_cost,
                    x: new_x as u32,
                    y: new_y as u32,
                    parent: Some(Box::new(Node {
                        cost,
                        x: curr_x,
                        y: curr_y,
                        parent: parent.clone(),
                    })),
                };

                // If so, add it to the frontier and continue
                if dist[(curr_x as usize, curr_y as usize)] + 1
                    < dist[(new_x as usize, new_y as usize)]
                {
                    heap.push(next);
                    // Update, we have now found a better way
                    dist[(new_x as usize, new_y as usize)] =
                        dist[(curr_x as usize, curr_y as usize)] + 1;
                }
            }
        }
    }

    // Destination not reachable
    log::error!(
        "[Pathfinding] destination point {:?} is unreachable from {:?}",
        dest,
        self_pos
    );
    path
}
