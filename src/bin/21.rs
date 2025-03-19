use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(21);

type NumberPad = [[Option<char>; 3]; 4];
type DirectionPad = [[Option<char>; 3]; 2];
type Grid = Vec<Vec<Option<char>>>;

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
