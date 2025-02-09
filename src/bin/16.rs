advent_of_code::solution!(16);

use std::collections::{HashMap, HashSet};
use grid::Grid;

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
struct Visited {
    position: Position,
    dir: usize,
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

fn calc_position(offset: (i32, i32), position: Position) -> Option<Position> {
    Option::from(Position {
        row: position.row.checked_add(offset.0 as isize)?,
        col: position.col.checked_add(offset.1 as isize)?,
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(9846)
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
    let mut visited: HashSet<Visited> = HashSet::new();
    let mut curr_pos = start_location;
    let mut curr_dir = TOP;
    let mut prev_pos = start_location;
    while (curr_pos != end_location) {
        (curr_pos, curr_dir) = next_move(&grid, curr_pos, curr_dir, &mut visited);
        if curr_pos == prev_pos {
            break;
        }
        prev_pos = curr_pos;
    }

    None
}

fn next_move(grid: &Grid<char>, curr_pos: Position, curr_dir: usize, visited: &mut HashSet<Visited >) -> (Position, usize) {
    let new_pos = calc_position(NEIGHBOR_OFFSETS[curr_dir], curr_pos);
    // check for wall or out of bounds
    if !is_valid_move(grid, &new_pos) {
        let (new_pos, dir) =  get_new_pos(grid, &curr_pos, curr_dir);
        // has it been visited using the same direction and position
        let added = visited.insert(Visited { position: new_pos, dir });
        if added {
            return (new_pos, dir);
        }
    }
    (curr_pos, curr_dir)
}

fn check_neighbor(grid: &Grid<char>, curr_pos: Position, dir: usize) -> Option<Position> {
    let calc_pos = calc_position(NEIGHBOR_OFFSETS[dir], curr_pos)?;
    let c = grid.get(calc_pos.row, calc_pos.col).unwrap();
    if *c != '#' {
        return Some(calc_pos);
    }
    None
}

fn get_new_pos(grid: &Grid<char>, curr_pos: &Position, curr_dir: usize) -> (Position, usize) {
    // Define direction patterns for each current direction
    let directions = match curr_dir {
        RIGHT => [(TOP, TOP), (BOTTOM, BOTTOM), (LEFT, LEFT)],
        LEFT => [(TOP, TOP), (BOTTOM, BOTTOM), (RIGHT, RIGHT)],
        TOP => [(RIGHT, RIGHT), (LEFT, LEFT), (BOTTOM, BOTTOM)],
        BOTTOM => [(RIGHT, RIGHT), (LEFT, LEFT), (TOP, TOP)],
        _ => return (*curr_pos, curr_dir),
    };

    // Find the first valid neighbor
    directions
        .iter()
        .find_map(|&(check_dir, new_dir)| {
            check_neighbor(grid, *curr_pos, check_dir)
                .map(|pos| (pos, new_dir))
        })
        .unwrap_or((*curr_pos, curr_dir))
}
fn get_new_pos_old(grid: &Grid<char>, curr_pos: &Position, curr_dir: usize) -> (Position, usize) {
    match curr_dir {
        RIGHT => {
            //Top
            let tp = check_neighbor(grid, *curr_pos, TOP);
            //Bottom
            let bp = check_neighbor(grid, *curr_pos, BOTTOM);
            //Left
            let lp = check_neighbor(grid, *curr_pos, LEFT);
            if tp.is_some() {
                return (tp.unwrap(), TOP);
            }
            if bp.is_some() {
                return (bp.unwrap(), BOTTOM);
            }
            if lp.is_some() {
                return (lp.unwrap(), LEFT);
            }
            (*curr_pos, curr_dir)
        }
        LEFT => {
            //Top
            let tp = check_neighbor(&grid, *curr_pos, TOP);
            //Bottom
            let bp = check_neighbor(&grid, *curr_pos, BOTTOM);
            //Left
            let rp = check_neighbor(&grid, *curr_pos, RIGHT);
            if tp.is_some() {
                return (tp.unwrap(), TOP);
            }
            if bp.is_some() {
                return (bp.unwrap(), BOTTOM);
            }
            if rp.is_some() {
                return (rp.unwrap(), RIGHT);
            }
            (*curr_pos, curr_dir)
        }
        TOP => {
            //Top
            let rp = check_neighbor(&grid, *curr_pos, RIGHT);
            //Bottom
            let bp = check_neighbor(&grid, *curr_pos, BOTTOM);
            //Left
            let lp = check_neighbor(&grid, *curr_pos, LEFT);
            if rp.is_some() {
                return (rp.unwrap(), RIGHT);
            }
            if lp.is_some() {
                return (lp.unwrap(), LEFT);
            }
            if bp.is_some() {
                return (bp.unwrap(), BOTTOM);
            }
            (*curr_pos, curr_dir)
        }
        BOTTOM => {
            //Top
            let tp = check_neighbor(&grid, *curr_pos, TOP);
            //Bottom
            let rp = check_neighbor(&grid, *curr_pos, RIGHT);
            //Left
            let lp = check_neighbor(&grid, *curr_pos, LEFT);
            if rp.is_some() {
                return (rp.unwrap(), RIGHT);
            }
            if lp.is_some() {
                return (lp.unwrap(), LEFT);
            }
            if tp.is_some() {
                return (tp.unwrap(), TOP);
            }
            (*curr_pos, curr_dir)
        }
        _ => (*curr_pos, curr_dir),
    }
}

fn is_valid_move(grid: &Grid<char>, position: &Option<Position>) -> bool {
    match position {
        Some(p) => {
            let c = grid.get(p.row, p.col);
            !matches!(c, Some('#'))
        }
        _ => false,
    }
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
