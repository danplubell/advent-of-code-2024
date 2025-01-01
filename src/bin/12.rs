use std::collections::HashMap;

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
    region: RegionId,
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
    fn new(species: char, region: RegionId, position: Position) -> Self {
        Self {
            species,
            region,
            position,
            borders: Borders::default(),
        }
    }
}

const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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
pub fn part_one(input: &str) -> Option<u32> {
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
