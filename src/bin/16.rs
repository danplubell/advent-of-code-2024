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
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Facing {
    row: isize,
    col: isize,
}
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
    // queue of positions to process
    let mut q: Vec<(Position, Facing)> = vec![(start_location, Facing { row: 0, col: 1 })];
    let mut all_costs: HashMap<(Position,Position), Cost> = HashMap::new();
    all_costs.insert((start_location, start_location), 0);
    
    while let Some((_current_location, _current_facing)) = q.pop(){
        let curr_cost = all_costs.entry((_current_location,_current_location)).or_insert(isize::MAX);
        let new_points:Vec<(Position,Facing)> = Vec::new();
        if _current_facing == Facing {}
    }
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
}
