advent_of_code::solution!(15);
use grid::*;
use itertools::Itertools;

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

fn move_robot(current_position: Position, next_position: Position, grid: &mut Grid<char>) {
    // Now we can do the mutations one at a time, move the robot
    // set the previous location to empty '.'
    if let Some(current) = grid.get_mut(current_position.row, current_position.col) {
        *current = '.';
    }

    // set the robot to the next location
    if let Some(next) = grid.get_mut(next_position.row, next_position.col) {
        *next = '@';
    }
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
                let open_position = move_boxes(grid, offset, p);
                if open_position.is_some() {
                    move_robot(robot_location, p, grid);
                    Position {
                        row: p.row,
                        col: p.col,
                    }
                } else {
                    robot_location
                }
            }
            Some('.') => {
                move_robot(robot_location, p, grid);
                Position {
                    row: p.row,
                    col: p.col,
                }

                /*
                // Now we can do the mutations one at a time, move the robot
                // set the previous location to empty '.'
                if let Some(current) = grid.get_mut(robot_location.row, robot_location.col) {
                    *current = '.';
                }

                // set the robot to the next location
                if let Some(next) = grid.get_mut(p.row, p.col) {
                    *next = '@';
                }

                 */
            }
            _ => robot_location,
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
                    return move_boxes(grid, offset, position);
                }
                None
            }
            _ => None,
        };
    }
    None
}
pub fn part_one(input: &str) -> Option<usize> {
    let line_length = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(line_length, line_length);
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

    for i in 0..line_length {
        println!();
        for j in 0..line_length {
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
                for i in 0..line_length {
                    println!();
                    for j in 0..line_length {
                        let c = grid.get(i, j).unwrap();
                        print!("{}", c);
                    }
                }
            })
        }
    });
    let mut total = 0;
    for i in 0..line_length {
        println!();
        for j in 0..line_length {
            let c = grid.get(i, j).unwrap();
            if *c == 'O' {
                total += i * 100 + j;
            }
        }
    }

    Some(total)
}

fn move_boxes2(grid: &mut Grid<char>, offset: (i32, i32), position: Position) -> Option<Position> {
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
                let opened_position = crate::move_boxes(grid, offset, p);
                if let Some(current) = opened_position {
                    return crate::move_boxes(grid, offset, position);
                }
                None
            }
            _ => None,
        };
    }
    None
}

fn make_move2(direction: Direction, robot_location: Position, grid: &mut Grid<char>) -> Position {
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
            Some('[') | Some(']')=> {
                return match direction {
                    Direction::Up | Direction::Down => {
                        let open_position = move_boxes_vert(grid, offset, p);
                        if open_position.is_some() {
                            move_robot(robot_location, p, grid);
                            Position {
                                row: p.row,
                                col: p.col,
                            }
                        } else {
                            robot_location
                        }
                    },
                    _ => {
                        let open_position = move_boxes_horz(grid, offset, p);
                        if open_position.is_some() {
                            move_robot(robot_location, p, grid);
                            Position {
                                row: p.row,
                                col: p.col,
                            }
                        } else {
                            robot_location
                        }
                    }
                }
            }
            Some('.') => {
                move_robot(robot_location, p, grid);
                Position {
                    row: p.row,
                    col: p.col,
                }

                /*
                // Now we can do the mutations one at a time, move the robot
                // set the previous location to empty '.'
                if let Some(current) = grid.get_mut(robot_location.row, robot_location.col) {
                    *current = '.';
                }

                // set the robot to the next location
                if let Some(next) = grid.get_mut(p.row, p.col) {
                    *next = '@';
                }

                 */
            }
            _ => robot_location,
        };
    }
    robot_location
}
fn calc_position_horz(offset: (i32, i32), position: Position) -> (Option<Position>,Option<Position>) {
    let p1 = Option::from(Position {
        row: position.row.checked_add_signed(offset.0 as isize)?,
        col: position.col.checked_add_signed(offset.1 as isize)?,
    });
    let p2 = Option::from(Position {
        row: position.row.checked_add_signed(offset.0 as isize)?,
        col: position.col.checked_add_signed((offset.1 * 2) as isize)?,
    });
    (p1,p2)
}

fn move_boxes_horz(grid: &mut Grid<char>, offset: (i32, i32), position: Position) -> Option<Position> {
    let next_box_position = calc_position_horz(offset, position);
    /*
    if let Some(p) = next_box_position {
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
            Some('[') | Some(']') => {
                // check next box
                let opened_position = move_boxes(grid, offset, p);
                if let Some(current) = opened_position {
                    return move_boxes(grid, offset, position);
                }
                None
            }
            _ => None,
        };
    }
    
     */
    None

}

fn move_boxes_vert(grid: &mut Grid<char>, offset: (i32, i32), position: Position) -> Option<Position> {
    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid_width = 0_usize;
    let mut grid_length = 0_usize;
    // expand map
    let new_map: Vec<_> = input
        .lines()
        .filter(|l| l.starts_with("#")).map(|l| {
            let mut new_line = Vec::new();
            l.chars().for_each(|c| match c {
                '@' => {
                    new_line.push('@');
                    new_line.push('.')
                }
                'O'=> {
                    new_line.push('[');
                    new_line.push(']');
                }
                _ => {
                    new_line.push(c);
                    new_line.push(c)
                }
            });
            grid_width = new_line.len();
            new_line
        })
        .collect();
    println!("{:?}", new_map);
    grid_length = new_map.len();
    //find robot location
    let mut robot_location= Position {
        row: 0,
        col: 0,
    };
    new_map.iter().enumerate().for_each(|(row,l)|{
        l.iter().enumerate().for_each(|(col,c)|{
            if *c == '@' {
                robot_location = Position {
                    row, col
                }
            }
        })
    });

    let mut grid: Grid<char> = Grid::new(grid_length, grid_width);
    new_map.iter().enumerate().for_each(|(row,l)|{
        l.iter().enumerate().for_each(|(col,c)|{
           let gc = grid.get_mut(row,col);
           if let Some(p) = gc { *p = *c }
        })
    });
    /*
    println!("robot location: {:?}", robot_location);
    for i in 0..grid.rows() {
        println!();
        for j in 0..grid.cols() {
            let c = grid.get(i, j).unwrap();
            print!("{}",c)
        }
    }
    
     */
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
                println!("Directon: {:?}", direction);
                robot_location = make_move2(direction, robot_location, &mut grid);
                for i in 0..grid.rows() {
                    println!();
                    for j in 0..grid.cols() {
                        let c = grid.get(i, j).unwrap();
                        print!("{}", c);
                    }
                }
            })
        }
    });
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_move_boxes() {
        let mut grid: Grid<char> = Grid::new(4, 4);
        grid.insert_row(0, vec!['#', 'O', '.', '#']);
    }
    #[test]
    fn test_make_move() {}
}
