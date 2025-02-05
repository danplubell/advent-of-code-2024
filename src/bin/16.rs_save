advent_of_code::solution!(16);

use grid::Grid;
use petgraph::algo::{all_simple_paths, astar, dijkstra};
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::Graph;
use priority_queue::{DoublePriorityQueue, PriorityQueue};
use std::collections::{HashMap, HashSet, VecDeque};

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
pub fn part_one1(input: &str) -> Option<u32> {
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
                    start_location = Position {
                        row: row as isize,
                        col: col as isize,
                    }
                }
                if c == 'E' {
                    end_location = Position {
                        row: row as isize,
                        col: col as isize,
                    }
                }
                *g = c;
            })
        }
    });

    None
}
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
                    start_location = Position {
                        row: row as isize,
                        col: col as isize,
                    }
                }
                if c == 'E' {
                    end_location = Position {
                        row: row as isize,
                        col: col as isize,
                    }
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
                println!("length: {}", pq.len());
                break;
            }
            // create moves
            let moves: Vec<QueueEntry> = vec![
                QueueEntry {
                    cost: cost + 1,
                    location_entry: LocationEntry {
                        position: Position {
                            row: entry.location_entry.position.row
                                + entry.location_entry.d_position.row,
                            col: entry.location_entry.position.col
                                + entry.location_entry.d_position.col,
                        },
                        d_position: Position {
                            row: entry.location_entry.d_position.row,
                            col: entry.location_entry.d_position.col,
                        },
                    },
                },
                QueueEntry {
                    cost: cost + 1000,
                    location_entry: LocationEntry {
                        position: Position {
                            row: entry.location_entry.position.row,
                            col: entry.location_entry.position.col,
                        },
                        d_position: Position {
                            row: entry.location_entry.d_position.col,
                            col: -entry.location_entry.d_position.row,
                        },
                    },
                },
                QueueEntry {
                    cost: cost + 1000,
                    location_entry: LocationEntry {
                        position: Position {
                            row: entry.location_entry.position.row,
                            col: entry.location_entry.position.col,
                        },
                        d_position: Position {
                            row: -entry.location_entry.d_position.col,
                            col: entry.location_entry.d_position.row,
                        },
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
    println!("seen: {:?}", seen.len());
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
                    start_location.position = Position {
                        row: row as isize,
                        col: col as isize,
                    }
                }
                if c == 'E' {
                    end_location.position = Position {
                        row: row as isize,
                        col: col as isize,
                    }
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
            let neighbors = get_neighbors(
                Position {
                    row: i as isize,
                    col: j as isize,
                },
                &grid,
            );
            let mut node = Node {
                position: Position {
                    row: i as isize,
                    col: j as isize,
                },
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
    // 1. Find *a* shortest path and its length using astar
    let start_node = start_location.index.unwrap();
    let end_node = end_location.index.unwrap();
    let shortest_path_info = astar(&graph, start_node, |goal| goal == end_node, |_| 1.0, |_| 0f64); // Assuming unit edge weights
    let shortest_path_length = shortest_path_info.map(|(len, _)| len).unwrap();

    // 2. Use BFS to find all shortest paths
    let mut all_shortest_paths = Vec::new();
    let mut bfs_queue = vec![(start_node, vec![start_node])]; // (node, current_path)

    while let Some((current_node, current_path)) = bfs_queue.pop() {
        if current_node == end_node
            && current_path.len() == (shortest_path_length + (1f64)) as usize
        {
            // +1 because we start with start_node
            all_shortest_paths.push(current_path);
        } else if current_path.len() <= (shortest_path_length + (1f64)) as usize {
            for neighbor in graph.neighbors(current_node) {
                let mut new_path = current_path.clone();
                new_path.push(neighbor);
                bfs_queue.push((neighbor, new_path));
            }
        }
    }
    println!("all_shortest_paths: {:?}", all_shortest_paths.len());

    // let paths: Vec<_> =
    //     all_simple_paths::<Vec<_>, _>(&graph, start_location.index?, end_location.index?, 0, None)
    //         .collect();
    /*
    let distances = dijkstra(&graph,start_location.index?, Some(end_location.index?), |e| *e.weight());

    let shortest_path = distances.get(&end_location.index?).unwrap();
    println!("\nShortest path distance to D: {:?}", shortest_path);
    let path = astar(&graph, start_location.index?, |finish| finish == end_location.index.unwrap(), |e| *e.weight(), |_| 0);
    let p = path.unwrap();
    println!("p: {}", p.0);
    for i in 0..400 {
        let p1: Vec<_> = all_simple_paths::<Vec<_>, _>(&graph, start_location.index?, end_location.index?, 0, Some(i)).collect();
        println!("iteration: {}", i);
        if !p1.is_empty() {
            println!("p1: {} {}", i, p1.len());
            break;
        }
    }


     */
    // Store all paths with the shortest distance
    /*    let mut equal_shortest_paths: Vec<Vec<Option<NodeIndex>>> = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(vec![end_location.index]);

        // BFS to find all paths of equal length
        while let Some(current_path) = queue.pop_front() {
            let current_opt = *current_path.first().unwrap();
            let current = current_opt.unwrap();

            if current == start_location.index? {
                let mut path = current_path.clone();
                path.reverse();
                equal_shortest_paths.push(path);
                continue;
            }

            // Check all possible predecessors
            for neighbor in graph.neighbors_directed(current, petgraph::Direction::Incoming) {
                if let Some(&neighbor_dist) = distances.get(&neighbor) {
                    if let Some(edge) = graph.find_edge(neighbor, current) {
                        let weight = graph.edge_weight(edge).unwrap();

                        // If this edge is part of a shortest path
                        if neighbor_dist + weight == distances[&current] {
                            let mut new_path = current_path.clone();
                            new_path.insert(0, Option::from(neighbor));
                            queue.push_back(new_path);
                        }
                    }
                }
            }
        }
        let mut t = 0_usize;
        equal_shortest_paths.iter().for_each(|path|  t += path.len());
        println!("{}", t);
    */
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
use petgraph::Graph;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, VecDeque};

fn find_all_shortest_paths(
    graph: &Graph<&str, i32>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    // Get distances using Dijkstra
    let distances = dijkstra(
        &graph,
        start,
        Some(end),
        |e| *e.weight()
    );

    // Get the shortest distance to end
    let shortest_distance = match distances.get(&end) {
        Some(&distance) => distance,
        None => return vec![], // No path exists
    };

    // Store all paths with the shortest distance
    let mut equal_shortest_paths: Vec<Vec<NodeIndex>> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(vec![end]);

    // BFS to find all paths of equal length
    while let Some(current_path) = queue.pop_front() {
        let current = *current_path.first().unwrap();

        if current == start {
            let mut path = current_path.clone();
            path.reverse();
            equal_shortest_paths.push(path);
            continue;
        }

        // Check all possible predecessors
        for neighbor in graph.neighbors_directed(current, petgraph::Direction::Incoming) {
            if let Some(&neighbor_dist) = distances.get(&neighbor) {
                if let Some(edge) = graph.find_edge(neighbor, current) {
                    let weight = graph.edge_weight(edge).unwrap();

                    // If this edge is part of a shortest path
                    if neighbor_dist + weight == distances[&current] {
                        let mut new_path = current_path.clone();
                        new_path.insert(0, neighbor);
                        queue.push_back(new_path);
                    }
                }
            }
        }
    }

    equal_shortest_paths
}

fn main() {
    let mut graph = Graph::<&str, i32>::new();

    // Create nodes
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    // Add edges with weights - creating multiple paths of equal length
    graph.add_edge(a, b, 1);
    graph.add_edge(b, d, 2);
    graph.add_edge(a, c, 1);
    graph.add_edge(c, d, 2);

    let paths = find_all_shortest_paths(&graph, a, d);

    println!("All shortest paths from A to D:");
    for (i, path) in paths.iter().enumerate() {
        print!("Path {}: ", i + 1);
        for node_idx in path {
            print!("{} -> ", graph[*node_idx]);
        }
        println!("");
    }
}
 */
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
