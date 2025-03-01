use grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut start_loc: (usize, usize) = (0, 0);
    let mut end_loc: (usize, usize) = (0, 0);
    let max_rows: usize = input.lines().count();
    let max_cols: usize = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(max_rows, max_cols);

    // find the start and end
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            grid[(row, col)] = ch;
            if ch == 'S' {
                start_loc = (row, col);
            } else if ch == 'E' {
                end_loc = (row, col);
            }
        })
    });
    println!("start_loc, end_loc {:?} {:?}", start_loc, end_loc);
    // BFS to find shortest path
    let mut queue = VecDeque::from([(start_loc.0, start_loc.1)]);
    let mut visited = HashSet::from([(start_loc.0, start_loc.1)]);

    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    while let Some((r, c)) = queue.pop_front() {
        // Check all four directions
        for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            // Found the target
            if (r,c) == end_loc {
                break;
            }
            // Skip if out of bounds
            if nr < 0 || nc < 0 || nr >= max_rows as isize || nc >= max_cols as isize {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            // Skip if it's a blocked cell or already visited
            if grid[(nr, nc)] == '#' || visited.contains(&(nr, nc)) {
                continue;
            }
            /*
            if (nr, nc) == end_loc {
                break;
            }
            
             */
            // Add to queue
            queue.push_back((nr, nc));
            visited.insert((nr, nc));
            parent.insert((nr, nc), (r, c));
        }
    }
    
    let mut path:Vec<(usize,usize)> = Vec::new();
    let mut current = end_loc;
    while current != start_loc {
        path.push(current);
        current = parent[&current];
    }
    path.push((start_loc.0, start_loc.1));
    path.reverse();
    
     
    println!("path {} {:?}",path.len(), path);
    
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
