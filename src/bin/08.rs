use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut ant: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    // collection all antennas
    let max_row = input.lines().count();
    let max_col = input.lines().nth(0).unwrap().chars().count();
    input.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, c)| {
            if c.is_alphanumeric() {
                let f_c = ant.get(&c);
                match f_c {
                    Some(v) => {
                        let mut new_v = v.clone();
                        new_v.push((row, col));
                        ant.insert(c, new_v);
                    }
                    None => {
                        let new_vec = vec![(row, col)];
                        ant.insert(c, new_vec);
                    }
                };
            }
        })
    });
    // now look for antinodes
    Some(find_antinodes(ant,max_row as i64, max_col as i64, ))
}
fn calculate_next_point((e1, n1): (i64, i64), (e2, n2): (i64, i64)) -> (i64, i64) {
    let delta_e = e2 - e1;
    let delta_n = n2 - n1;
    let e3 = e2 + delta_e;
    let n3 = n2 + delta_n;
    (e3, n3)
}
fn calculate_reverse_next_point((e1, n1): (i64, i64), (e2, n2): (i64, i64)) -> (i64, i64) {
    let delta_e = e1 - e2;
    let delta_n = n1 - n2;
    let e3 = e1 + delta_e;
    let n3 = n1 + delta_n;
    (e3, n3)
}

fn calculate_next_points((e1, n1): (i64, i64), (e2, n2): (i64, i64)) -> (i64, i64) {
    let delta_e = e2 - e1;
    let delta_n = n2 - n1;
    let e3 = e2 + delta_e;
    let n3 = n2 + delta_n;
    (e3, n3)
}
fn calculate_reverse_next_points((e1, n1): (i64, i64), (e2, n2): (i64, i64)) -> (i64, i64) {
    let delta_e = e1 - e2;
    let delta_n = n1 - n2;
    let e3 = e1 + delta_e;
    let n3 = n1 + delta_n;
    (e3, n3)
}


fn find_antinodes(ant: HashMap<char, Vec<(usize, usize)>>, max_row:i64, max_col:i64) ->usize {
//    println!("{:?}", ant); 
    let mut antinode_set: HashSet<(i64, i64)> = HashSet::new();
    ant.iter().for_each(|(k, v)| {
       
        // go through each pair in vector
        v.iter().for_each(|(r, c)| {
            // match up with all other pairs in vector
            for i in 0..v.len() {
                if v[i] == (*r, *c) {
                    continue;
                }
                let a1 =
                    calculate_next_point((*r as i64, *c as i64), (v[i].0 as i64, v[i].1 as i64));
                let a2 = calculate_reverse_next_point(
                    (*r as i64, *c as i64),
                    (v[i].0 as i64, v[i].1 as i64),
                );
                if is_valid(a1, max_row, max_col) {
                    antinode_set.insert(a1);
                }
                if is_valid(a2, max_row, max_col) {
                    antinode_set.insert(a2);
                }
            }
        })
    });
//    println!("antinodes: {:?}", antinode_set);
   antinode_set.len()
}

fn find_antinodes2(ant: HashMap<char, Vec<(usize, usize)>>, max_row:i64, max_col:i64) ->i64 {
    let mut total = 0_i64;
    ant.iter().for_each(|(k, v)| {

        // go through each pair in vector
        v.iter().for_each(|(r, c)| {
            // match up with all other pairs in vector
            for i in 0..v.len() {
                if v[i] == (*r, *c) {
                    continue;
                }
                let r = all_anti_nodes((*r as i64, *c as i64), (v[i].0 as i64, v[i].1 as i64), max_row, max_col);
                total = r.iter().count() as i64;
            }
        })
    });
    total
}

fn is_valid(p0: (i64, i64), p1: i64, p2: i64) -> bool {
    matches!((p0.0 >= 0, p0.1 >=0, p0.0 < p1, p0.1 < p2 ), (true, true, true, true))
}

fn all_anti_nodes(e:(i64, i64), n:(i64, i64), max_row:i64, max_col:i64) -> Vec<(i64, i64)> {
    let mut nodes:HashSet<(i64,i64)> = HashSet::new();
    let mut a = e;
    let mut b = n;
    loop {
        let n =  calculate_next_points(a, b);
        if !is_valid(n, max_row, max_col) {
            break;
        }
        nodes.insert(n);
        a = b;
        b = n;

    }
    loop {
        let n =  calculate_reverse_next_points(a, b);
        if !is_valid(n, max_row, max_col) {
            break;
        }
        nodes.insert(n);
        b = a;
        a = n;
    }
    println!("{:?}", nodes);
    nodes.into_iter().collect()
}



pub fn part_two(input: &str) -> Option<i64> {
    let mut ant: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    // collection all antennas
    let max_row = input.lines().count();
    let max_col = input.lines().nth(0).unwrap().chars().count();
    input.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, c)| {
            if c.is_alphanumeric() {
                let f_c = ant.get(&c);
                match f_c {
                    Some(v) => {
                        let mut new_v = v.clone();
                        new_v.push((row, col));
                        ant.insert(c, new_v);
                    }
                    None => {
                        let new_vec = vec![(row, col)];
                        ant.insert(c, new_vec);
                    }
                };
            }
        })
    });
    // now look for antinodes
    Some(find_antinodes2(ant,max_row as i64, max_col as i64, ))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    
    #[test]
    fn test_next_points() {
        let mut nodes:HashSet<(i64,i64)> = HashSet::new();
        let mut a = (2_i64,5_i64);
        let mut b = (3_i64, 7_i64);
        let i = 0;
        loop {
            let n =  calculate_next_points(a, b);
            if !is_valid(n, 11, 11) {
                break;
            }
            nodes.insert(n);
            a = b;
            b = n;
            
        }
        loop {
            let n =  calculate_reverse_next_points(a, b);
            if !is_valid(n, 11, 11) {
                break;
            }
            nodes.insert(n);
            b = a;
            a = n;
        }
        println!("{:?}", nodes);
    }
}
