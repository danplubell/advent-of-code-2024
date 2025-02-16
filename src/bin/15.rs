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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Box {
    left: Position,
    right: Position,
}
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

fn make_move2(direction: Direction, robot_location: Position, grid: &mut Grid<char>) -> Option<Position> {
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
            Some('#') => Some(robot_location),
            Some('.') => {
                move_robot(robot_location, p, grid);
                Some(Position {
                    row: p.row,
                    col: p.col,
                })
            }
            //move boxes if possible
            Some(b) => {
                if *b == '[' || *b == ']' {
                    return match direction {
                        Direction::Up | Direction::Down => {
                            let open_position = move_boxes_vert(grid, offset, p);
                            if open_position.is_some() {
                                move_robot(robot_location, p, grid);
                                Some(Position {
                                    row: p.row,
                                    col: p.col,
                                })
                            } else {
                                None
                            }
                        }
                        _ => {
                            let open_position = move_boxes_horz(grid, offset, p);
                            if open_position.is_some() {
                                move_robot(robot_location, p, grid);
                                Some(Position {
                                    row: p.row,
                                    col: p.col,
                                })
                            } else {
                                None
                            }
                        }
                    };
                }
                None
            }
            _ => None,
        };
    }
    None
}

fn calc_position_horz(
    offset: (i32, i32),
    position: Position,
) -> (Option<Position>, Option<Position>) {
    let p1 = position
        .row
        .checked_add_signed(offset.0 as isize)
        .and_then(|row| {
            position
                .col
                .checked_add_signed(offset.1 as isize)
                .map(|col| Position { row, col })
        });
    let p2 = position
        .row
        .checked_add_signed(offset.0 as isize)
        .and_then(|row| {
            position
                .col
                .checked_add_signed((offset.1 * 2) as isize)
                .map(|col| Position { row, col })
        });
    (p1, p2)
}

fn move_box(grid: &mut Grid<char>, current_position: Box, new_position: Box) {
    // set old position to .
    if let Some(row) = grid.get_mut(current_position.right.row, current_position.right.col) {
        *row = '.';
    }
    if let Some(col) = grid.get_mut(current_position.left.row, current_position.left.col) {
        *col = '.';
    }
    if let Some(row) = grid.get_mut(new_position.right.row, new_position.right.col) {
        *row = ']';
    }
    if let Some(col) = grid.get_mut(new_position.left.row, new_position.left.col) {
        *col = '[';
    }
}
fn move_boxes_horz(
    grid: &mut Grid<char>,
    offset: (i32, i32),
    position: Position,
) -> Option<Position> {
    let next_box_position = calc_position_horz(offset, position);
    if let (Some(l), Some(r)) = next_box_position {
        let neighbor_value = grid.get(r.row, r.col);
        return match neighbor_value {
            Some('.') => match offset {
                (0, 1) => {
                    let next_box = Box {
                        left: Position {
                            row: l.row,
                            col: l.col,
                        },
                        right: Position {
                            row: r.row,
                            col: r.col,
                        },
                    };
                    let curr_box = Box {
                        left: position,
                        right: Position {
                            row: position.row,
                            col: position.col,
                        },
                    };
                    move_box(grid, curr_box, next_box);
                    Some(Position {
                        row: l.row,
                        col: l.col,
                    })
                }
                (0, -1) => {
                    let next_box = Box {
                        left: Position {
                            row: r.row,
                            col: r.col,
                        },
                        right: Position {
                            row: l.row,
                            col: l.col,
                        },
                    };
                    let curr_box = Box {
                        left: position,
                        right: Position {
                            row: position.row,
                            col: position.col,
                        },
                    };
                    move_box(grid, curr_box, next_box);
                    Some(Position {
                        row: l.row,
                        col: l.col,
                    })
                }
                _ => Some(position),
            },
            // another box
            Some('[') | Some(']') => {
                let new_location = move_boxes_horz(
                    grid,
                    offset,
                    Position {
                        row: r.row,
                        col: r.col,
                    },
                );
                if let Some(np) = new_location {
                    return move_boxes_horz(grid, offset, position);
                }
                None
            }
            _ => None,
        };
    }
    None
}
fn get_box(grid: &Grid<char>, position: Position) -> Option<Box> {
    let char_at_pos = grid.get(position.row, position.col).unwrap();

    let new_box = match char_at_pos {
        ']' => Some(Box {
            left: Position {
                row: position.row,
                col: position.col - 1,
            },
            right: position,
        }),
        '[' => Some(Box {
            left: position,
            right: Position {
                row: position.row,
                col: position.col + 1,
            },
        }),
        _ => None,
    };
    new_box
}
fn move_boxes_vert(
    grid: &mut Grid<char>,
    offset: (i32, i32),
    position: Position,
) -> Option<Position> {
    //[]
    // can both characters move?
    // get the current box based on the position []
    // this is the box that robot was going to run into
    let curr_box = get_box(grid, position);
    if let Some(cb) = curr_box {
        // columns are the same
        // adjust the rows based on next_position
        let next_position = calc_position(offset, position);
        // build the next box based off the position to move
        if let Some(p) = next_position {
            // check to see if the next row is open for left char
            // check to see if the next row is open for right char
            let chl = grid.get(p.row, cb.left.col);
            let chr = grid.get(p.row, cb.right.col);
            return match (chl, chr) {
                (Some(']'), Some('[')) => {
                    //touching two boxes, one on left edge and one on right edge
                    let left = move_boxes_vert(
                        grid,
                        offset,
                        Position {
                            row: p.row,
                            col: cb.left.col,
                        },
                    );
                    let right = move_boxes_vert(
                        grid,
                        offset,
                        Position {
                            row: p.row,
                            col: cb.right.col,
                        },
                    );
                    if left.is_some() && right.is_some() {
                        //based on new position create new box position
                        let new_box = Box {
                            left: Position {
                                row: p.row,
                                col: cb.left.col,
                            },
                            right: Position {
                                row: p.row,
                                col: cb.right.col,
                            },
                        };
                        move_box(grid, curr_box?, new_box);
                        Some(p)
                    } else {
                        None
                    }
                }
                (Some('['), Some(']')) => {
                    //touching one box
                    let pos = move_boxes_vert(grid, offset, p);
                    if let Some(new_p) = pos {
                        let new_box = Box {
                            left: Position {
                                row: p.row,
                                col: cb.left.col,
                            },
                            right: Position {
                                row: p.row,
                                col: cb.right.col,
                            },
                        };
                        move_box(grid, curr_box?, new_box);
                        return Some(p)
                    }
                    None
                }
                (Some('.'), Some('[')) => {
                    //Touching right box on left edge of other
                    let pos = move_boxes_vert(
                        grid,
                        offset,
                        Position {
                            row: p.row,
                            col: cb.right.col,
                        },
                    );
                    if let Some(new_pos) = pos {
                        let new_box = Box {
                            left: Position {
                                row: p.row,
                                col: cb.left.col,
                            },
                            right: Position {
                                row: p.row,
                                col: cb.right.col,
                            },
                        };
                        move_box(grid, curr_box?, new_box);
                        return Some(p)

                    }
                    None
                }
                (Some(']'), Some('.')) => {
                    //Touching left box on right edge of a box
                    let pos = move_boxes_vert(
                        grid,
                        offset,
                        Position {
                            row: p.row,
                            col: cb.left.col,
                        },
                    );
                    if let Some(new_pos) = pos {
                        let new_box = Box {
                            left: Position {
                                row: p.row,
                                col: cb.left.col,
                            },
                            right: Position {
                                row: p.row,
                                col: cb.right.col,
                            },
                        };
                        move_box(grid, curr_box?, new_box);
                        return Some(p)

                    }
                    None
                }
                (Some('.'), Some('.')) => {
                    //based on new position create new box position
                    let new_box = Box {
                        left: Position {
                            row: p.row,
                            col: cb.left.col,
                        },
                        right: Position {
                            row: p.row,
                            col: cb.right.col,
                        },
                    };
                    move_box(grid, curr_box?, new_box);
                    Some(p)
                }
                _ => None,
            };
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid_width = 0_usize;
    let mut grid_length = 0_usize;
    // expand map
    let new_map: Vec<_> = input
        .lines()
        .filter(|l| l.starts_with("#"))
        .map(|l| {
            let mut new_line = Vec::new();
            l.chars().for_each(|c| match c {
                '@' => {
                    new_line.push('@');
                    new_line.push('.')
                }
                'O' => {
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
//    println!("{:?}", new_map);
    grid_length = new_map.len();
    //find robot location
    let mut robot_location = Position { row: 0, col: 0 };
    new_map.iter().enumerate().for_each(|(row, l)| {
        l.iter().enumerate().for_each(|(col, c)| {
            if *c == '@' {
                robot_location = Position { row, col }
            }
        })
    });

    let mut grid: Grid<char> = Grid::new(grid_length, grid_width);
    new_map.iter().enumerate().for_each(|(row, l)| {
        l.iter().enumerate().for_each(|(col, c)| {
            let gc = grid.get_mut(row, col);
            if let Some(p) = gc {
                *p = *c
            }
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
//                println!("Directon: {:?}", direction);
                let save_grid = grid.clone();
                let new_robot_location = make_move2(direction, robot_location, &mut grid);
                match new_robot_location {
                    Some(new_pos) => robot_location = new_pos,
                    _=> grid = save_grid.clone(),
                }
            })
        }
    });
    let mut total = 0;
    for i in 0..grid.rows() {
        println!();
        for j in 0..grid.cols() {
            let c = grid.get(i, j).unwrap();
            if *c == '[' {
                total += i * 100 + j
            }
        }
    }

    Some(total as u32)
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
        assert_eq!(result, Some(9021));
    }
    #[test]
    fn test_move_boxes() {
        let mut grid: Grid<char> = Grid::new(4, 4);
        grid.insert_row(0, vec!['#', 'O', '.', '#']);
    }
    #[test]
    fn test_move_box() {
        let mut grid: Grid<char> = Grid::new(7, 7);
        grid.insert_row(0, vec!['#', '#', '[', ']', '.', '#', '#']);
        let curr_pos = Box {
            left: Position { row: 0, col: 2 },
            right: Position { row: 0, col: 3 },
        };
        let new_pos = Box {
            left: Position { row: 0, col: 3 },
            right: Position { row: 0, col: 4 },
        };
        move_box(&mut grid, curr_pos, new_pos);
        println!("{:?}", grid);
    }
    #[test]
    fn test_calc_pos_horz() {
        let p = calc_position_horz((0, 1), Position { row: 0, col: 1 });
        println!("{:?}", p);
        let p = calc_position_horz((0, -1), Position { row: 0, col: 2 });
        println!("{:?}", p);
    }

    #[test]
    fn test_move_boxes_horz() {
        let mut grid: Grid<char> = Grid::new(1, 9);
        grid.insert_row(0, vec!['#', '#', '[', ']', '[', ']', '.', '#', '#']);
        move_boxes_horz(&mut grid, (0, 1), Position { row: 0, col: 2 });
        println!("{:?}", grid);

        let mut grid: Grid<char> = Grid::new(1, 9);
        grid.insert_row(0, vec!['#', '#', '.', '[', ']', '[', ']', '#', '#']);
        move_boxes_horz(&mut grid, (0, -1), Position { row: 0, col: 6 });
        println!("{:?}", grid);
    }
    #[test]
    fn test_move_boxes_horz_left() {
        let mut grid: Grid<char> = Grid::new(1, 9);
        grid.insert_row(0, vec!['#', '#', '.', '[', ']', '[', ']', '#', '#']);
        move_boxes_horz(&mut grid, (0, -1), Position { row: 0, col: 6 });
        println!("{:?}", grid);
    }
    #[test]
    fn test_move_box_up_left() {
        let mut grid: Grid<char> = Grid::new(2, 4);
        grid.insert_row(0, vec!['#', '.', '.', '#']);
        grid.insert_row(1, vec!['#', '[', ']', '#']);
        move_boxes_vert(&mut grid, (-1, 0), Position { row: 1, col: 1 });
        println!("{:?}", grid);
    }
    #[test]
    fn test_move_box_up_right() {
        let mut grid: Grid<char> = Grid::new(2, 4);
        grid.insert_row(0, vec!['#', '.', '.', '#']);
        grid.insert_row(1, vec!['#', '[', ']', '#']);
        move_boxes_vert(&mut grid, (-1, 0), Position { row: 1, col: 2 });
        println!("{:?}", grid);
    }
    #[test]
    fn test_move_box_down_left() {
        let mut grid: Grid<char> = Grid::new(2, 4);
        grid.insert_row(0, vec!['#', '[', ']', '#']);
        grid.insert_row(1, vec!['#', '.', '.', '#']);
        move_boxes_vert(&mut grid, (1, 0), Position { row: 0, col: 1 });
        println!("{:?}", grid);
    }
    #[test]
    fn test_move_box_down_right() {
        let mut grid: Grid<char> = Grid::new(2, 4);
        grid.insert_row(0, vec!['#', '[', ']', '#']);
        grid.insert_row(1, vec!['#', '.', '.', '#']);
        move_boxes_vert(&mut grid, (1, 0), Position { row: 0, col: 2 });
        println!("{:?}", grid);
    }
    #[test]
    fn test_get_box() {
        let mut grid: Grid<char> = Grid::new(2, 4);
        grid.insert_row(0, vec!['#', '.', '.', '#']);
        grid.insert_row(1, vec!['#', '[', ']', '#']);
        let new_box = get_box(&grid, Position { row: 1, col: 1 });
        println!("{:?}", new_box);
        let new_box = get_box(&grid, Position { row: 1, col: 2 });
        println!("{:?}", new_box);
    }
    #[test]
    fn test_move_box_2_up_left() {
        let mut grid: Grid<char> = Grid::new(4, 8);
        grid.insert_row(0, vec!['#', '.', '.', '.', '.', '.', '.', '#']);
        grid.insert_row(1, vec!['#', '[', ']', '[', ']', '[', ']', '#']);
        grid.insert_row(2, vec!['#', '.', '[', ']', '[', ']', '.', '#']);
        grid.insert_row(3, vec!['#', '.', '.', '[', ']', '.', '.', '#']);

        move_boxes_vert(&mut grid, (-1, 0), Position { row: 3, col: 3 });
        for i in 0..grid.rows() {
            println!();
            for j in 0..grid.cols() {
                let c = grid.get(i, j).unwrap();
                print!("{}", c);
            }
        }
    }
    #[test]
    fn test_move_box_2_up_right() {
        let mut grid: Grid<char> = Grid::new(4, 8);
        grid.insert_row(0, vec!['#', '.', '.', '.', '.', '.', '.', '#']);
        grid.insert_row(1, vec!['#', '[', ']', '[', ']', '[', ']', '#']);
        grid.insert_row(2, vec!['#', '.', '[', ']', '[', ']', '.', '#']);
        grid.insert_row(3, vec!['#', '.', '.', '[', ']', '.', '.', '#']);

        move_boxes_vert(&mut grid, (-1, 0), Position { row: 3, col: 4 });
        for i in 0..grid.rows() {
            println!();
            for j in 0..grid.cols() {
                let c = grid.get(i, j).unwrap();
                print!("{}", c);
            }
        }
    }
    #[test]
    fn test_move_box_2_down_left() {
        let mut grid: Grid<char> = Grid::new(4, 8);
        grid.insert_row(0, vec!['#', '.', '.', '[', ']', '.', '.', '#']);
        grid.insert_row(1, vec!['#', '.', '[', ']', '[', ']', '.', '#']);
        grid.insert_row(2, vec!['#', '[', ']', '[', ']', '[', ']', '#']);
        grid.insert_row(3, vec!['#', '.', '.', '.', '.', '.', '.', '#']);

        move_boxes_vert(&mut grid, (1, 0), Position { row: 0, col: 3 });
        for i in 0..grid.rows() {
            println!();
            for j in 0..grid.cols() {
                let c = grid.get(i, j).unwrap();
                print!("{}", c);
            }
        }
    }
    #[test]
    fn test_move_box_2_down_right() {
        let mut grid: Grid<char> = Grid::new(4, 8);
        grid.insert_row(0, vec!['#', '.', '.', '[', ']', '.', '.', '#']);
        grid.insert_row(1, vec!['#', '.', '[', ']', '[', ']', '.', '#']);
        grid.insert_row(2, vec!['#', '[', ']', '[', ']', '[', ']', '#']);
        grid.insert_row(3, vec!['#', '.', '.', '.', '.', '.', '.', '#']);

        move_boxes_vert(&mut grid, (1, 0), Position { row: 0, col: 4 });
        for i in 0..grid.rows() {
            println!();
            for j in 0..grid.cols() {
                let c = grid.get(i, j).unwrap();
                print!("{}", c);
            }
        }
    }
    #[test]
    fn test_move_box_2_down_right_blocked() {
        let mut grid: Grid<char> = Grid::new(4, 8);
        grid.insert_row(0, vec!['#', '.', '.', '[', ']', '.', '.', '#']);
        grid.insert_row(1, vec!['#', '.', '[', ']', '[', ']', '.', '#']);
        grid.insert_row(2, vec!['#', '[', ']', '[', ']', '[', ']', '#']);
        grid.insert_row(3, vec!['#', '.', '.', '.', '#', '.', '.', '#']);

        assert_eq!(move_boxes_vert(&mut grid, (1, 0), Position { row: 0, col: 4 }), None);
        for i in 0..grid.rows() {
            println!();
            for j in 0..grid.cols() {
                let c = grid.get(i, j).unwrap();
                print!("{}", c);
            }
        }
    }
}
