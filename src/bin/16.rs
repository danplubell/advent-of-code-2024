advent_of_code::solution!(16);

use grid::Grid;
use petgraph::data::Build;
use petgraph::Directed;
use petgraph::Graph;
use petgraph::graph::NodeIndex;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    NoDirection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}
#[derive(Debug, Clone, PartialEq, Default, Copy)]
struct Neighbors {
    top: Option<Position>,
    right: Option<Position>,
    bottom: Option<Position>,
    left: Option<Position>,
}

const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const UP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
    position: Position,
    neighbors: Neighbors,
}
fn calc_position(offset: (i32, i32), position: Position) -> Option<Position> {
    Option::from(Position {
        row: position.row.checked_add_signed(offset.0 as isize)?,
        col: position.col.checked_add_signed(offset.1 as isize)?,
    })
}

fn get_neighbors(curr_position: Position, grid: Grid<char>) -> Neighbors {
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

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph = Graph::<Node, Direction >::new();
    let grid_rows = input.lines().count();
    let grid_cols = input.lines().next().unwrap().len();
     let mut grid: Grid<char> = Grid::new(grid_rows, grid_cols);
    let mut start_location = Position {
        row: 0,
        col: 0,
    };
    let mut end_location = Position {
        row: 0,
        col: 0,
    };  
    // put stuff in grid
    input.lines().enumerate().for_each(|(row, l)| {
        if l.starts_with("#") {
            l.chars().enumerate().for_each(|(col, c)| {
                let g = grid.get_mut(row, col).unwrap();
                if c == 'S' {
                    start_location = Position { row, col }
                }
                if c == 'E' {
                    end_location = Position {row,col}
                }
                *g = c;
            })
        }
    });
    let mut node_indexes:Vec<NodeIndex> = Vec::new();
    // build graph nodes
    for i in 0..grid.rows() {
        println!();
        for j in 0..grid.cols() {
            let c = grid.get(i, j).unwrap();
            let ix = graph.add_node((i,j));
            node_indexes.push(ix);
        }
    }
    

    // add edges
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
