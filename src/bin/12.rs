use std::collections::HashMap;
use serde_json::value::Index;

advent_of_code::solution!(12);
type SpeciesType = char;
type RegionId = usize;
type Row = usize;
type Col = usize;
#[derive(Debug, PartialEq, Clone)]
struct Plant {
    species: SpeciesType,
    region: usize,
    row: usize,
    col: usize,
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
}
impl Plant {
    fn new(species: SpeciesType, region: usize, row: usize, col: usize)-> Self {
        Self {
            species,
            region,
            row, 
            col,
            top: false,
            right: false,
            bottom: false,
            left: false,
        }
    }
}
fn get_neighbor_region(species:char, row:usize, col:usize, plants:&HashMap<(Row,Col), Plant, >) -> Option<RegionId> {
    let neighbors:[(i32,i32);4] = [(-1,0), (0,1), (1,0), (0,-1)];
    let plant_list: Vec<_> = neighbors.iter().map(|n|{
        let r:i32 = (row as i32) + (n.0);
        let c:i32 = (col as i32) + (n.1);
        let key_opt = match (r >=0,c>=0)  {
            (true, true) => Some((r,c)),
            _=>None
        };
        let result:Option<RegionId> = match key_opt {
            Some(key_parts) => {
                let key = (key_parts.0 as usize,key_parts.1 as usize );
                let plant = plants.get(&key);
                match plant {
                    Some(p) => {
                      if p.species == species {
                          Some(p.region)
                      }else {
                          None
                      }
                    },
                    None => None
                }
            }
            _=>None
        };
        result
    }).filter(|x|x.is_some()).collect();
    match plant_list.first() {
        Some(v) => {
            *v
        },
        None=> None
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut plants: HashMap<(Row, Col), Plant> = HashMap::new();
    let mut regions: HashMap<RegionId, Vec<Plant>> = HashMap::new();
    let mut region_ids = 0..1000;
    input.lines().enumerate().for_each(|(row_idx, row)| {
        row.chars().enumerate().for_each(|(col_idx, species)| {
            // based on the neighbors is the plant in a region
            let in_region = get_neighbor_region(species, row_idx, col_idx, &plants);
            
            // us the key that was returned or get a new key
            let current_region_key = match in_region {
                Some(region) => region,
                None=>  region_ids.next().unwrap_or(9999999)
            };
            // create the plant
            let plant = Plant::new(species, current_region_key, row_idx, col_idx);
            
            println!("plant: {:?}", plant);
            // insert the plant
            plants.insert((row_idx, col_idx), plant);
            /*
            let _region  =  match regions.get(&key) {
                Some(r) => r,
                None => {
                    regions.insert(key,vec![]);
                    regions.get(&key).unwrap()
                }
            };
            
            
             */
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
