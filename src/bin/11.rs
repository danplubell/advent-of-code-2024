use std::collections::HashMap;
use rayon::prelude::*;
advent_of_code::solution!(11);

enum StoneType {
    IsZero,
    IsEvenDigits,
    Multiply,
}
fn is_even_digits(n: i64) -> bool {
    n.abs().to_string().len() % 2 == 0
}
fn split_stone(n: i64) -> (i64, i64) {
    let abs_n = n.abs();
    let n_str = abs_n.to_string();
    let s = n_str.split_at(n_str.len() / 2);
    (s.0.parse::<i64>().unwrap(), s.1.parse::<i64>().unwrap())
}
pub fn part_one(input: &str) -> Option<usize> {
    let list: Vec<_> = input.lines().nth(0)?.split(" ").collect();
    let stones: Vec<i64> = list.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    let mut stone_list: Vec<i64> = stones.clone();
    let mut working_list: Vec<i64> = vec![];
    (0..25).for_each(|i| {
        stone_list.iter().for_each(|n| {
            let stone_type: StoneType;
            if *n == 0 {
                stone_type = StoneType::IsZero;
            } else if is_even_digits(*n) {
                stone_type = StoneType::IsEvenDigits;
            } else {
                stone_type = StoneType::Multiply;
            }
            match stone_type {
                StoneType::IsZero => working_list.push(1),
                StoneType::IsEvenDigits => {
                    let result = split_stone(*n);
                    working_list.push(result.0);
                    working_list.push(result.1);
                }
                StoneType::Multiply => working_list.push(*n * 2024),
            }
        });
        stone_list = working_list.clone();
        working_list = vec![];
    });
    Some(stone_list.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let list: Vec<_> = input.lines().nth(0)?.split(" ").collect();
    let stones: Vec<i64> = list.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    let mut cache:HashMap<(i64,i64), i64> = HashMap::new();

    let m:Vec<i64> = stones.iter().map(|s| blink(*s, 75, &mut cache)).collect();
    let r: i64= m.iter().sum(); 
    Some(r as usize)
}

pub fn part_two3(input: &str) -> Option<usize> {
    let list: Vec<_> = input.lines().nth(0)?.split(" ").collect();
    let stones: Vec<i64> = list.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    let mut work_list: &mut std::vec::Vec<i64> = &mut vec![];
    let mut ref_list: &mut Vec<i64>;
    let mut even_list: std::vec::Vec<i64> = vec![];
    let mut odd_list: Vec<i64> = stones.clone();
    let mut total: usize = 0;
    for i in 0..75 {
        println!("iteration: {}", i);
        if i % 2_usize == 0 {
            even_list = vec![];
            work_list = &mut even_list;
            ref_list = &mut odd_list;
        } else {
            odd_list = vec![];
            work_list = &mut odd_list;
            ref_list = &mut even_list;
        }
        for j in 0..ref_list.len() {
            let stone = ref_list[j];
            let stone_type: StoneType;
            if stone == 0 {
                stone_type = StoneType::IsZero;
            } else if is_even_digits(stone) {
                stone_type = StoneType::IsEvenDigits;
            } else {
                stone_type = StoneType::Multiply;
            }
            match stone_type {
                StoneType::IsZero => {
                    work_list.push(1);
                }
                StoneType::IsEvenDigits => {
                    let result = split_stone(stone);
                    work_list.push(result.0);
                    work_list.push(result.1);
                }
                StoneType::Multiply => work_list.push(stone * 2024),
            }
            total = work_list.len();
        }
    }
    Some(total)
}
fn blink(stone: i64, blinks: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if blinks == 0 {
        return 1
    }
    let key = (stone,blinks);
    let cache_value = cache.get(&key);
    match cache_value {
        Some(value) => *value,
        None=> {
            let stone_type: StoneType;
            if stone == 0 {
                stone_type = StoneType::IsZero;
            } else if is_even_digits(stone) {
                stone_type = StoneType::IsEvenDigits;
            } else {
                stone_type = StoneType::Multiply;
            }
            let result = match stone_type {
                StoneType::IsZero => {
                    blink(1, blinks -1, cache)
                }
                StoneType::IsEvenDigits => {
                    let result = split_stone(stone);
                    let r1 = blink(result.0, blinks -1, cache);
                    let r2 = blink(result.1, blinks-1, cache);
                    r1 + r2
                }
                StoneType::Multiply => {
                    blink(stone * 2024, blinks-1, cache)
                }
            };
            cache.insert(key, result);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
    #[test]
    fn text_blink() {
        let mut cache:HashMap<(i64,i64), i64> = HashMap::new();
        let l = [125_i64, 17];
        let m: Vec<_> = l.iter().map(|s| blink(*s, 25, &mut cache)).collect();
        //let r = blink(1,25, &mut cache);
        let r: i64 = m.iter().sum();
        println!("{}", r);
    }
}
