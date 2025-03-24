use std::collections::{BTreeSet, HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let mut node_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let split = line.split('-').collect::<Vec<_>>();
        node_map.entry(split[0]).or_default().insert(split[1]);
        node_map.entry(split[1]).or_default().insert(split[0]);
    }
    let mut triples = HashSet::new();
    for node in node_map.iter() {
        for node1 in node_map.iter() {
            let l = node1.1;
            let target = node.0;
            // does the node contain the parent
            if let Some(s) = l.iter().find(|n| *n == target) {
                // the target is in the list
                // compare lists for common nodes
                node.1.iter().for_each(|n1| {
                    if node1.1.contains(n1) {
                        let mut t = vec![target, *n1, node1.0];
                        t.sort();
                        let mut has_t = false;
                        t.iter().for_each(|n2| {
                            let c = n2.to_string().chars().next().unwrap();
                            if c == 't' {
                                has_t = true;
                            }
                        });
                        if has_t {
                            triples.insert(t);
                        }
                    }
                })
            }
        }
    }
    Some(triples.len())
}
fn search<'a>(
    node: (&&'a str, &'a std::collections::HashSet<&'a str>),
    node_map: &'a std::collections::HashMap<&'a str, std::collections::HashSet<&'a str>>,
    required: &mut std::collections::BTreeSet<&'a &'a str>
) {
    for n in node.1.iter() {
        if required.contains(n) {
            continue;
        }
        // all elements in required 
        let list = node_map.get(n).unwrap();
        let r = required.iter().all(|q| {
           list.contains(*q) });
        if !r {
            continue;
        }
        let nt = node_map.get(n).unwrap();
        required.insert(n);
        search((n,nt), node_map, required);
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let mut node_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let split = line.split('-').collect::<Vec<_>>();
        node_map.entry(split[0]).or_default().insert(split[1]);
        node_map.entry(split[1]).or_default().insert(split[0]);
    }
    let mut l = BTreeSet::new();
    for node in node_map.iter() {
        let mut required = BTreeSet::new();
        required.insert(node.0);
        search(node, &node_map, &mut required);
        
        l.insert(required);
    }
//    println!("node_map: {:?}", node_map);
//    println!{"{:?}", l}
    let mut max_len = 0usize;
    let mut max_party = BTreeSet::new();
    l.iter().for_each(|p| {
        if p.len() > max_len {
            max_len = p.len();
            max_party = p.clone();
        }
    });
    Some(max_party.iter().join(","))
}
// ar,ep,ih,ju,jx,le,ol,pk,pm,pp,xf,yu,zg == good
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
