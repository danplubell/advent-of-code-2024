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
fn find_guard(input: &str) -> Option<Guard> {
    for (i, line) in input.lines().enumerate() {
        let position = line
            .chars()
            .position(|c| c == '^' || c == 'v' || c == '<' || c == '>');
        match position {
            Some(p) => {
                let direction_char = line.chars().nth(p);
                let direction = match direction_char {
                    Some('v') => Some(Direction::Down),
                    Some('<') => Some(Direction::Left),
                    Some('>') => Some(Direction::Right),
                    Some('^') => Some(Direction::Up),
                    _ => return None
                };
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
fn move_guard(guard: &mut Guard, input: &str) -> bool {
    let len = input.lines().count();
    let line_len = input.lines().next().unwrap().len();
    // check move in next direction
    
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
fn move_guard_one(guard: &Guard, input: &str) -> Option<(usize, usize, Direction)> {
    None
}
fn check_move(guard: &Guard, input: &str) -> bool {
    true
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
