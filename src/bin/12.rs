use std::collections::HashMap;
use std::ops::{DerefMut, Range};

advent_of_code::solution!(12);

// Use meaningful newtype patterns instead of type aliases for better type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RegionId(usize);

#[derive(Debug, PartialEq, Clone)]
struct Plant {
    species: char,
    region: Option<RegionId>,
    position: Position,
    // Group related fields into a more meaningful structure
    borders: Borders,
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
        }
    }
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
fn get_neighbors(plant: &Plant, plants: &HashMap<Position, Plant>)-> Vec<Position> {
    let neighbor_positions: Vec<Position> = NEIGHBOR_OFFSETS
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

            Some(Position {
                row: new_row,
                col: new_col,
            })
        })
        .filter(|pos| plants
            .get(pos)
            .map_or(false, |p| p.species == plant.species))
        .collect();
    neighbor_positions
}
fn flood_neighbors(plant: &mut Plant, plants: &mut HashMap<Position, Plant>, region_ids: &mut Range<i32>)  {
    let neighbor_positions = get_neighbors(plant, plants);
    let region_id = neighbor_positions.iter()
        .find_map(|pos| plants.get(pos).and_then(|p| p.region))
        .unwrap_or_else(|| RegionId(region_ids.next().unwrap() as usize));
    if plant.region.is_none() {
        plant.region = Option::from(region_id);
    }
    
    
    // Recursively flood to neighbors
    for pos in neighbor_positions {
        if let Some(neighbor) = plants.get_mut(&pos) {
            if neighbor.region.is_none() {
                flood_neighbors(neighbor, plants, region_ids);
            }
        }
    }
    plants.insert(plant.position, plant.clone());

}

pub fn part_one(input: &str) -> Option<u32> {
    let mut plants: HashMap<Position, Plant> = HashMap::new();
    let mut regions: HashMap<RegionId, Vec<Plant>> = HashMap::new();
    let mut region_ids:Range<i32> = 0..1000;
    input.lines().enumerate().for_each(|(row_idx, row)| {
        row.chars().enumerate().for_each(|(col_idx, species)| {
            // based on the neighbors is the plant in a region
            let position = Position {
                row: row_idx,
                col: col_idx,
            };
            // create the plant
            let mut plant = Plant::new(
                species,
                None,
                position,
            );

            //            println!("plant: {:?}", plant);
            // insert the plant
            plants.insert(
                position,
                plant.clone(),
            );
           let mut p = plant.clone();
            flood_neighbors(&mut p, &mut plants, &mut region_ids);
        })
    });
    //    println!("regions: {:?}", regions);
    println!("plants: {:?}", plants);
    plants.iter().for_each(|p| println!("{:?}", p));
//    plants.iter().for_each(|i| {});
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
}
