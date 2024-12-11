advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct Guard {
    row: Option<Row>,
    column: Option<Col>,
    direction: Option<Direction>,
}
fn get_direction_from_char(char: Option<char>) -> Option<Direction> {
    match char {
        Some('v') => Some(Direction::Down),
        Some('<') => Some(Direction::Left),
        Some('>') => Some(Direction::Right),
        Some('^') => Some(Direction::Up),
        _ => None,
    }
}
fn find_guard(input: &str) -> Option<Guard> {
    for (i, line) in input.lines().enumerate() {
        let position = line
            .chars()
            .position(|c| c == '^' || c == 'v' || c == '<' || c == '>');
        match position {
            Some(p) => {
                let direction_char = line.chars().nth(p);
                let direction = get_direction_from_char(direction_char);
                return Some(Guard {
                    row: Some(i as Row),
                    column: Some(p as Col),
                    direction,
                });
            }
            None => continue,
        }
    }
    None
}
fn is_obstacle(char: char) -> bool {
    char == '#' || char == 'o'
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Move {
    row: Row,
    col: Col,
}
type Row = u32;
type Col = u32;
fn get_next_move(
    direction: Option<Direction>,
    curr_row: Option<Row>,
    curr_col: Option<Col>,
    max_row: usize,
    max_col: usize,
) -> Option<Move> {
    match (direction, curr_row, curr_col) {
        (Some(d), Some(r), Some(c)) => {
            let (new_r, new_c) = match d {
                Direction::Up => (r.checked_sub(1), Some(c)),
                Direction::Down => (r.checked_add(1), Some(c)),
                Direction::Left => (Some(r), c.checked_sub(1)),
                Direction::Right => (Some(r), c.checked_add(1)),
            };
            if new_r? < max_row as u32 && new_c? < max_col as u32 {
                Some(Move {
                    row: new_r?,
                    col: new_c?,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
fn can_move_there(next_move: &Move, input: &str) -> Option<bool> {
    //check for out of bounds
    //check for obstacle

    let (r, c) = (next_move.row, next_move.col);
    {
        let new_row = input.lines().nth(r as usize);
        if new_row.is_some() {
            let char = new_row.unwrap().chars().nth(c as usize);
            if let Some(c) = char {
                if !is_obstacle(c) {
                    Some(true)
                } else {
                    Some(false)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_row = input.lines().count();
    let max_col = input.lines().next().unwrap().chars().count();
    let mut guard: Guard = find_guard(input).unwrap();
    let mut moves: Vec<(Row, Col)> = vec![];
    let mut total: u32 = 1;
    moves.push((guard.row.unwrap(), guard.column.unwrap()));
    loop {
        let next_move = get_next_move(guard.direction, guard.row, guard.column, max_row, max_col);
        if next_move.is_none() {
            break Some(total);
        }
        let can_move = can_move_there(&next_move.unwrap(), input).unwrap();
        if can_move {
            let new_move = (next_move.unwrap().row, next_move.unwrap().col);
            // did the guard hit that spot already
            let found = moves
                .iter()
                .find(|m| m.0 == new_move.0 && m.1 == new_move.1);
            if found.is_none() {
                total += 1;
                moves.push(new_move);
            }
            guard = Guard {
                row: Some(next_move.unwrap().row),
                column: Some(next_move.unwrap().col),
                direction: guard.direction,
            };
        } else {
            let new_direction = new_direction(&guard.direction);
            // update the guard with the same row, column, but new direction
            guard = Guard {
                row: guard.row,
                column: guard.column,
                direction: Some(new_direction),
            }
        }
    }
}

fn new_direction(direction: &Option<Direction>) -> Direction {
    let d = direction.as_ref().unwrap();
    match d {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}
fn place_obstacle(input: &str, row: usize, col: usize) -> String {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| if i == row && j == col { 'o' } else { c })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
pub fn part_two(input: &str) -> Option<u32> {
    let max_row = input.lines().count();
    let max_col = input.lines().next().unwrap().chars().count();
    let mut guard: Guard = find_guard(input).unwrap();
    let mut total: u32 = 0;
    let mut moves: Vec<(Row, Col)> = vec![];
    println!("max: {} {}", max_row, max_col);
    'main: for (i, r) in input.lines().enumerate() {
        for (j, c) in r.chars().enumerate() {
            println!("cycle: {} {}",i,j);
            let new_input = place_obstacle(input, i, j);
            guard = find_guard(input).unwrap();
            moves = vec![];
            loop {
                let next_move =
                    get_next_move(guard.direction, guard.row, guard.column, max_row, max_col);
                if next_move.is_none() {
                    break;
                }
                let can_move = can_move_there(&next_move.unwrap(), &new_input).unwrap();
                if can_move {
                    let new_move = (next_move.unwrap().row, next_move.unwrap().col);
                    let visited = moves.iter().find(|m| m.0==new_move.0 && m.1==new_move.1);
                    if visited.is_some() {
                       let visits =  moves.iter().filter(|m| m.0==new_move.0 && m.1==new_move.1).count();
                        if visits > 4 {
                            total += 1;
                            break;
                        }
                    }
                    moves.push(new_move);
                    guard = Guard {
                        row: Some(next_move.unwrap().row),
                        column: Some(next_move.unwrap().col),
                        direction: guard.direction,
                    };
                } else {
                    let new_direction = new_direction(&guard.direction);
                    // update the guard with the same row, column, but new direction
                    guard = Guard {
                        row: guard.row,
                        column: guard.column,
                        direction: Some(new_direction),
                    }
                }
            }
        }
    }
    Some(total)
    /*
    let max_row = input.lines().count() ;
    let max_col = input.lines().next().unwrap().chars().count();
    let mut guard: Guard = find_guard(input).unwrap();
    let mut moves: Vec<(Row,Col)> = vec![];
    let mut total: u32 = 1;
    moves.push((guard.row.unwrap(), guard.column.unwrap()));
    loop {
        let next_move = get_next_move(guard.direction,guard.row, guard.column,max_row, max_col);
        if next_move.is_none() {
            break Some(total)
        }
        let can_move = can_move_there(&next_move.unwrap(), input).unwrap();
        if can_move {
            let new_move = (next_move.unwrap().row, next_move.unwrap().col);
            // did the guard hit that spot already
            let found = moves.iter().find(|m| m.0 == new_move.0 && m.1 == new_move.1);
            if found.is_none() {
                total +=1;
                moves.push(new_move);
            }
            guard = Guard {
                row: Some(next_move.unwrap().row),
                column: Some(next_move.unwrap().col),
                direction: guard.direction
            };

        } else {
            let new_direction = new_direction(&guard.direction);
            // update the guard with the same row, column, but new direction
            guard = Guard {
                row: guard.row,
                column: guard.column,
                direction: Some(new_direction)
            }
        }
    }


     */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_get_next_move() {
        let r = get_next_move(Some(Direction::Up), Some(0), Some(1), 5, 5);
        assert!(r.is_none());
        let r = get_next_move(Some(Direction::Down), Some(3), Some(0), 5, 5);
        assert_eq!(
            r,
            Some(Move {
                row: 4 as Row,
                col: 0 as Row,
            })
        );
        let r = get_next_move(Some(Direction::Down), Some(5), Some(0), 5, 5);
        assert!(r.is_none());
    }
}
