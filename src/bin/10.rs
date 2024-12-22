advent_of_code::solution!(10);

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    directions: Vec<Direction>,
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, ch)| {
            let d = ch.to_digit(10).unwrap();
            if d == 0 {
                let directions: Vec<Direction> = check_directions(d, input, row, col);
            }
        })
    });
    None
}
fn check_directions(value: u32,  input: &str, row: usize, col: usize) -> Vec<Direction> {
    let nr = input.lines().nth(row - 1);
    let sr = input.lines().nth(row + 1);
    let ec = input.lines().nth(row).unwrap().chars().nth(col + 1);
    let wc = input.lines().nth(row).unwrap().chars().nth(col - 1);
    let mut directions:Vec<Direction> = vec![];
    if let Some(nv) = nr {
        let cv = nv.chars().nth(col);
        if let Some(c) = cv {
            let v = c.to_digit(10).unwrap();
            if v - value == 1 {
                directions.push(Direction::North);
            }
        }
    }
    if let Some(nv) = sr {
        let cv = nv.chars().nth(col);
        if let Some(c) = cv {
            let v = c.to_digit(10).unwrap();
            if v - value == 1 {
                directions.push(Direction::South);
            }
        }
    }
    if let Some(nv) = ec {
        let v = nv.to_digit(10).unwrap();
        if v - value == 1 {
            directions.push(Direction::East);
        }
    }
    if let Some(nv) = ec {
        let v = nv.to_digit(10).unwrap();
        if v - value == 1 {
            directions.push(Direction::West);
        }
    }
    directions
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
