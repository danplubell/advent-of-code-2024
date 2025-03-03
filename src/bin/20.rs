use grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::absolute;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut start_loc: (usize, usize) = (0, 0);
    let mut end_loc: (usize, usize) = (0, 0);
    let max_rows: usize = input.lines().count();
    let max_cols: usize = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(max_rows, max_cols);
    let mut total =0;
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
            if (r, c) == end_loc {
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

    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut current = end_loc;
    while current != start_loc {
        path.push(current);
        current = parent[&current];
    }
    path.push((start_loc.0, start_loc.1));
    path.reverse();
    let max_distance = path.len();
    let mut steps: HashMap<(usize, usize), usize> = HashMap::new();
    // add to map for later lookup
    path.iter().enumerate().for_each(|(idx, (nr, nc))| {
        steps.insert((*nr, *nc), idx);
    });
    for (r, c) in path.iter() {
        for (nr, nc) in [
            (*r + 2, *c),
            (*r + 1, *c + 1),
            (*r, *c + 2),
            (*r - 1, *c + 1),
        ] {

            let ch = match grid.get(nr, nc){
                Some(c) => *c,
                None => continue,
            };
            if ch == '#' {
                continue;
            }
            let dist_next = match steps.get(&(nr, nc)) {
                Some(d) => d,
                None => continue,
            };
            let dist_curr = match steps.get(&(*r, *c)) {
                Some(d) => d,
                None => continue,
            };
            if (*dist_curr as isize - *dist_next as isize).abs() >= 102 {
                total += 1;
            }

        }
    }

    Some(total)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut start_loc: (usize, usize) = (0, 0);
    let mut end_loc: (usize, usize) = (0, 0);
    let max_rows: usize = input.lines().count();
    let max_cols: usize = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(max_rows, max_cols);
    let mut total =0;
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
            if (r, c) == end_loc {
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

    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut current = end_loc;
    while current != start_loc {
        path.push(current);
        current = parent[&current];
    }
    path.push((start_loc.0, start_loc.1));
    path.reverse();
    let max_distance = path.len();
    let mut steps: HashMap<(usize, usize), usize> = HashMap::new();
    // add to map for later lookup
    path.iter().enumerate().for_each(|(idx, (nr, nc))| {
        steps.insert((*nr, *nc), idx);
    });
    for (ru, cu) in path.iter() {
        let r = *ru as isize;
        let c = *cu as isize;
        for radius in 2..21 {
            for dr in 0..(radius + 1) {
                let dc = radius - dr;
                for (nr, nc) in [
                    (r + 2, c),
                    (r + 1, c + 1),
                    (r, c + 2),
                    (r - 1, c + 1),
                    (r - 2, c),
                    (r - 1, c - 1),
                    (r, c - 2),
                    (r + 1, c - 1)
                ] {
                    let ch = match grid.get(nr, nc) {
                        Some(c) => *c,
                        None => continue,
                    };
                    if ch == '#' {
                        continue;
                    }
                    let dist_next = match steps.get(&(nr as usize, nc as usize)) {
                        Some(d) => d,
                        None => continue,
                    };
                    let dist_curr = match steps.get(&(r as usize, c as usize)) {
                        Some(d) => d,
                        None => continue,
                    };
                    if (*dist_curr as isize - *dist_next as isize) >= 102 + radius {
                        total += 1;
                    }

                }
            }
        }
    }

    Some(total)

}
pub fn part_two1(input: &str) -> Option<u32> {
    let mut start_loc: (usize, usize) = (0, 0);
    let mut end_loc: (usize, usize) = (0, 0);
    let rows: usize = input.lines().count();
    let cols: usize = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::new(rows, cols);
    let mut total =0;
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
    let mut dists:Grid<i32> = Grid::new(rows, cols);
    dists.fill(-1);
    dists[(start_loc.0, start_loc.1)] = 0;
    let mut r = start_loc.0 as isize;
    let mut c = start_loc.1 as isize;
    
    while let Some(ch) = grid.get(r, c){
        if *ch == 'E' {
            break;
        }
        for (nr, nc) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize { continue }
            // is it a wall #
            match grid.get(nr, nc) {
                Some(&'#') | None => continue,
                _=> ()
            };
            // is there already a distance
            let d_n = match dists.get(nr,nc) {
                Some(d) => {
                    if *d != -1 {
                        continue;
                    }
                    d
                },
                _ => continue
            };
            let d = match dists.get(r,c) {
                Some(d) => d,
                _=> continue
            };
            dists[(nr as usize, nc as usize)] = d + 1;;
            r = nr;
            c = nc;
        }
    } 
 //   println!("finished with distances: {:?}",dists);
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[(r, c)] == '#' {
                continue;
            }
            for radius in 2..21 {
                for dr in 0..(radius + 1) {
                    let dc = radius - dr;
                    for (nr ,nc) in [(r as isize + dr, c as isize + dc), (r as isize + dr, c as isize - dc), (r as isize - dr, c as isize + dc), (r as isize - dr, c as isize - dc)] {
                        if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize { continue}
                        if grid[(nr as usize, nc as usize)] == '#' {
                            continue;
                        }
                        println!("distance: {}", dists[(r,c)] - dists[(nr as usize,nc as usize)]);
                        if dists[(r,c)] - dists[(nr as usize,nc as usize)] >= (100 + radius) as i32 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    Some(count)
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
