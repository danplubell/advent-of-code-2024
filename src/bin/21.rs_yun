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
    static ref DIR_DELTA: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1)), ('^', (-1, 0))]) ;
}
fn ways(code:&str,keypad: &HashMap<char, (usize, usize)>) -> Vec<&'static str> {
    let mut parts:Vec<String> = Vec::new();
    let mut cur_loc = keypad.get(&'A').unwrap();
    
    for c in code.chars() {
        let next_loc = keypad.get(&c).unwrap();
        let di:isize = next_loc.0 as isize - cur_loc.0 as isize;
        let dj:isize = next_loc.1 as isize - cur_loc.1 as isize;
        
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
            _=>()
        }
        println!("moves: {} {:?}",c, moves);
        let permutations = moves.chars().permutations(moves.len());
        let raw_combos: HashSet<String> = permutations.map(|p|p.iter().collect::<String>() + "A").collect();

        println!("{:?}", raw_combos);
        let mut combos = Vec::new();
        for combo in raw_combos {
            let (ci,cj) = cur_loc;
            let mut good = true;
            let combo_slice = &combo[..combo.len()-1];
            for c in combo_slice.chars() {
                let (di, dj) = DIR_DELTA.get(&c).unwrap();
                let (ci, cj) = (ci + *di as usize , cj + *dj as usize);
                if !keypad.values().contains(&(ci, cj)) {
                    good = false;
                    break;
                };
                if good {
                    combos.push(combo.clone())
                }
            }
        }
        parts.append(&mut combos);
        cur_loc = next_loc;
    }
    println!("parts: {:?}", parts);
    Vec::new()
}
pub fn part_one(input: &str) -> Option<u32> {
    for input_line in input.lines() {
        let p = ways(input_line.trim(), &NUMERIC_KEYS);
        println!("{:?}", p);
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
    fn test_ways() {
        let r = ways("<A^A>^^AvvvA", &DIRECTION_KEYS);

    }
}
