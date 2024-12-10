advent_of_code::solution!(6);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Guard {
    row: Option<Row>,
    column: Option<Col>,
    direction: Option<Direction>,
}
fn get_direction_from_char(char: Option<char>) -> Option<Direction>) {
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

struct Move {
    row: Row,
    col: Col,
}
type Row = u32;
type Col = u32;
// get the next row and col based on the current direction
/*
fn next_row_col(direction: Option<Direction>, row: Option<Row>, col: Option<Col>, input: &str) -> Option<Move>{
    match (direction, row, col) {
        (Some(direction), Some(r), Some(c)) => {
            Direction::Up => Move {row: r.checked_sub(1), col: Some(c)}}//(r.checked_sub(1), Some(c)),
            /*
            Direction::Down => (r.checked_add(1), Some(c)),
            Direction::Left => (Some(r), c.checked_sub(1)),
            Direction::Right => (Some(r), c.checked_add(1)),
            
             */
            _ => ()
        }
        _ => ()
    }
}
fn get_next_move(direction: Option<Direction>, row: Option<usize>, col: Option<usize>, input: &str) -> Option<Move> {
    let (r,c ) = match (direction, row, col) {
        
        // get new row and column
        (Some(direction), Some(r), Some(c)) => {
            let (new_r, new_c) = match direction {
                Direction::Up => (r.checked_sub(1), Some(c)),
                Direction::Down => (r.checked_add(1), Some(c)),
                Direction::Left => (Some(r), c.checked_sub(1)),
                Direction::Right => (Some(r), c.checked_add(1)),
            };
            match (new_r, new_c) {
                (Some(r), Some(c)) => {
                    if let Some(row) = input.lines().nth(r) {
                        if row.chars().nth(c).is_some() {
                            (new_r, new_c)
                        } else {
                            (None, None)
                        }
                    }
                    (None, None)
                }
                _ => (None, None)
            }
        }
        _=> (None, None)
    };
    Some(Move { row: r, col: c})
}
fn check_move(direction: Option<Direction>, row: Option<usize>, col: Option<usize>, input: &str) -> bool {
   match (direction, row, col) {
       (Some(direction), Some(r), Some(c)) => {
           let (new_r, new_c) = match direction {
               Direction::Up => (r.checked_sub(1), Some(c)),
               Direction::Down => (r.checked_add(1), Some(c)),
               Direction::Left => (Some(r), c.checked_sub(1)),
               Direction::Right => (Some(r), c.checked_add(1)),
           };
           match (new_r, new_c) {
               (Some(r), Some(c)) => {
                   if let Some(row) = input.lines().nth(r) {
                       if row.chars().nth(c).is_some() {
                           return true
                       } else {
                           return false
                       }
                   }
               }
               _ => return false
           }
       }
       _ => return false
   }
    false
}


 */
fn get_next_move(guard: &Guard, input: &str) -> Option<Move> {
    None
}
fn can_move_there(next_move: &Move, input: &str) -> bool {
    true
}
fn move_guard(guard: &mut Guard, input: &str) -> (bool, Option<Guard>) {
    let len = input.lines().count();
    let line_len = input.lines().next().unwrap().len();
    let next_move = get_next_move(&guard, input);
    next_move.filter(|m| can_move_there(m, input)).map(|m|{
                     let new_guard = Guard {
            direction: *guard.direction,
            row: Some(m.row),
            column: Some(m.col),
        };
        (true, Some(new_guard))

    }).unwrap_or((false, None))

/*    if let Some(ref move_pos) = next_move {
        if can_move_there(&next_move, input: &str) {
            let new_guard = Guard {
                direction: *guard.direction,
                row: Some(move_pos.row),
                column: Some(move_pos.col),
            };
            return (true, Some(new_guard));
        }
    }


 */
    //(false, None)
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut guard: Guard = find_guard(input).unwrap();
    let mut total: u32 = 0;
    loop {
        let move_opt = move_guard(&mut guard, input);
//        match move_opt {
//            true => { total += 1;}
//            false => break,
        }
    }
    None
}
fn new_direction(guard: &Guard, input: &str) -> Option<Direction> {
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
