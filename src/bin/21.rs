use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use grid::Grid;
use itertools::Itertools;

advent_of_code::solution!(21);

type NumberPad = [[Option<char>; 3];4];
type DirectionPad = [[Option<char>; 3]; 2];

pub fn part_one(input: &str) -> Option<u32> {
    let number_pad:NumberPad = [
        [Some('7'), Some('8'), Some('9')],
        [Some('4'), Some('5'), Some('6')],
        [Some('1'), Some('2'), Some('3')],
        [None, Some('0'), Some('A')],
    ];
    let direction_pad:DirectionPad = [
        [None, Some('^'), Some('A')],
        [Some('<'), Some('v'), Some('>')],
        
    ],
    let mut start_char = 'A';
    for input_line in input.lines() {
        println!("Part one: {:?}", input_line);
        let avoid_number_pos = (0,3);
        let mut patterns = Vec::new();
        let mut pattern = Vec::new();
        input_line.chars().for_each(|c| {
            let start_pos = find_pos(start_char, &number_pad);
            let end_pos = find_pos(c, &number_pad);
            if let (Some(start_pos), Some(end_pos))= (start_pos, end_pos) {
                patterns = get_path_patterns(start_pos, end_pos, avoid_number_pos, &number_pad);
                println!("patterns: {:?}", patterns);
                pattern.push(patterns[0].clone())
            }
            start_char = c;
        });
        // have number pad patterns in patterns
       let pattern =  pattern.iter().join("");
        
    }
    None
}
fn get_path_patterns(start_pos:(usize,usize),end_pos:(usize,usize),avoid_pos:(usize,usize),number_pad:&NumberPad)-> Vec<String>  {
    let max_col = number_pad[0].len();
    let max_row = number_pad.len();
    println!("{:?} {:?} ", max_col, max_row );
    let mut queue = VecDeque::new();
    queue.push_back((start_pos, String::new(), 0));
    // Track distance to each cell (used instead of a simple visited set)
    let mut distance = std::collections::HashMap::new();
    distance.insert(start_pos, 0);
    let mut shortest_paths = Vec::new();
    let mut shortest_distance = usize::MAX;

    // Define possible movements: right, down, left, up
    // The index corresponds to the direction: 0 = right (>), 1 = down (v), 2 = left (<), 3 = up (^)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let direction_chars = ['>', 'v', '<', '^'];
    while let Some(((r, c), mut path, level)) = queue.pop_front() {
        if !shortest_paths.is_empty() && level > shortest_distance {
            continue;
        }
        // Check all four directions
        if (r, c) == end_pos {
            match level.cmp(&shortest_distance) {
                Ordering::Less => {
                    shortest_distance = path.len();
                    shortest_paths = Vec::new();
                    path.push('A');
                    shortest_paths.push(path);
                    
                }
                Ordering::Equal => {
                    path.push('A');
                    shortest_paths.push(path);
                }
                _=>()
            }
            continue;
        }
        for (idx, ((dr, dc))) in directions.iter().enumerate() {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if (r,c) == avoid_pos {
                continue;
            }
            // Found the target
            // Skip if out of bounds
            if nr < 0 || nc < 0 || nr >= 4isize || nc >= 3isize {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            let new_level = level + 1;
            if !distance.contains_key(&(nr, nc)) || distance[&(nr, nc)]  == new_level {
                distance.insert((nr, nc), new_level);
                let mut new_path = path.clone();
                new_path.push(direction_chars[idx]);
                queue.push_back(((nr,nc), new_path, new_level));
            }
        }
    }
    shortest_paths
}
fn find_pos(value: char, number_pad:&NumberPad)-> Option<(usize, usize)> {
    for r in 0..number_pad.len() {
        for c in 0..number_pad[r].len() {
            if let Some(v) = number_pad[r][c] {
                if v == value {
                    return Some((r, c));
                }
            }
        }
    }
    None
}
fn solve_number_pad(number_pad: &[[Option<&str>; 3];4]) {

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
    #[test]
    fn test_find_pos() {
        let number_pad:[[Option<char>; 3];4] = [
            [Some('7'), Some('8'), Some('9')],
            [Some('4'), Some('5'), Some('6')],
            [Some('1'), Some('2'), Some('3')],
            [None, Some('0'), Some('A')],
        ];
        assert_eq!(find_pos('7', &number_pad), Some((0,0)));
    }
}
