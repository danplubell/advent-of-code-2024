use std::collections::VecDeque;
use grid::Grid;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let s: i32 = 70;
    let mut grid: Grid<i32> = Grid::new((s + 1) as usize, (s + 1) as usize);
    grid.fill_with(|| 0);
    input.lines().take(1024).for_each(|line| {
        let s = line.split(',').collect::<Vec<&str>>();
        let c = s.first().unwrap().parse::<i32>().unwrap();
        let r = s.last().unwrap().parse::<i32>().unwrap();
        grid[(r as usize, c as usize)] = 1;
    });

    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::from([(0, 0, 0)]);
    let mut seen: Vec<(i32, i32)> = Vec::from([(0, 0)]);
    while let Some((r, c, d)) = q.pop_front() {
        for (nr, nc) in [(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)] {
            if nr < 0 || nc < 0 || nr > s || nc > s {
                continue;
            }
            if grid[(nr as usize, nc as usize)] == 1 {
                continue;
            }
            if seen.contains(&(nr, nc)) {
                continue;
            }
            
            if nr == s && nc == s {
                return Some((d + 1) as u32);
            }
            seen.push((nr, nc));
            let mut b:VecDeque<_> = [(nr, nc, d + 1)].into();
            q.append(&mut b);
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
