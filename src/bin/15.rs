advent_of_code::solution!(15);
use grid::*;
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
fn make_move(direction: Direction, robot_location: Position, grid: &mut Grid<char>) -> Position {
    match direction {
        Direction::Right => {
            let current_position = grid.get_mut(robot_location.row, robot_location.col).unwrap();
            let new_col = robot_location.col.checked_sub(1);
            // out of bounds?
            // up against wall?
            return match new_col {
                Some(c) => {
                    let neighbor = grid.get_mut(robot_location.row, c);
                    match neighbor {
                        Some('#') => robot_location,
                        Some('O') => {
                            robot_location
                        }
                        Some(n) => {
                            // move to new location
                            // set neighbor to robot
                            // set old robot to no location
                            *current_position = '.';
                            *n = '@';
                            Position {
                                row: robot_location.row,
                                col: c,
                            }
                        }
                        _ => robot_location
                    }
                },
                None => robot_location
            }
        }
        Direction::Left => {}
        Direction::Up => {}
        Direction::Down => {}
        _ =>    {}
    }
   robot_location
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Grid<char> = Grid::new(8, 8);
    let mut robot_location: Position = Position { row: 0, col: 0 };
    input.lines().enumerate().for_each(|(row, l)| {
        if l.starts_with("#") {
            l.chars().enumerate().for_each(|(col, c)| {
                let g = grid.get_mut(row, col).unwrap();
                if c == '@' {
                    robot_location = Position { row, col }
                }
                *g = c;
            })
        } else {
            // process the moves
            l.chars().enumerate().for_each(|(col, c)| {
                let g = grid.get(row, col).unwrap();
                let direction = match g {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    'V' => Direction::Down,
                    _ => Direction::NoDirection,
                };
                robot_location = make_move(direction, robot_location, &mut grid);
            })
        }
    });
    /*
    for i in 0..8 {
        println!();
        for j in 0..8 {
            let c = grid.get(i,j).unwrap();
            print!("{}", c);
        }
    }

     */
    None
}

fn move_robot(p0: &char, p1: &mut Grid<char>) {
    todo!()
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
