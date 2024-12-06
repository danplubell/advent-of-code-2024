advent_of_code::solution!(3);

use regex::Regex;
use std::ops::Add;

// mul\((?![^()]*mul\()[^()]*\) extract mul(*)
// r"mul\([^)]*\)"
//
// \((\d+(?:,\d+)*)\) extract numbers from between parenthesis
//         re.find_iter("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
//    let re = Regex::new(r"mul\([^)]*\)").unwrap();
    let mut numbers: Vec<u32> = vec![];
    for capture in re.find_iter(input) {
        let inner_re = Regex::new(r"\((\d+(?:,\d+)*)\)").unwrap();
        for c in inner_re.find_iter(capture.as_str()) {
            let trimmed = c
                .as_str()
                .trim_matches(['(', ')'])
                .split(",")
                .collect::<Vec<_>>()
                .into_iter()
                .fold(1, |x, y| x * y.parse::<u32>().unwrap());
            numbers.push(trimmed);
        }
    }
    let result = numbers.into_iter().reduce(u32::add).unwrap();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_id() {
        let re = Regex::new(r"mul\([^)]*\)").unwrap();
        let mut numbers: Vec<i32> = vec![];
        for capture in
            re.find_iter("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
        {
            let inner_re = Regex::new(r"\((\d+(?:,\d+)*)\)").unwrap();
            for c in inner_re.find_iter(capture.as_str()) {
                let trimmed = c
                    .as_str()
                    .trim_matches(['(', ')'])
                    .split(",")
                    .collect::<Vec<_>>()
                    .into_iter()
                    .fold(1, |x, y| x * y.parse::<i32>().unwrap());
                numbers.push(trimmed);
            }
        }
        let result = numbers.into_iter().reduce(i32::add).unwrap();
        println!("{}", result);
    }
}
