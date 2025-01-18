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
const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const UP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

fn calc_position(offset: (i32, i32), position: Position) -> Option<Position> {
    Option::from(Position {
        row: position.row.checked_add_signed(offset.0 as isize)?,
        col: position.col.checked_add_signed(offset.1 as isize)?,
    })
}

fn make_move(direction: Direction, robot_location: Position, grid: &mut Grid<char>) -> Position {
    let offset = match direction {
        Direction::Up => NEIGHBOR_OFFSETS[UP],
        Direction::Right => NEIGHBOR_OFFSETS[RIGHT],
        Direction::Left => NEIGHBOR_OFFSETS[LEFT],
        Direction::Down => NEIGHBOR_OFFSETS[BOTTOM],
        Direction::NoDirection => (0, 0),
    };
    let next_robot_position = calc_position(offset, robot_location);
    if let Some(p) = next_robot_position {
        let neighbor_value = grid.get(p.row, p.col);
        return match neighbor_value {
            // hit a wall
            Some('#') => robot_location,
            //move boxes if possible
            Some('O') => {
                robot_location
//                move_boxes(grid, offset, p);
//                make_move(direction, robot_location, grid)
            }
            _ => {
                // Now we can do the mutations one at a time, move the robot
                // set the previous location to empty '.'
                if let Some(current) = grid.get_mut(robot_location.row, robot_location.col) {
                    *current = '.';
                }

                // set the robot to the next location
                if let Some(next) = grid.get_mut(p.row, p.col) {
                    *next = '@';
                }
                Position {
                    row: p.row,
                    col: p.col,
                }
            }
        };
    }
    robot_location
}

fn move_boxes(grid: &mut Grid<char>, offset: (i32, i32), position: Position) -> Option<Position> {
    let next_robot_position = calc_position(offset, position);
    if let Some(p) = next_robot_position {
        let neighbor_value = grid.get(p.row, p.col);
        return match neighbor_value {
            Some('.') => {
                // Now we can do the mutations one at a time, move the robot
                // set the previous location to empty '.'
                if let Some(current) = grid.get_mut(position.row, position.col) {
                    *current = '.';
                }

                // set the box to the next location
                if let Some(next) = grid.get_mut(p.row, p.col) {
                    *next = 'O';
                }
                Some(Position {
                    row: position.row,
                    col: position.col,
                }) // return the position that opened up
            }
            Some('O') => {
                // check next box
                let opened_position = move_boxes(grid, offset, p);
                if let Some(current) = opened_position {
                    return move_boxes(grid, offset, position)
                }
                None
            }
            _ => None,
        }
    }
    None
}
/*
match direction {
    Direction::Right => {
        let new_col = robot_location.col.checked_add(1);

        match new_col {
            Some(c) => {
                // First check if we can move by copying the neighbor value
                let neighbor_value = *grid.get(robot_location.row, c).unwrap_or(&'#');

                match neighbor_value {
                    '#'  => robot_location,
                    'O' => {
                        robot_location
                    },
                    _ => {
                        // Now we can do the mutations one at a time
                        if let Some(current) = grid.get_mut(robot_location.row, robot_location.col) {
                            *current = '.';
                        }
                        if let Some(next) = grid.get_mut(robot_location.row, c) {
                            *next = '@';
                        }
                        Position {
                            row: robot_location.row,
                            col: c,
                        }
                    }
                }
            },
            None => robot_location
        }
    }
    Direction::Left => robot_location,
    Direction::Up => robot_location,
    Direction::Down => robot_location,
    Direction::NoDirection => robot_location,
}

 */

/*
fn make_move1(direction: Direction, robot_location: Position, grid: &mut Grid<char>) -> Position {
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

 */
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
        }
    });

    for i in 0..8 {
        println!();
        for j in 0..8 {
            let c = grid.get(i, j).unwrap();
            print!("{}", c);
        }
    }
    input.lines().enumerate().for_each(|(row, l)| {
        if !l.starts_with("#") {
            l.chars().enumerate().for_each(|(col, c)| {
                let direction = match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => Direction::NoDirection,
                };
                println!("robot_location: {:?} {:?}", robot_location, direction);
                robot_location = make_move(direction, robot_location, &mut grid);
                for i in 0..8 {
                    println!();
                    for j in 0..8 {
                        let c = grid.get(i, j).unwrap();
                        print!("{}", c);
                    }
                }
            })
        }
    });
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
    #[test]
    fn test_move_boxes(){
        let mut grid: Grid<char> = Grid::new(4, 4);
        grid.insert_row(0,vec!['#','O','.','#']);
    }
    #[test]
    fn test_make_move(){
        
    }
}
