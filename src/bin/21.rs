use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(21);
/*
direction_keys = {
    "^": (0, 1),
    "A": (0, 2),
    "<": (1, 0),
    "v": (1, 1),
    ">": (1, 2)
}

dd = {
    ">": (0, 1),
    "v": (1, 0),
    "<": (0, -1),
    "^": (-1, 0)
}
 */

lazy_static! {
    static ref NUMERIC_KEYS: HashMap<char, (usize, usize)> = {
        HashMap::from([
            ('7', (0, 0)),
            ('8', (0, 1)),
            ('9', (0, 2)),
            ('4', (1, 0)),
            ('5', (1, 1)),
            ('6', (1, 2)),
            ('1', (2, 0)),
            ('2', (2, 1)),
            ('3', (2, 2)),
            ('0', (3, 1)),
            ('A', (3, 2)),
        ])
    };
}
lazy_static! {
    static ref DIRECTION_KEYS: HashMap<char, (usize, usize)> = {
        HashMap::from([
            ('^', (0, 1)),
            ('A', (0, 2)),
            ('<', (1, 0)),
            ('v', (1, 1)),
            ('>', (1, 2)),
        ])
    };
}
lazy_static! {
    static ref DIR_DELTA: HashSet<(char, (i32, i32))> =
        HashSet::from([('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1)), ('^', (-1, 0)),]) ;
}
fn ways(code:&str,keypad: &HashMap<char, (usize, usize)>) -> Vec<&'static str> {
    let parts:Vec<&str> = Vec::new();
    let cur_loc = keypad.get(&'A').unwrap();
    
    for c in code.chars() {
        let next_loc = keypad.get(&c).unwrap();
        let di:isize = (next_loc.0 - cur_loc.0) as isize;
        let dj:isize = (next_loc.1 - cur_loc.1) as isize;
        
        let mut moves:String = "".to_string();
        match di.cmp(&0) {
            Ordering::Greater => {
                moves.push_str(&"v".repeat(di as usize));
            }
            Ordering::Less => {
                if di < 0 {
                    moves.push_str(&"^".repeat((-di) as usize));
                }
            }
            _=>()
        }
        match dj.cmp(&0) {
            Ordering::Greater => {
                moves.push_str(&">".repeat(dj as usize));
            } 
            Ordering::Less => {
                if dj < 0 {
                    moves.push_str(&"^".repeat((-dj) as usize));
                }
            }
        }
    }
    Vec::new()
}
pub fn part_one(input: &str) -> Option<u32> {
    for input_line in input.lines() {}
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
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
