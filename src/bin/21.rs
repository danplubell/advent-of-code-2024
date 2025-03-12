use std::collections::{HashMap, HashSet, VecDeque};
use grid::Grid;

advent_of_code::solution!(21);

type NumberPad = [[Option<char>; 3];4];
pub fn part_one(input: &str) -> Option<u32> {
    let number_pad:NumberPad = [
        [Some('7'), Some('8'), Some('9')],
        [Some('4'), Some('5'), Some('6')],
        [Some('1'), Some('2'), Some('3')],
        [None, Some('0'), Some('A')],
    ];
    let mut start_char = 'A';
    for input_line in input.lines() {
        println!("Part one: {:?}", input_line);
        let mut start_loc: (usize, usize) = (0, 0);
        let mut end_loc: (usize, usize) = (0, 0);
        let avoid_number_pos = (0,3);
        input_line.chars().enumerate().for_each(|(i, c)| {
            let start_pos = find_pos(start_char, &number_pad);
            let end_pos = find_pos(c, &number_pad);
            if let (Some(start_pos), Some(end_pos))= (start_pos, end_pos) {
                println!("{:?} {:?}", start_pos, end_pos);
                get_path_patterns(start_pos, end_pos, avoid_number_pos, &number_pad);
            }
            start_char = c;
        })
    }
    
    None
}
fn get_path_patterns(start_pos:(usize,usize),end_pos:(usize,usize),avoid_pos:(usize,usize),number_pad:&NumberPad)  {
    let mut queue = VecDeque::from([start_pos]);
    let mut visited = HashSet::from([start_pos]);

    let mut parent: HashMap<(usize, usize), (usize, usize, char)> = HashMap::new();
    while let Some((r, c)) = queue.pop_front() {
        // Check all four directions
        for (idx, (dr, dc, dir)) in [(1, 0, 'v'), (0, 1,'>' ), (-1, 0, '<'), (0, -1,'^' )].iter().enumerate() {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if (r,c) == avoid_pos {
                continue;
            }
            // Found the target
            if (r, c) == end_pos {
                break;
            }
            // Skip if out of bounds
            if nr < 0 || nc < 0 || nr >= 4 as isize || nc >= 3 as isize {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            // Add to queue
            queue.push_back((nr, nc));
            visited.insert((nr, nc));
            parent.insert((nr, nc), (r, c, *dir));
        }
    }
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut current = end_pos;
    while current != start_pos {
        path.push(current);
        current = parent[&current];
    }
    
    path.reverse();

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
