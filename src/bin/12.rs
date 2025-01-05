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
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Ord, PartialOrd)]
struct Region {
    perimeter: i32,
    area: i32,
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
fn is_species_neighbor(
    position: Option<Position>,
    plant: &Plant,
    plants: &Plants,
) -> Option<Position> {
    match position {
        Some(p)=> {
            let found_plant = plants.get(&p);
            match found_plant {
                Some(v) => {
                    if v.species == plant.species {
                        return position
                    }
                    None
                },
                _=>None
            }
        }
        _=> None
    }
    /*
    position.and_then(|pos| {
        plants
            .get(&pos)
            .and_then(|neighbor| (neighbor.species == plant.species).then_some(neighbor.position))
    })
    
     */
}
fn get_species_neighbors(plant: &Plant, plants: &Plants) -> Neighbors {
    Neighbors {
        top: is_species_neighbor(plant.neighbors.top, plant, plants),
        right: is_species_neighbor(plant.neighbors.right, plant, plants),
        bottom: is_species_neighbor(plant.neighbors.bottom, plant, plants),
        left: is_species_neighbor(plant.neighbors.left, plant, plants),
    }
}
fn is_single_plant(neighbors: &Neighbors)->bool {
    let n: Vec<_> = [
        neighbors.top,
        neighbors.right,
        neighbors.bottom,
        neighbors.left,
    ].into_iter().flatten().collect();
    n.is_empty()
}
fn visit_neighbors(plant: &Plant, plants: &mut Plants, visit_list: &mut HashMap<Position, bool>, region_ids: &mut Range<i32>) -> HashMap<Position,Plant>{
    let mut plants_clone = plants.clone();
    let region_id:Option<RegionId> = plant.region.map_or_else(|| Some(RegionId(region_ids.next().unwrap())), |r| Some(r) );
    let species_neighbors = get_species_neighbors(plant, plants);
    if is_single_plant(&species_neighbors) {
        let mut new_plant = plant.clone();
        new_plant.region = region_id;
        plants_clone.insert(new_plant.position, new_plant);
        visit_list.insert(plant.position, true);
    }
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
fn get_perimeter(plant: &Plant, plants: &Plants) -> i32 {
    let neighbors = get_species_neighbors(plant,plants);
    let n = [
        neighbors.top,
        neighbors.right,
        neighbors.bottom,
        neighbors.left
    ];
    (4 - n.iter().flatten().collect::<Vec<_>>().len()) as i32
}
pub fn part_one(input: &str) -> Option<i32> {
    let mut plants: HashMap<Position, Plant> = HashMap::new();
    let mut region_ids: Range<i32> = 0..i32::MAX;
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
        let was_visited = visit_list.get(&p.position).is_some();
        if !was_visited {
            plants_clone =  visit_neighbors(p, &mut plants_clone, &mut visit_list, &mut region_ids);
        }
    });
    let mut regions:HashMap<RegionId, Region> = HashMap::new();
    plants_clone.iter().for_each(|(k, v)| {
        if v.region.is_some() {
            let plant_region = v.region.unwrap();
            let mut region = regions.get_mut(&plant_region).map_or_else(|| Region {perimeter: 0, area: 0}, |r| *r);
            region.area += 1;
            region.perimeter += get_perimeter(v, &plants_clone);
            regions.insert(plant_region, region);
        }
    });
    let t = regions.iter().fold(0, |acc, (_,r)|{
        
        let price = r.area * r.perimeter;
        acc + price
    });
    Some(t)
}


pub fn part_two(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
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
