advent_of_code::solution!(16);

use grid::Grid;
use priority_queue::DoublePriorityQueue;
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

type Facing = (i32, i32);
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Visited {
    position: Position,
    dir: usize,
    blocked: bool,
}
#[derive(Debug, Clone, PartialEq, Default, Copy, Hash, Eq)]
struct Neighbors {
    top: Option<Position>,
    right: Option<Position>,
    bottom: Option<Position>,
    left: Option<Position>,
}

const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const UP: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);

fn calc_position(offset: (i32, i32), position: Position) -> Option<Position> {
    Option::from(Position {
        row: position.row.checked_add(offset.0 as isize)?,
        col: position.col.checked_add(offset.1 as isize)?,
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid_rows = input.lines().count();
    let grid_cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(grid_rows, grid_cols);
    let mut start_location = Position { row: 0, col: 0 };
    let mut end_location = Position { row: 0, col: 0 };

    // put stuff in grid
    input.lines().enumerate().for_each(|(row, l)| {
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
    });
    let mut pq: DoublePriorityQueue<QueueEntry, Cost> = DoublePriorityQueue::new();
    let initial_entry = QueueEntry {
        cost: 0,
        location_entry: LocationEntry {
            position: start_location,
            d_position: Position { row: 0, col: 1 },
        },
    };
    pq.push(initial_entry, 0);
    let mut lowest_cost: HashMap<LocationEntry, Cost> = HashMap::new();
    lowest_cost.insert(
        LocationEntry {
            position: start_location,
            d_position: Position { row: 0, col: 1 },
        },
        0,
    );
    let back_track: HashMap<LocationEntry, HashSet<LocationEntry>> = HashMap::new();
    let end_state:HashSet<LocationEntry> = HashSet::new();
    let best_cost = isize::MAX;
    while let Some((state,cost)) = pq.pop_min() {
        let current_cost = state.cost;
        let r = state.location_entry.position.row;
        let c = state.location_entry.position.col;
        let dr = state.location_entry.d_position.row;
        let dc = state.location_entry.d_position.col;
        
        if current_cost > *lowest_cost.get(&state.location_entry).unwrap_or(&isize::MAX){
            continue;
        }
    }
    None
}

// This example is from a python solution.  It is off by 1000 points somewhere
pub fn part_two_from_python(input: &str) -> Option<u32> {
    let grid_rows = input.lines().count();
    let grid_cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(grid_rows, grid_cols);
    let mut start_location = Position { row: 0, col: 0 };
    let mut end_location = Position { row: 0, col: 0 };

    // put stuff in grid
    input.lines().enumerate().for_each(|(row, l)| {
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
    });
    // queue of positions to process
    let mut q: Vec<(Position, Facing)> = vec![(start_location, DOWN)];
    let mut all_costs: HashMap<(Position, Facing), Cost> = HashMap::new();
    all_costs.insert((start_location, DOWN), 0);

    while let Some((_current_location, _current_facing)) = q.pop() {
        let curr_cost = *all_costs
            .entry((_current_location, _current_facing))
            .or_insert(isize::MAX);
        let mut new_points: HashMap<(Position, Facing), Cost> = HashMap::new();
        if _current_facing == DOWN || _current_facing == UP {
            new_points.insert((_current_location, RIGHT), 1000);
            new_points.insert((_current_location, LEFT), 1000);
        }
        if _current_facing == RIGHT || _current_facing == LEFT {
            new_points.insert((_current_location, DOWN), 1000);
            new_points.insert((_current_location, UP), 1000);
        }
        let same_direction_pos = calc_position(_current_facing, _current_location).unwrap();
        new_points.insert((same_direction_pos, _current_facing), 1);
        for (new_state, cost_incr) in new_points.iter() {
            let (pp, new_fac) = new_state;
            let c = grid.get(pp.row, pp.col).unwrap();
            if *c == '#' {
                continue;
            }
            let new_cost = curr_cost + *cost_incr;
            let check_cost = all_costs.get(&(*pp, *new_fac));
            if check_cost.is_some() && check_cost.unwrap() <= &new_cost {
                continue;
            }
            all_costs.insert((*pp, *new_fac), new_cost);
            q.push((*pp, *new_fac));
        }
    }
    let mut lowest = isize::MAX;
    let mut end_fac = (0, 0);
    for e in all_costs {
        let ((position, facing), cost) = e;
        if position == end_location && cost < lowest {
            lowest = cost;
            end_fac = facing;
        }
    }
    println!("lowest: {} {:?}", lowest, end_fac);
    None
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
    }
    println!("seen: {:?}", seen.len());
    Some(total_cost as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9846));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_part_two_python() {
        let result = part_two_from_python(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
