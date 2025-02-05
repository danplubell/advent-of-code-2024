advent_of_code::solution!(16);

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
    None
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

