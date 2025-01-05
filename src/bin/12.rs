use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Range};
advent_of_code::solution!(12);

// Use meaningful newtype patterns instead of type aliases for better type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RegionId(i32);
type Plants = HashMap<Position, Plant>;
#[derive(Debug, PartialEq, Clone)]
struct Plant {
    species: char,
    region: Option<RegionId>,
    position: Position,
    // Group related fields into a more meaningful structure
    borders: Borders,
    neighbors: Neighbors,
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Borders {
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
}

impl Plant {
    fn new(species: char, region: Option<RegionId>, position: Position) -> Self {
        Self {
            species,
            region,
            position,
            borders: Borders::default(),
            neighbors: Neighbors::default(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
struct Neighbors {
    top: Option<Position>,
    right: Option<Position>,
    bottom: Option<Position>,
    left: Option<Position>,
}
const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn calc_position(offset: (i32, i32), position: Position) -> Option<Position> {
    Option::from(Position {
        row: position.row.checked_add_signed(offset.0 as isize)?,
        col: position.col.checked_add_signed(offset.1 as isize)?,
    })
}
fn get_neighbors(plant: &Plant, plants: &Plants) -> Neighbors {
    let mut neighbor_positions: [Option<Position>; 4] = [None, None, None, None];
    for (i, offset) in NEIGHBOR_OFFSETS.iter().enumerate() {
        neighbor_positions[i] = calc_position(NEIGHBOR_OFFSETS[i], plant.position);
    }
    Neighbors {
        top: neighbor_positions[0],
        right: neighbor_positions[1],
        bottom: neighbor_positions[2],
        left: neighbor_positions[3],
    }
}
fn visit_neighbors(plant: &Plant, plants: &mut Plants, visit_list: &mut HashMap<Position, bool>, region_ids: &mut Range<i32>) -> HashMap<Position,Plant>{
    let mut plants_clone = plants.clone();
    let region_id:Option<RegionId> = plant.region.map_or_else(|| Some(RegionId(region_ids.next().unwrap())), |r| Some(r) );
    
    for neighbor in      [
        plant.neighbors.top,
        plant.neighbors.right,
        plant.neighbors.bottom,
        plant.neighbors.left,
    ]
        .iter()
        .flatten() {
        let was_visited = visit_list.get(&neighbor).is_some();
        if let Some(neighbor_plant) = plants.get_mut(&neighbor) {
            if neighbor_plant.species == plant.species && !was_visited {
                println!("plant: {} {:?} neighbor: {} {:?}", plant.species, plant.position, neighbor_plant.species, neighbor_plant.position);
                visit_list.insert(neighbor_plant.position,true);
                neighbor_plant.region = Some(RegionId(0));
                let mut new_plant = neighbor_plant.clone();
                new_plant.region = region_id;
                plants_clone.insert(*neighbor,new_plant.clone() );
                plants_clone = visit_neighbors(&new_plant, &mut plants_clone, visit_list,region_ids );
            }
        }
    }
    plants_clone
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut plants: HashMap<Position, Plant> = HashMap::new();
    let mut regions: HashMap<RegionId, Vec<Plant>> = HashMap::new();
    let mut region_ids: Range<i32> = 0..1000;
    input.lines().enumerate().for_each(|(row_idx, row)| {
        row.chars().enumerate().for_each(|(col_idx, species)| {
            // based on the neighbors is the plant in a region
            let position = Position {
                row: row_idx,
                col: col_idx,
            };
            // create the plant
            let mut plant = Plant::new(species, None, position);
            let neighbors = get_neighbors(&plant, &plants);
            plant.neighbors = neighbors;

            // insert the plant
            plants.insert(position, plant.clone());
        })
    });
    let mut visit_list = HashMap::new();
    let mut plants_clone = plants.clone();
    plants.iter().sorted_by_key(|&(k, _)| k).for_each(|(_,p)|{
       plants_clone =  visit_neighbors(p, &mut plants_clone, &mut visit_list, &mut region_ids);
    });
    plants_clone.iter().for_each(|(k, v)| {
        if v.species == 'Z' {
            println!("{:?}", v);
        }
    });
    None
}


pub fn part_two(_input: &str) -> Option<u32> {
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
    fn test_calc_position() {
        assert_eq!(
            calc_position((0, 0), Position { row: 0, col: 0 }),
            Some(Position { row: 0, col: 0 })
        );
        assert_eq!(
            calc_position((1, 0), Position { row: 0, col: 0 }),
            Some(Position { row: 1, col: 0 })
        );
        assert_eq!(
            calc_position((0, 1), Position { row: 0, col: 0 }),
            Some(Position { row: 0, col: 1 })
        );
        assert_eq!(
            calc_position((0, 2), Position { row: 0, col: 0 }),
            Some(Position { row: 0, col: 2 })
        );
        assert_eq!(calc_position((0, -1), Position { row: 0, col: 0 }), None);
        assert_eq!(calc_position((0, -2), Position { row: 0, col: 0 }), None);
    }
    #[test]
    fn test_get_neighbors() {
        let mut plants: Plants = Plants::new();
        let p = Plant {
            species: 'a',
            region: None,
            position: Position { row: 0, col: 0 },
            borders: Default::default(),
            neighbors: Neighbors::default(),
        };
        plants.insert(p.position, p);
        let p = Plant {
            species: 'a',
            region: None,
            position: Position { row: 0, col: 1 },
            borders: Default::default(),
            neighbors: Neighbors::default(),

        };
        plants.insert(p.position, p);
        let p = Plant {
            species: 'a',
            region: None,
            position: Position { row: 1, col: 0 },
            borders: Default::default(),
            neighbors: Neighbors::default(),
        };
        plants.insert(p.position, p);
        let p = Plant {
            species: 'a',
            region: None,
            position: Position { row: 0, col: 0 },
            borders: Default::default(),
            neighbors: Neighbors::default(),

        };

        let n = get_neighbors(&p, &plants);
        let expected = Neighbors {
            top: None,
            right: Some(Position { row: 0, col: 1 }),
            bottom: Some(Position { row: 1, col: 0 }),
            left: None,
        };
        assert_eq!(n, expected);
    }
}
