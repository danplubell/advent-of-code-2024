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
        _ => None
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
    char == '#'
}

#[derive(Debug,PartialEq, Clone, Copy)]struct Move {
    row: Row,
    col: Col,
}
type Row = u32;
type Col = u32;
fn get_next_move(direction: Option<Direction>, curr_row: Option<Row>, curr_col: Option<Col>) -> Option<Move> {
    match (direction, curr_row, curr_col) {
        (Some(d), Some(r), Some(c)) => {
            let (new_r, new_c) = match d {
                Direction::Up => (r.checked_sub(1), Some(c)),
                Direction::Down => (r.checked_add(1), Some(c)),
                Direction::Left => (Some(r), c.checked_sub(1)),
                Direction::Right => (Some(r), c.checked_add(1)),
            };
            Some(Move{
                row: new_r?,
                col: new_c?
            })
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
            if char.is_some() {
                if !is_obstacle(char.unwrap()) {
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
    let mut guard: Guard = find_guard(input).unwrap();
    let mut total: u32 = 0;
    loop {
        let result = get_next_move(guard.direction,guard.row, guard.column).filter(|m| can_move_there(m, input)).map(|m| {
            total += 1;
            // update the guard with the new row and column, same direction
            guard = Guard {
                row: Some(m.row),
                column: Some(m.col),
                direction: guard.direction
            };
        });
        if result.is_none() {
            let new_direction = new_direction(&guard.direction);
            println!("next move failed : {:?}", new_direction);
            // update the guard with the same row, column, but new direction
            guard = Guard {
                row: guard.row,
                column: guard.column,
                direction: Some(new_direction)
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
pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
