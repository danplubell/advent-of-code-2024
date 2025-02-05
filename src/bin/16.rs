advent_of_code::solution!(16);

use grid::Grid;
use petgraph::algo::{all_simple_paths, dijkstra};
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::Graph;
use priority_queue::{DoublePriorityQueue, PriorityQueue};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
    NoDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Position {
    row: isize,
    col: isize,
}
#[derive(Debug, Clone, PartialEq, Default, Copy, Hash, Eq)]
struct Neighbors {
    top: Option<Position>,
    right: Option<Position>,
    bottom: Option<Position>,
    left: Option<Position>,
}

const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Node {
    position: Position,
    neighbors: Neighbors,
    index: Option<NodeIndex>,
}
fn calc_position(offset: (i32, i32), position: Position) -> Option<Position> {
    Option::from(Position {
        row: position.row.checked_add(offset.0 as isize)?,
        col: position.col.checked_add(offset.1 as isize)?,
    })
}

fn get_neighbors(curr_position: Position, grid: &Grid<char>) -> Neighbors {
    let mut neighbor_positions: [Option<Position>; 4] = [None, None, None, None];
    let neighbors = Neighbors {
        top: None,
        right: None,
        bottom: None,
        left: None,
    };
    for (i, offset) in NEIGHBOR_OFFSETS.iter().enumerate() {
        neighbor_positions[i] = calc_position(NEIGHBOR_OFFSETS[i], curr_position);
        if let Some(new_pos) = neighbor_positions[i] {
            let c = grid.get(new_pos.row, new_pos.col);
            match c {
                Some('#') => neighbor_positions[i] = None,
                _ => neighbor_positions[i] = Some(new_pos),
            }
        }
    }
    Neighbors {
        top: neighbor_positions[0],
        right: neighbor_positions[1],
        bottom: neighbor_positions[2],
        left: neighbor_positions[3],
    }
}
//int cost, int r, int c, int dr, int dc
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct QueueEntry {
    cost: Cost,
    location_entry: LocationEntry,
}
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct LocationEntry {
    position: Position,
    d_position: Position,
}
type Cost = isize;
pub fn part_one(input: &str) -> Option<u32> {
    let mut pq: DoublePriorityQueue<QueueEntry, Cost> = DoublePriorityQueue::new();
    let mut seen: HashSet<LocationEntry> = HashSet::new();

    let grid_rows = input.lines().count();
    let grid_cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(grid_rows, grid_cols);
    let mut start_location = Position { row: 0, col: 0 };
    let mut end_location = Position { row: 0, col: 0 };
    // put stuff in grid
    input.lines().enumerate().for_each(|(row, l)| {
        if l.starts_with("#") {
            l.chars().enumerate().for_each(|(col, c)| {
                let g = grid.get_mut(row, col).unwrap();
                if c == 'S' {
                    start_location = Position { row: row as isize, col: col as isize }
                }
                if c == 'E' {
                    end_location = Position { row: row as isize, col: col as isize }
                }
                *g = c;
            })
        }
    });
    let start_location_entry = LocationEntry {
        position: start_location,
        d_position: Position { row: 0, col: 1 },
    };
    let start_entry = QueueEntry {
        cost: 0,
        location_entry: start_location_entry,
    };
    pq.push(start_entry, 0);
    seen.insert(start_location_entry);
    let mut total_cost: isize = 0;
    while !pq.is_empty() {
        let (entry, cost) = pq.peek_min().unwrap();
        let c = grid
            .get(
                entry.location_entry.position.row,
                entry.location_entry.position.col,
            )
            .unwrap();
        let queue_entry: Option<(QueueEntry, Cost)> = pq.pop_min();
        if let Some((entry, cost)) = queue_entry {
            if *c == 'E' {
                total_cost = cost;
                break;
            }
            // create moves
            let moves: Vec<QueueEntry> = vec![
                QueueEntry {
                    cost: cost + 1,
                    location_entry: LocationEntry {
                        position: Position { row: entry.location_entry.position.row + entry.location_entry.d_position.row,
                            col: entry.location_entry.position.col + entry.location_entry.d_position.col},
                        d_position: Position { row: entry.location_entry.d_position.row,
                            col: entry.location_entry.d_position.col },
                    },
                },
                QueueEntry {
                    cost: cost + 1000,
                    location_entry: LocationEntry {
                        position: Position { row: entry.location_entry.position.row,
                            col: entry.location_entry.position.col },
                        d_position: Position { row: entry.location_entry.d_position.col,
                            col: -entry.location_entry.d_position.row },
                    },
                },
                QueueEntry {
                    cost: cost + 1000,
                    location_entry: LocationEntry {
                        position: Position { row: entry.location_entry.position.row,
                            col: entry.location_entry.position.col },
                        d_position: Position { row: -entry.location_entry.d_position.col,
                            col: entry.location_entry.d_position.row },
                    },
                },
            ];
            moves.iter().for_each(|e| {
                // skip out-of-bounds or blocked cells
                let c = grid.get(e.location_entry.position.row, e.location_entry.position.col);
                if let Some(c) = c {
                    if *c != '#' {
                        let a = seen.insert(e.location_entry);
                        if a {
                            let new_entry = QueueEntry {
                                cost: e.cost,
                                location_entry: e.location_entry,
                            };
                            pq.push(new_entry, e.cost);
                        }
                    }
                }
            })
        }
        /*
                   var (cost, row, col, dirRow, dirCol) = pq.Dequeue();

           if (_grid[row, col] == 'E')
           {
               return cost;
           }

           // Possible moves
           var moves = new (int newCost, int newRow, int newCol, int newDirRow, int newDirCol)[]
           {
               (cost + 1, row + dirRow, col + dirCol, newDirRow: dirRow, newDirCol: dirCol),
               (cost + 1000, newRow: row, newCol: col, newDirRow: dirCol, -dirRow),
               (cost + 1000, newRow: row, newCol: col, -dirCol, newDirCol: dirRow)
           };

           foreach (var (newCost, newRow, newCol, newDirRow, newDirCol) in moves)
           {
               // Skip out-of-bounds or blocked cells
               if (newRow < 0 || newRow >= _rows || newCol < 0 || newCol >= _cols ||
                   _grid[newRow, newCol] == '#') continue;

               // Skip already-seen states
               if (!seen.Add((newRow, newCol, newDirRow, newDirCol))) continue;

               pq.Enqueue((newCost, newRow, newCol, newDirRow, newDirCol), newCost);
           }

        */
    }

    Some(total_cost as u32)
}
fn add_neighbor_edge(
    mut graph: &mut Graph<Node, usize>,
    node: Node,
    neighbor_position: Option<Position>,
    map: &HashMap<Position, Node>,
    direction: Direction,
) -> Option<EdgeIndex> {
    if let Some(neighbor_position) = neighbor_position {
        let neighbor_node = map.get(&neighbor_position);
        if let Some(neighbor_node) = neighbor_node {
            let d = match direction {
                Direction::Right => RIGHT,
                Direction::Left => LEFT,
                Direction::Bottom => BOTTOM,
                Direction::Top => TOP,
                _ => unreachable!(),
            };
            let edge_idx = graph.add_edge(node.index?, neighbor_node.index?, d);
            return Some(edge_idx);
        }
    }
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

/*
pub fn part_one_graph(input: &str) -> Option<u32> {
    let mut graph = Graph::<Node, usize>::new();
    let grid_rows = input.lines().count();
    let grid_cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(grid_rows, grid_cols);
    let mut start_location = Node {
        position: Position { row: 0, col: 0 },
        neighbors: Default::default(),
        index: None,
    };
    let mut end_location = Node {
        position: Position { row: 0, col: 0 },
        neighbors: Default::default(),
        index: None,
    };
    // put stuff in grid
    input.lines().enumerate().for_each(|(row, l)| {
        if l.starts_with("#") {
            l.chars().enumerate().for_each(|(col, c)| {
                let g = grid.get_mut(row, col).unwrap();
                if c == 'S' {
                    start_location.position = Position { row: row as isize, col:col as isize }
                }
                if c == 'E' {
                    end_location.position = Position { row: row as isize, col: col as isize }
                }
                *g = c;
            })
        }
    });
    let mut node_indexes: Vec<NodeIndex> = Vec::new();
    let mut map: HashMap<Position, Node> = HashMap::new();
    // build graph nodes
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            let c = grid.get(i, j).unwrap();
            let neighbors = get_neighbors(Position { row: i as isize, col: j as isize }, &grid);
            let mut node = Node {
                position: Position { row: i as isize, col: j as isize },
                neighbors,
                index: None,
            };
            let ix = graph.add_node(node);
            node.index = Some(ix);
            if node.position == start_location.position {
                start_location.index = Some(ix);
            }
            if node.position == end_location.position {
                end_location.index = Some(ix);
            }

            map.insert(node.position, node);
            node_indexes.push(ix);
        }
    }

    // add edges
    for (n, map_node) in map.iter() {
        // for each neighbor add an edge
        let node = map.get(n);
        if let Some(node) = node {
            add_neighbor_edge(
                &mut graph,
                *map_node,
                node.neighbors.top,
                &map,
                Direction::Top,
            );
            add_neighbor_edge(
                &mut graph,
                *map_node,
                node.neighbors.right,
                &map,
                Direction::Right,
            );
            add_neighbor_edge(
                &mut graph,
                *map_node,
                node.neighbors.bottom,
                &map,
                Direction::Bottom,
            );
            add_neighbor_edge(
                &mut graph,
                *map_node,
                node.neighbors.left,
                &map,
                Direction::Left,
            );
        }
    }
    let paths: Vec<_> =
        all_simple_paths::<Vec<_>, _>(&graph, start_location.index?, end_location.index?, 0, None)
            .collect();
    //let paths = dijkstra(&graph,start_location.index?, Some(end_location.index?), |e| *e.weight());
    println!("{:?}", paths.len());

    //println!("\nShortest path distance to D: {:?}", paths.get(&end_location.index?).unwrap());
    /*    for path in paths {
           if path.len() == 37 {
               for node_index in path {
                   println!("{:?}", graph[node_index].position);
               }
               println!();
           }
       }

    */
    None
}

 */