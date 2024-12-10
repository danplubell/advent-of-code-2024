advent_of_code::solution!(6);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Guard {
    row: Option<usize>,
    column: Option<usize>,
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
                    row: Some(i),
                    column: Some(p),
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
    row: Option<Row>,
    col: Option<Col>,
}
type Row = u32;
type Col = u32;
// get the next row and col based on the current direction
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

fn move_guard(guard: Guard, input: &str) -> bool {
    let len = input.lines().count();
    let line_len = input.lines().next().unwrap().len();
    // check move in current_direction
   
    let move_ok = check_move(guard.direction, guard.row, guard.col, input)
    let move_one 
    // if it's a good move
        // move the guard, update the guard and return true
    // get new direction
    // if valid new direction
        // move the guard, update the guard and return true
    // otherwise return false
    false
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut guard: Guard = find_guard(input).unwrap();
    let mut total: u32 = 0;
    loop {
        let move_opt = move_guard(&mut guard, input);
        match move_opt {
            true => { total += 1;}
            false => break,
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
