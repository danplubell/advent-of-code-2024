use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(21);
type KeyPad = Vec<Vec<Option<char>>>;
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
    static ref DIR_DELTA: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1)), ('^', (-1, 0))]);
}
lazy_static! {
    static ref NUM_KEYPAD:KeyPad= {
        vec![
            vec![Some('7'), Some('8'), Some('9')],
            vec![Some('4'), Some('5'), Some('6')],
            vec![Some('1'), Some('2'), Some('3')],
            vec![None, Some('0'), Some('A')],
        ]
    };
}
lazy_static! {
    static ref DIR_PAD: KeyPad= {
        vec![
            vec![None, Some('^'), Some('A')],
            vec![Some('<'), Some('v'), Some('>')],
        ]
    };
}
fn compute_pos(keypad: &KeyPad) -> HashMap<char,(usize, usize)> {
    let mut pos = HashMap::new();
    for r in 0..keypad.len() {
        for c in 0..keypad[r].len() {
            if keypad[r][c].is_some() {
                pos.insert(keypad[r][c].unwrap(), (r,c));
            }
        }
    }
    pos
}
fn compute_seqs(keypad:KeyPad)  {
    
}
pub fn part_one(input: &str) -> Option<u32> {
    for input_line in input.lines() {
    }
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
    #[test]
    fn test_compute_pos() {
        let r = compute_pos(&NUM_KEYPAD);
        println!("{:?}", r);
    }
}
