use std::collections::{HashSet, VecDeque};
use grid::Grid;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut start_loc:(usize, usize) = (0, 0);
    let mut end_loc:(usize, usize) = (0, 0);
    let max_rows:usize = input.lines().count();
    let max_cols:usize = input.lines().next().unwrap().len();
    let mut grid:Grid<char> = Grid::new(max_rows, max_cols);

    // find the start and end
    input.lines().enumerate().for_each(|(row,line)| {
        line.chars().enumerate().for_each(|(col,ch)| {
            grid[(row,col)] = ch;
            if ch == 'S' {
                start_loc = (row,col);
            } else if ch == 'E' {
                end_loc = (row,col);
            }
        })
    });
    println!("start_loc, end_loc {:?} {:?}", start_loc, end_loc);
    /*
    // Initialize grid and fill with zeros

    // Parse input and mark grid points
    for line in input.lines().take(1024) {
        if let Some((c_str, r_str)) = line.split_once(',') {
            if let (Ok(c), Ok(r)) = (c_str.parse::<usize>(), r_str.parse::<usize>()) {
                grid[(r, c)] = 1;
            }
        }
    }

    // BFS to find shortest path
    let mut queue = VecDeque::from([(0, 0, 0)]);
    let mut visited = HashSet::from([(0, 0)]);

    while let Some((r, c, distance)) = queue.pop_front() {
        // Check all four directions
        for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nr, nc) = (r + dr, c + dc);

            // Skip if out of bounds
            if nr < 0 || nc < 0 || nr >= GRID_SIZE as i32 || nc >= GRID_SIZE as i32 {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);

            // Skip if it's a blocked cell or already visited
            if grid[(nr, nc)] == 1 || !visited.insert((nr as i32, nc as i32)) {
                continue;
            }

            // Found the target
            if nr == GRID_SIZE - 1 && nc == GRID_SIZE - 1 {
                return Some((distance + 1) as u32);
            }

            // Add to queue
            queue.push_back((nr as i32, nc as i32, distance + 1));
        }
    }



     */
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
