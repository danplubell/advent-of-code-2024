use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

advent_of_code::solution!(21);

type NumberPad = [[Option<char>; 3];4];
type DirectionPad = [[Option<char>; 3]; 2];
type Grid = Vec<Vec<Option<char>>>;

pub fn part_one(input: &str) -> Option<u32> {
    let number_pad:Grid = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];
    let direction_pad:Grid = vec![
        vec![None, Some('^'), Some('A')],
        vec![Some('<'), Some('v'), Some('>')],
        
    ];
    let mut total:u32 = 0;
    for input_line in input.lines() {
        let pn = gen_path(&number_pad,input_line,(0,3));
//        println!("Numeric: {:?}",pn);
        let pd1 = gen_path(&direction_pad, &pn, (0, 0));
//        println!("First: {:?}",pd1);
        let pd2 = gen_path(&direction_pad, &pd1, (0, 0));
    //    println!("Second: {} {:?}",pd2.len(), pd2 );
        let split = input_line.chars().take(3).collect::<String>();
        let n:u32 = split.parse().unwrap();
        let complexity = n * pd2.len() as u32;
        println!("{:?} {} {} {:?}",input_line, pd2.len(), n, pd2);
        total += complexity;
//        let pd3 = gen_path(&direction_pad, &pd2, (0, 0));
//        println!("Third: {:?} {}",pd3,pd3.len());
                
        /*
        let mut avoid_number_pos = (0,3);
        let mut patterns = Vec::new();
        let mut pattern = Vec::new();
        input_line.chars().for_each(|c| {
            let start_pos = find_pos(start_char, &number_pad);
            let end_pos = find_pos(c, &number_pad);
            if let (Some(start_pos), Some(end_pos))= (start_pos, end_pos) {
                patterns = get_path_patterns(start_pos, end_pos, avoid_number_pos, &number_pad);
                pattern.push(patterns[0].clone())
            }
            start_char = c;
        });
        // have number pad patterns in patterns
       let curr_pattern =  pattern.iter().join("");
//        println!("pattern: {:?}", pattern);
        avoid_number_pos = (0,0);
        pattern = Vec::new();
        curr_pattern.chars().for_each(|c| {
            let start_pos = find_pos(start_char, &direction_pad);
            let end_pos = find_pos(c, &direction_pad);
            if let (Some(start_pos), Some(end_pos))= (start_pos, end_pos) {
                patterns = get_path_patterns(start_pos, end_pos, avoid_number_pos, &direction_pad);
                pattern.push(patterns[0].clone());
                println!("patterns1: {:?}", patterns);
            }
            start_char = c;
        });
        println!("second: {:?}", pattern.iter().join(""));
        
         */
    }
    Some(total)
}
fn gen_path(grid:&Grid,input_line: &str,avoid_pos: (usize,usize)) -> String {
    let mut start_char = 'A';
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();
    input_line.chars().for_each(|c| {
        let start_pos = find_pos(start_char, grid);
        let end_pos = find_pos(c, grid);
        if let (Some(start_pos), Some(end_pos))= (start_pos, end_pos) {
            patterns = get_path_patterns(start_pos, end_pos, avoid_pos, grid);
            pattern.push(patterns[0].clone())
        }
        start_char = c;
    });
    // have number pad patterns in patterns
    pattern.iter().join("")

}
fn get_path_patterns(start_pos:(usize,usize),end_pos:(usize,usize),avoid_pos:(usize,usize),number_pad:&Grid)-> Vec<String>  {
    let max_col = number_pad[0].len();
    let max_row = number_pad.len();
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
fn find_pos(value: char, number_pad:&Grid)-> Option<(usize, usize)> {
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
    fn test_find_pos() {
        let number_pad:Grid = vec![
            vec![Some('7'), Some('8'), Some('9')],
            vec![Some('4'), Some('5'), Some('6')],
            vec![Some('1'), Some('2'), Some('3')],
            vec![None, Some('0'), Some('A')],
        ];
        assert_eq!(find_pos('7', &number_pad), Some((0,0)));
    }
}
