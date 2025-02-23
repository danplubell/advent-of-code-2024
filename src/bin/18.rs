use grid::Grid;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    const GRID_SIZE: usize = 71; // s + 1 as usize

    // Initialize grid and fill with zeros
    let mut grid = Grid::new(GRID_SIZE, GRID_SIZE);
    grid.fill_with(|| 0);

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

    None
}
pub fn part_two(input: &str) -> Option<String> {
    const GRID_SIZE: usize = 71; // 71; // s + 1 as usize

    // Initialize grid and fill with zeros
    let mut grid = Grid::new(GRID_SIZE, GRID_SIZE);
    grid.fill_with(|| 0);

    // Parse input and mark grid points
    //1024
    for line in input.lines().take(1024) {
        if let Some((c_str, r_str)) = line.split_once(',') {
            if let (Ok(c), Ok(r)) = (c_str.parse::<usize>(), r_str.parse::<usize>()) {
                grid[(r, c)] = 1;
            }
        }
    }

    for i in 1025..input.lines().count() {
        // Parse input and mark grid points
        //1024
        let mut current_bytes = (0, 0);
        let next_bytes = input.lines().nth(i);
        if next_bytes.is_some() {
        } else {
            break;
        }

        if let Some((c_str, r_str)) = next_bytes.unwrap().split_once(',') {
            if let (Ok(c), Ok(r)) = (c_str.parse::<usize>(), r_str.parse::<usize>()) {
                current_bytes = (c, r);
                grid[(r, c)] = 1;
            }
        }

        // BFS to find shortest path
        let mut queue = VecDeque::from([(0, 0, 0)]);
        let mut visited = HashSet::from([(0, 0)]);
        let mut d = 0;
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
                    d = distance + 1; //Some((distance + 1) as u32);
                    break;
                }

                // Add to queue
                queue.push_back((nr as i32, nc as i32, distance + 1));
            }
        }

       if d == 0 {
           
           return Some(format!("{},{}",  current_bytes.0, current_bytes.1))
       }
    }
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
