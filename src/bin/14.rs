advent_of_code::solution!(14);
#[derive(Debug, Clone, Copy, PartialEq)]
struct Robot {
    position: Vector,
    velocity: Vector,
}
fn get_robot(line: &str) -> Robot {
    // p=81,85 v=-35,88
    let parts_raw: Vec<_> = line.split(" ").collect();
    let parts: Vec<_> = parts_raw
        .iter()
        .map(|p| {
            let p_raw: Vec<_> = p.split("=").collect();
            p_raw[1]
        })
        .collect();
    let p: Vec<_> = parts[0]
        .split(",")
        .map(|p| p.parse::<i32>().ok().unwrap())
        .collect();
    let v: Vec<_> = parts[1]
        .split(",")
        .map(|p| p.parse::<i32>().ok().unwrap())
        .collect();

    Robot {
        position: Vector {
            x: p[0] as f64,
            y: p[1] as f64,
            z: 0.0,
        },
        velocity: Vector {
            x: v[0] as f64,
            y: v[1] as f64,
            z: 0.0,
        },
    }
}
fn get_quadrant(x: u32, y: u32, grid_cell: &GridCell) ->Option<u32> {
    let mid_x = x/2;
    let mid_y = y/2;
    if grid_cell.x == mid_x || grid_cell.y == mid_y {
        return None;
    }
    if grid_cell.x < mid_x && grid_cell.y < mid_y {
        return Some(0);
    }
    if grid_cell.x > mid_x && grid_cell.y < mid_y {
        return Some(1);
    }
    if grid_cell.x < mid_x && grid_cell.y > mid_y {
        return Some(2);
    }
    if grid_cell.x > mid_x && grid_cell.y > mid_y {
        return Some(3);
    }
    None
    
}
pub fn part_one(input: &str) -> Option<u32> {
    let width = 101;
    let height = 103;
    let grid = Grid::new(width, height).unwrap();
    // get all robots
    let robots: Vec<_> = input.lines().map(|l| get_robot(l)).collect();
    // for all robots calculate normalized position
    let normalized_positions: Vec<_> = robots
        .iter()
        .map(|r| {
            let initial_pos = Vector::new_2d(r.position.x, r.position.y);
            let velocity = Vector::new_2d(r.velocity.x, r.velocity.y);
            let time = 100.0;

            match calculate_position(&initial_pos, &velocity, time, &grid) {
                Ok(result) => {
                    Some(result.grid_cell)
                    /*                println!("Initial position: {}", initial_pos);
                                   println!("Velocity: {}", velocity);
                                   println!("Time: {} seconds", time);
                                   println!("Raw final position: {}", result.raw_position);
                                   println!("Position in grid: {}", result.grid_position);
                                   println!("Grid cell: ({}, {})", result.grid_cell.x, result.grid_cell.y);
                                   result.grid_cell
                    */
                }
                Err(e) => {
                    println!("Error: {}", e);
                    None
                }
            }
        })
        .collect();
//    println!("{:?}", normalized_positions);

    let mut grid_cell_map: HashMap<GridCell, i32> = HashMap::new();
    for p in normalized_positions.iter() {
        let position = p.clone().unwrap();
        let map_grid_cell = grid_cell_map.get(&position);
        let count = match map_grid_cell {
            Some(t) => {
                t + 1
            },
            None=> 1
        };
        grid_cell_map.insert(position, count);
    }
    //println!("gridcells, {:?}", grid_cell_map);
    let mut quadrants = [0,0,0,0];
    for e in grid_cell_map.iter() {
        let cell_q = get_quadrant(width, height, e.0);
        if let Some(q) = cell_q {
            quadrants[q as usize] += e.1;
        }
    }
    //println!("quads: {:?}", quadrants);
    let mut total = 1;
    for q in quadrants {
        total *= q;
    }
//    let total = quadrants.iter().product();
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = 101;
    let height = 103;
    let grid = Grid::new(width, height).unwrap();
    // get all robots
    let robots: Vec<_> = input.lines().map(get_robot).collect();
    // for all robots calculate normalized position
    for i in 0..17174{
        println!("time: {i}");
        println!();
        let normalized_positions: Vec<_> = robots
            .iter()
            .map(|r| {
                let initial_pos = Vector::new_2d(r.position.x, r.position.y);
                let velocity = Vector::new_2d(r.velocity.x, r.velocity.y);
                let time = i as f64;

                match calculate_position(&initial_pos, &velocity, time, &grid) {
                    Ok(result) => {
                        Some(result.grid_cell)
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        None
                    }
                }
            })
            .collect();
        let mut grid_cell_map: HashMap<GridCell, i32> = HashMap::new();
        for p in normalized_positions.iter() {
            let position = p.clone().unwrap();
            let map_grid_cell = grid_cell_map.get(&position);
            let count = match map_grid_cell {
                Some(t) => {
                    t + 1
                },
                None=> 1
            };
            grid_cell_map.insert(position, count);
        }

        for i in 0..height {
            for j in 0..width {
                let gc = GridCell {
                    x: j,
                    y: i,
                };
                let c = grid_cell_map.get(&gc);
                let symbol = match c {
                    Some(x) => 'X',
                    _=> '.'
                };
                print!("{symbol}")
            }
            println!();
        }

    }
    None
}

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn new_2d(x: f64, y: f64) -> Self {
        Vector { x, y, z: 0.0 }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn multiply(&self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
pub struct Grid {
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err("Grid dimensions must be positive numbers".to_string());
        }
        Ok(Grid { width, height })
    }

    pub fn normalize_position(&self, position: &Vector) -> Vector {
        let x =
            ((position.x.rem_euclid(self.width as f64)) + self.width as f64) % self.width as f64;
        let y =
            ((position.y.rem_euclid(self.height as f64)) + self.height as f64) % self.height as f64;
        Vector::new(x, y, position.z)
    }

    pub fn get_grid_cell(&self, position: &Vector) -> GridCell {
        let normalized = self.normalize_position(position);
        GridCell {
            x: normalized.x.floor() as u32,
            y: normalized.y.floor() as u32,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
#[derive(Clone)]
pub struct GridCell {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct PositionResult {
    pub raw_position: Vector,
    pub grid_position: Vector,
    pub grid_cell: GridCell,
}

pub fn calculate_position(
    initial_position: &Vector,
    velocity: &Vector,
    time: f64,
    grid: &Grid,
) -> Result<PositionResult, String> {
    if time < 0.0 {
        return Err("Time must be non-negative".to_string());
    }

    let displacement = velocity.multiply(time);
    let raw_position = initial_position.add(&displacement);
    let grid_position = grid.normalize_position(&raw_position);
    let grid_cell = grid.get_grid_cell(&grid_position);

    Ok(PositionResult {
        raw_position,
        grid_position,
        grid_cell,
    })
}

/*
fn main() {
    // Example usage
    let grid = Grid::new(10, 10).unwrap();
    let initial_pos = Vector::new_2d(5.0, 5.0);
    let velocity = Vector::new_2d(2.0, 1.0);
    let time = 3.0;

    match calculate_position(&initial_pos, &velocity, time, &grid) {
        Ok(result) => {
            println!("Initial position: {}", initial_pos);
            println!("Velocity: {}", velocity);
            println!("Time: {} seconds", time);
            println!("Raw final position: {}", result.raw_position);
            println!("Position in grid: {}", result.grid_position);
            println!("Grid cell: ({}, {})", result.grid_cell.x, result.grid_cell.y);
        }
        Err(e) => println!("Error: {}", e),
    }
}

 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_get_robot() {
        let line = "p=2,4 v=3,-3";
        println!("{:?}", get_robot(line))
    }
    #[test]
    fn test_basic_movement() {
        let grid = Grid::new(10, 10).unwrap();
        let initial_pos = Vector::new_2d(5.0, 5.0);
        let velocity = Vector::new_2d(2.0, 1.0);
        let time = 3.0;

        let result = calculate_position(&initial_pos, &velocity, time, &grid).unwrap();

        // After 3 seconds:
        // x = 5 + (2 * 3) = 11 -> wraps to 1
        // y = 5 + (1 * 3) = 8
        assert_eq!(result.grid_cell.x, 1);
        assert_eq!(result.grid_cell.y, 8);
    }

    #[test]
    fn test_wrapping() {
        let grid = Grid::new(10, 10).unwrap();
        let initial_pos = Vector::new_2d(9.0, 9.0);
        let velocity = Vector::new_2d(2.0, 2.0);
        let time = 2.0;

        let result = calculate_position(&initial_pos, &velocity, time, &grid).unwrap();

        // After 2 seconds:
        // x = 9 + (2 * 2) = 13 -> wraps to 3
        // y = 9 + (2 * 2) = 13 -> wraps to 3
        assert_eq!(result.grid_cell.x, 3);
        assert_eq!(result.grid_cell.y, 3);
    }

    #[test]
    fn test_negative_time() {
        let grid = Grid::new(10, 10).unwrap();
        let initial_pos = Vector::new_2d(5.0, 5.0);
        let velocity = Vector::new_2d(2.0, 1.0);
        let time = -1.0;

        assert!(calculate_position(&initial_pos, &velocity, time, &grid).is_err());
    }

    #[test]
    fn test_invalid_grid() {
        assert!(Grid::new(0, 10).is_err());
        assert!(Grid::new(10, 0).is_err());
    }
}
