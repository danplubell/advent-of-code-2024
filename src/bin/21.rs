use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

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
    static ref NUM_KEYPAD: KeyPad = {
        vec![
            vec![Some('7'), Some('8'), Some('9')],
            vec![Some('4'), Some('5'), Some('6')],
            vec![Some('1'), Some('2'), Some('3')],
            vec![None, Some('0'), Some('A')],
        ]
    };
}
lazy_static! {
    static ref DIR_PAD: KeyPad = {
        vec![
            vec![None, Some('^'), Some('A')],
            vec![Some('<'), Some('v'), Some('>')],
        ]
    };
}
fn compute_pos(keypad: &KeyPad) -> HashMap<char, (usize, usize)> {
    let mut pos = HashMap::new();
    for r in 0..keypad.len() {
        for c in 0..keypad[r].len() {
            if keypad[r][c].is_some() {
                pos.insert(keypad[r][c].unwrap(), (r, c));
            }
        }
    }
    pos
}
/*
fn compute_seqs(keypad: &KeyPad) -> HashMap<(&char,&char), Vec<String>> {
    let mut seqs= HashMap::new();
    let pos = compute_pos(keypad);
    for x in pos.keys() {
        for y in pos.keys() {
            if x == y {
                seqs.insert((x, y), vec!["A".to_string()]);
                continue;
            }

            let mut possibilities: Vec<String> = Vec::new();
            let mut optimal = usize::MAX;
            let mut q = VecDeque::new();
            q.push_back((pos[&x], String::new()));
            while let Some(((ru, cu), moves)) = q.pop_front() {
                let r = ru as isize;
                let c = cu as isize;
                for (nr, nc, nm) in [
                    (r - 1, c, "^"),
                    (r + 1, c, "v"),
                    (r, c - 1, "<"),
                    (r, c + 1, ">"),
                ] {
                    // out of bounds
                    if nr < 0
                        || nc < 0
                        || nr >= keypad.len() as isize
                        || nc >= keypad[0].len() as isize
                    {
                        continue;
                    }
                    let v = keypad[nr as usize][nc as usize];
                    if v.is_none() {
                        continue;
                    }
                    let ml = moves.len();
                    if v.unwrap() == *y {
                        if optimal < ml + 1 {
                            continue;
                        }
                        optimal = ml + 1;
                        possibilities.push(moves.clone() + nm + "A");
                    } else {
                        q.push_back(((nc as usize, nr as usize), moves.clone() + nm))
                    }
                }
            }
            seqs.insert((&x.clone(),&y.clone()), possibilities);
        }
    }
    seqs
}


 */

fn compute_seqs_gem(keypad: &Vec<Vec<Option<char>>>) -> HashMap<(char, char), Vec<String>> {
    let mut pos = HashMap::new();
    for r in 0..keypad.len() {
        for c in 0..keypad[r].len() {
            if let Some(val) = keypad[r][c] {
                pos.insert(val, (r, c));
            }
        }
    }

    let mut seqs = HashMap::new();
    for &x in pos.keys() {
        println!("x: {}",x );
        for &y in pos.keys() {
            println!("y: {}",y );
            if x == y {
                seqs.insert((x, y), vec!["A".to_string()]);
                continue;
            }

            let mut possibilities = Vec::new();
            let mut q = VecDeque::new();
            q.push_back((pos[&x], String::new()));
            let mut optimal = f64::INFINITY;

            while !q.is_empty() {
                let ((r, c), moves) = q.pop_front().unwrap();
               
                let neighbors = vec![
                    (r.wrapping_sub(1), c, "^"),
                    (r + 1, c, "v"),
                    (r, c.wrapping_sub(1), "<"),
                    (r, c + 1, ">"),
                ];
                let mut found_target = false;
                for (nr, nc, nm) in neighbors {
                    if nr >= keypad.len() || nc >= keypad[0].len() || keypad[nr][nc].is_none() {
                        continue;
                    }
                    if keypad[nr][nc].is_none() {
                        continue;
                    }
                    if keypad[nr][nc] == Some(y) {
                        if optimal < (moves.len() + 1) as f64 {
                           found_target = true;
                            break;
                        }
                        optimal = (moves.len() + 1) as f64;
                        possibilities.push(format!("{}{}{}", moves, nm, "A"));

                    } else {
                        q.push_back(((nr, nc), format!("{}{}", moves, nm)));
                    }
                }
                if found_target {
                    break;
                }
            }
            seqs.insert((x, y), possibilities);
        }
    }
    seqs
}
fn solve(string: &str, seqs: &HashMap<(char, char), Vec<String>>) -> Vec<String> {
    // Create a vector of options for each character pair
    let options: Vec<&Vec<String>> = std::iter::once('A')
        .chain(string.chars())
        .zip(string.chars())
        .map(|(x, y)| seqs.get(&(x, y)).unwrap())
        .collect();

    // Generate the Cartesian product of all options
    options
        .iter()
        .map(|vec| vec.iter())  // Convert each &Vec<String> to an iterator over &String
        .multi_cartesian_product()
        .map(|product| {
            product
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join("")
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let dir_seqs = compute_seqs_gem(&DIR_PAD);
    for input_line in input.lines() {
        let seqs = compute_seqs_gem(&NUM_KEYPAD);
        let r = solve(input_line, &seqs);
        for _ in 0..2 {
//            let possible_next = Vec::new();
            for seq in r.clone() {
               let result = solve(&seq,&dir_seqs );
                println!("result: {:?}", result)
            }
        }

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
    #[test]
    fn test_compute_seqs() {
        let seqs = compute_seqs_gem(&NUM_KEYPAD);
        println!("{:?}", seqs);
    }
}
