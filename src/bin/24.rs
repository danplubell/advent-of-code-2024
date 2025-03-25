use std::collections::{BTreeMap, HashMap};

advent_of_code::solution!(24);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Wire {
    name: String,
    value: Option<i32>,
}
impl Wire {
    fn new(name: String, value: Option<i32>) -> Wire {
        Wire { name, value }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operator {
    And,
    Xor,
    Or,
}
fn wire_value(wire: &str) -> Option<i32> {
    Option::from(wire.to_string().parse::<i32>().unwrap())
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    inputs: (String, String),
    output: String,
    operator: Operator,
}
impl Gate {
    fn new(inputs: (String, String), output: String, operator: Operator) -> Gate {
        Gate {
            inputs,
            output,
            operator,
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut wires: BTreeMap<String, Wire> = BTreeMap::new();
    let mut gates = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            let parts = line.split(": ").collect::<Vec<_>>();
            wires.insert(
                parts[0].parse().unwrap(),
                Wire::new(parts[0].to_string(), wire_value(parts[1])),
            );
            continue;
        };
        let parts = line.split(" ").collect::<Vec<_>>();
        wires
            .entry(parts[0].parse().unwrap())
            .or_insert_with(|| Wire::new(parts[0].to_string(), None));
        wires
            .entry(parts[2].parse().unwrap())
            .or_insert_with(|| Wire::new(parts[2].to_string(), None));
        wires
            .entry(parts[4].parse().unwrap())
            .or_insert_with(|| Wire::new(parts[4].to_string(), None));

        let op = match parts[1] {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => unreachable!(),
        };
        let gate = Gate::new(
            (parts[0].to_string(), parts[2].to_string()),
            parts[4].to_string(),
            op,
        );
        gates.push(gate);
    }
    let mut done = false;
    // loop through the gates and apply the operations until there aren't anymore None values
    while !done {
        let mut cnt = 0;
        for gate in &mut gates {
            if !calculate_gate(gate, &mut wires) {
                cnt += 1;
            }
        }
        done = cnt == 0;
    }
//    wires.iter().for_each(|w| println!("{:?} {:?}", w.1.name, w.1.value));
//    let mut values = wires.values().map(|w|format!("{:?}: {:?}",w.name, w.value)).collect::<Vec<_>>();
//    values.sort();
   // println!("{:?}",values);
    let l: Vec<_> = wires.values().filter(|w| w.name.starts_with("z")).collect();
   

    let mut l = l
        .iter()
        .map(|w| w.value.unwrap().to_string())
        .collect::<Vec<_>>();
    let l = l.iter().rev().cloned().collect::<Vec<_>>();
    let l = l.join("");
    

    Some(u64::from_str_radix(&l, 2).unwrap())
}

fn calculate_gate(gate: &mut Gate, wires: &mut BTreeMap<String, Wire>) -> bool {
    let out = wires.get(&gate.output);
    let name = out.unwrap().name.as_str();
//    let out_v = match out {
//        Some(wire) => wire.value,
//        None => None,
//    };
//    if out_v.is_some() {
//        return true;
//    };
    let in1 = wires.get(gate.inputs.0.as_str()).and_then(|w| w.value);
    if in1.is_none() {
        return false;
    }
    let in2 = wires.get(gate.inputs.1.as_str()).and_then(|w| w.value);
    if in2.is_none() {
        return false;
    }
    let i1 = in2.unwrap();
    let i2 = in1.unwrap();
    let v: i32 = match gate.operator {
        Operator::And => i1 & i2,
        Operator::Xor => i1 ^ i2,
        Operator::Or => i1 | i2,
    };
    
    wires.insert(name.to_string(), Wire::new(name.to_string(), Some(v)));
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut wires: BTreeMap<String, Wire> = BTreeMap::new();
    let mut gates = Vec::new();
    let mut outputs = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            let parts = line.split(": ").collect::<Vec<_>>();
            wires.insert(
                parts[0].parse().unwrap(),
                Wire::new(parts[0].to_string(), wire_value(parts[1])),
            );
            continue;
        };
        let parts = line.split(" ").collect::<Vec<_>>();
        wires
            .entry(parts[0].parse().unwrap())
            .or_insert_with(|| Wire::new(parts[0].to_string(), None));
        wires
            .entry(parts[2].parse().unwrap())
            .or_insert_with(|| Wire::new(parts[2].to_string(), None));
        wires
            .entry(parts[4].parse().unwrap())
            .or_insert_with(|| Wire::new(parts[4].to_string(), None));

        let op = match parts[1] {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => unreachable!(),
        };
        let gate = Gate::new(
            (parts[0].to_string(), parts[2].to_string()),
            parts[4].to_string(),
            op,
        );
        gates.push(gate);
        outputs.push(parts[4].to_string());
    }
    let temp_gates = Vec::new();
    gates.iter().for_each(|g| {
        gates.iter().for_each(|g| {
            //swap
            
        })
    });
    let x_list: Vec<_> = wires.values().filter(|w| w.name.starts_with("x")).collect();
    let l = x_list
        .iter()
        .map(|w| w.value.unwrap().to_string())
        .collect::<Vec<_>>();
    let l = l.iter().rev().cloned().collect::<Vec<_>>();
    let x_bin_string = l.join("");
    let x = u64::from_str_radix(&x_bin_string, 2).unwrap();
    
    let y_list: Vec<_> = wires.values().filter(|w| w.name.starts_with("y")).collect();
    let l = y_list
        .iter()
        .map(|w| w.value.unwrap().to_string())
        .collect::<Vec<_>>();
    let l = l.iter().rev().cloned().collect::<Vec<_>>();
    let y_bin_string = l.join("");
    let y = u64::from_str_radix(&y_bin_string, 2).unwrap();

    let z_list: Vec<_> = wires.values().filter(|w| w.name.starts_with("z")).collect();
    let l = y_list
        .iter()
        .map(|w| w.value.unwrap().to_string())
        .collect::<Vec<_>>();
    let l = l.iter().rev().cloned().collect::<Vec<_>>();
    let z_bin_string = l.join("");
    let z = u64::from_str_radix(&z_bin_string, 2).unwrap();

    println!("z={} y={} x+y={} z={}",x,y, y + x, z);

    None
}
// 55544677167336

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
