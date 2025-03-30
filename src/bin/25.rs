use itertools::Itertools;

advent_of_code::solution!(25);
const KEY: &str = ".....";
const LOCK: &str = "#####";
pub fn part_one(input: &str) -> Option<u32> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for mut chunks in &input.lines().chunks(8) {
        // is it a lock or a key
        let mut is_lock = false;
        let mut lock_part = [0;5];
        //iterate through chunks
        for (idx,c) in chunks.into_iter().enumerate() {
            if c.is_empty() {
                continue;
            }
            if idx == 6 {
                if is_lock {
                    locks.push(lock_part);
                } else {
                    keys.push(lock_part);
                }
                break;
            }
            // check for type and skip first row
            if idx == 0{
                if c == LOCK {
                    is_lock = true;
                }
                continue;
            }
            // iterate through chunk and calculate heights
            for (j, p) in c.to_string().chars().enumerate() {
                if p == '#' {
                    lock_part[j] += 1;
                }
            }
            
        }
    }
    println!("keys: {:?}", keys);
    println!("locks: {:?}", locks);
    let mut total  = 0;
    for l in locks {
        for k in &keys {
            let mut ok = true;
            for i in 0..k.len() {
                let t = l[i] + k[i];
                if t > 5 {
                    ok = false;
                }
            }
            if ok {
                total += 1;
            }
        }
    }
    Some(total)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
