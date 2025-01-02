use std::cmp::max;
use std::collections::HashMap;
use std::ops::{DerefMut, Range};

advent_of_code::solution!(12);

// Use meaningful newtype patterns instead of type aliases for better type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RegionId(usize);
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

/*
fn get_neighbor_region(
    species: char,
    position: Position,
    plants: &HashMap<Position, Plant>,
) -> Option<RegionId> {
    NEIGHBOR_OFFSETS
        .iter()
        .filter_map(|(row_offset, col_offset)| {
            let new_row = position.row.checked_add_signed(*row_offset as isize)?;
            let new_col = position.col.checked_add_signed(*col_offset as isize)?;

            let neighbor_pos = Position {
                row: new_row,
                col: new_col,
            };
            plants
                .get(&neighbor_pos)
                .filter(|plant| plant.species == species)
                .map(|plant| plant.region)
        })
        .next()
}


 */
/*
fn flood_neighbors(plant: &mut Plant, plants: &mut HashMap<Position, Plant>, region_ids: &mut Range<i32>) {
    // get all the neighbors
    let neighbors: Vec<_> = NEIGHBOR_OFFSETS
        .iter()
        .filter_map(|(row_offset, col_offset)| {
            let new_row = plant
                .position
                .row
                .checked_add_signed(*row_offset as isize)?;
            let new_col = plant
                .position
                .col
                .checked_add_signed(*col_offset as isize)?;

            let neighbor_pos = Position {
                row: new_row,
                col: new_col,
            };
            plants
                .get(&neighbor_pos)
                .filter(|plant_ref| plant.species == plant_ref.species)
        })
        .collect();
    // have  list of neighbors now
    // now which region to use???
    // do any of the neighbors have a regions?

    let found_neighbor =  neighbors.iter().find(|neighbor| neighbor.region.is_some());
    let region_id = match found_neighbor {
        Some(n) => n.region.unwrap(),
        None => RegionId(region_ids.next().unwrap() as usize)
    };

    // update neighbors with regionId
    for neighbor in neighbors {
        neighbor.region = Some(region_id);
    }
}

 */
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
fn flood_neighbors(
    plant: &mut Plant,
    plants: &mut HashMap<Position, Plant>,
    region_ids: &mut Range<i32>,
) -> HashMap<Position, Plant> {
    plants.clone()
}
fn is_species_neighbor(position: Option<Position>, plant: &Plant, plants: &Plants) -> Option<Position> {
    position.and_then(|pos| {
        plants.get(&pos).and_then(|neighbor| {
            (neighbor.species == plant.species).then_some(neighbor.position)
        })
    })
}
fn get_species_neighbors(plant:&Plant, plants: &Plants)->Neighbors {
    Neighbors {
        top: is_species_neighbor(plant.neighbors.top,plant, plants),
        right: is_species_neighbor(plant.neighbors.right,plant, plants),
        bottom: is_species_neighbor(plant.neighbors.bottom, plant, plants),
        left: is_species_neighbor(plant.neighbors.left, plant, plants),
    }
}
fn get_neighbor_region_id(neighbors: &Neighbors, plants: &Plants) -> Option<RegionId>{
    let a: [Option<Position>;4] = [neighbors.top, neighbors.right, neighbors.bottom, neighbors.left];
    a.iter().min().map(|n| {
        let p = match n {
           Some(v) => v,
            _=> return None
            
        };
        let neighbor_plant = plants.get(&p);
        return neighbor_plant.map(|p| p.region)?;
    });
    None
}
fn assign_regions(plants: &mut Plants){
    plants.iter().for_each(|p| {
        let species_neighbors = get_species_neighbors(p.1, plants);
        let potential_region_id = get_neighbor_region_id(&species_neighbors, plants);
    })
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
            let mut plant = Plant::new(species, None, position, );
            let neighbors = get_neighbors(&plant, &plants);
            plant.neighbors = neighbors;

            // insert the plant
            plants.insert(position, plant.clone());
        })
    });
    // now we have all the plants in a hashmap by position
    // let's try to put them in regions
    assign_regions(&mut plants);
    println!("plants: {:?}", plants);
    None
}

/*
pub fn part_one1(input: &str) -> Option<u32> {
    let mut plants: HashMap<Position, Plant> = HashMap::new();
    let mut regions: HashMap<RegionId, Vec<Plant>> = HashMap::new();
    let mut region_ids = 0..1000;
    input.lines().enumerate().for_each(|(row_idx, row)| {
        row.chars().enumerate().for_each(|(col_idx, species)| {
            // based on the neighbors is the plant in a region
            let position = Position {
                row: row_idx,
                col: col_idx,
            };
            let in_region = get_neighbor_region(species, position, &plants);

            // us the key that was returned or get a new key
            let current_region_key = match in_region {
                Some(region) => region,
                None => RegionId(region_ids.next().unwrap_or(9999999)),
            };
            // create the plant
            let mut plant = Plant::new(
                species,
                current_region_key,
                Position {
                    row: row_idx,
                    col: col_idx,
                },
            );

//            println!("plant: {:?}", plant);
            // insert the plant
            plants.insert(
                Position {
                    row: row_idx,
                    col: col_idx,
                },
                plant,
            );


            let _region  =  match regions.get(&current_region_key) {
                Some(r) => r,
                None => {
                    regions.insert(current_region_key,vec![]);
                    regions.get(&current_region_key).unwrap()
                }
            };
        })
    });
    //    println!("regions: {:?}", regions);
    //    println!("plants: {:?}", plants);
    None
}

 */

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
