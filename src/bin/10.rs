use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

fn next_row_col(d: &Direction, row: usize, col: usize) -> (usize, usize) {
    match d {
        Direction::North => (row - 1, col),
        Direction::South => (row + 1, col),
        Direction::East => (row, col + 1),
        Direction::West => (row, col - 1),
    }
}
fn next_step(input: &str, d: u32, row: usize, col: usize, total: u32) -> u32 {
    println!("next step: {d} {row} {col}");
    let mut new_total = total;
    if d == 9 {
        new_total += 1;
        return new_total;
    };
    let directions: Vec<Direction> = check_directions(d, input, row, col);
    if !directions.is_empty() {
        directions.iter().for_each(|dir| {
            let next_r_c = next_row_col(dir, row, col);
            new_total = next_step(input, d + 1, next_r_c.0, next_r_c.1, new_total);
        })
    }
    new_total
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    input.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, ch)| {
            let d = ch.to_digit(10).unwrap();
            if d == 0 {
                let destinations:HashSet<(usize,usize)> = HashSet::new();
                total = next_step(input, d, row, col, total);
            }
        })
    });
    Some(total)
}
fn check_directions(value: u32, input: &str, row: usize, col: usize) -> Vec<Direction> {
    
    let mut directions: Vec<Direction> = vec![];
    let sr = input.lines().nth(row + 1);
    let ec = input.lines().nth(row).unwrap().chars().nth(col + 1);
    let next_row_nbr = row.checked_sub(1);
    if let Some(nrn) = next_row_nbr {
        let nr = input.lines().nth(nrn);
        if let Some(nv) = nr {
            let cv = nv.chars().nth(col);
            if let Some(c) = cv {
                let v = c.to_digit(10).unwrap();
                let diff = v.checked_sub(value);
                if let Some(nv) = diff {
                    if nv == 1 {
                        directions.push(Direction::North);
                        
                    }                    
                }
            }
        }
    }
    if let Some(nv) = sr {
        let cv = nv.chars().nth(col);
        if let Some(c) = cv {
            let v = c.to_digit(10).unwrap();
            let diff = v.checked_sub(value);
            if let Some(nv) = diff {
                if nv == 1 {
                    directions.push(Direction::South);
                }
            }
        }
    }
    if let Some(nv) = ec {
        let v = nv.to_digit(10).unwrap();
        let diff = v.checked_sub(value);
        if let Some(nv) = diff {
            if nv == 1{
                directions.push(Direction::East);
            }
        }
    }    
    let next_col_nbr = col.checked_sub(1);
    if let Some(ncn) =next_col_nbr {
        
        let wc = input.lines().nth(row).unwrap().chars().nth(ncn);
        if let Some(nv) = wc {
            let v = nv.to_digit(10).unwrap();
            let diff = v.checked_sub(value);
            if let Some(nv) = diff {
                if nv == 1 {
                    directions.push(Direction::West);
                }
            }
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
