use std::collections::{BTreeMap, HashMap, HashSet};

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
impl Operator {
    fn to_string(&self) -> &'static str {
        match self {
            Operator::And => "And",
            Operator::Xor => "Xor",
            Operator::Or => "Or",
        }
    }
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
            let l: Vec<_> = wires.values().filter(|w| w.name.starts_with("z")).collect();

            let mut l = l
                .iter()
                .map(|w| {
                    if w.value.is_some() {
                        w.value.unwrap().to_string()
                    } else {
                        "0".to_string()
                    }
                })
                .collect::<Vec<_>>();
            let l = l.iter().rev().cloned().collect::<Vec<_>>();
            let l = l.join("");

            let v = u64::from_str_radix(&l, 2).unwrap();
            println!("gate: {} {:?}", v, gate);
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

pub fn part_two(input: &str) -> Option<u64> {
    let mut wires: BTreeMap<String, Wire> = BTreeMap::new();
    let mut formulas = HashMap::new();
    let mut formulas_strings = Vec::new();

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
        formulas_strings.push(line.to_string());
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
        formulas.insert(parts[4].to_string(), gate);
    }
    for i in 0..=45 {
        if !verify(i,&formulas) {
            let wire = "z".to_owned() + &i.to_string();
            println!("failed on: {}", wire);
            break;
        }
    }
    
//    println!("{}",verify(0, &formulas));
    /*
    let keys = formulas.keys().collect::<Vec<_>>();
    for k in keys {
        if k.chars().nth(0).unwrap() == 'z' {
            let p = pp(k, &formulas, 0);
            println!("{}", p);
        }
    }
    
     */
    //    let p = pp("z02", &formulas, 0);
    //    println!("part two: {}", p);

    None
}
/*
pub fn part_two_0(input: &str) -> Option<u32> {
    let mut wires: BTreeMap<String, Wire> = BTreeMap::new();
    let mut gates = Vec::new();
    let mut outputs = Vec::new();
    let mut formulas = Vec::new();
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
        formulas.push(line.to_string());
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
    let l = z_list
        .iter()
        .map(|w| w.value.unwrap().to_string())
        .collect::<Vec<_>>();
    let l = l.iter().rev().cloned().collect::<Vec<_>>();
    let z_bin_string = l.join("");
    let z = u64::from_str_radix(&z_bin_string, 2).unwrap();

    println!("x={} y={} x+y={} z={} {}",x,y, y + x, z, z_bin_string);
    None
}

 */
// 55544677167336
//55544677167336
fn verify_z(wire: &str, gates: &HashMap<String, Gate>, num: usize) -> bool {
    println!("vx {} {}", wire, num);
    let mut gate = gates.get(wire).unwrap();
    let x = gate.inputs.0.clone();
    let y = gate.inputs.1.clone();

    if gate.operator != Operator::Xor {
        return false;
    }
    if num == 0 {
        let mut inputs = vec![x, y];
        inputs.sort();
        return inputs == vec!["x00", "y00"];
    }
    verify_intermediate_xor(&x, gates, num) && verify_carry_bit(&y, gates, num)
        || verify_intermediate_xor(&y, gates, num) && verify_carry_bit(&y, gates, num)
}

fn verify_carry_bit(wire: &str, gates: &HashMap<String, Gate>, num: usize) -> bool {
    println!("vc {} {}", wire, num);
    let gate = gates.get(wire).unwrap();
    let x = gate.inputs.0.clone();
    let y = gate.inputs.1.clone();
    let wire_x = "x".to_owned() + &num.to_string();
    let wire_y = "y".to_owned() + &num.to_string();

    if num == 1 {
        if gate.operator != Operator::And {
            return false;
        }
        let mut inputs = vec![x, y];
        inputs.sort();

        return inputs == vec!["x00", "y00"];
    }
    if gate.operator != Operator::Or {
        return false;
    }
    verify_direct_carry(&x, gates, num - 1) && verify_recarry(&y, gates, num - 1)
        || verify_direct_carry(&y, gates, num - 1) && verify_recarry(&x, gates, num - 1)
}

fn verify_recarry(wire: &str, gates: &HashMap<String, Gate>, num: usize) -> bool {
    println!("vr {} {}", wire, num);
    let gate = gates.get(wire).unwrap();
    let x = gate.inputs.0.clone();
    let y = gate.inputs.1.clone();
    if gate.operator != Operator::And {
        return false;
    }
    verify_intermediate_xor(&x, gates, num) && verify_carry_bit(&y, gates, num)
        || verify_intermediate_xor(&y, gates, num) && verify_carry_bit(&x, gates, num)
}

fn verify_direct_carry(wire: &str, gates: &HashMap<String, Gate>, num: usize) -> bool {
    println!("vd {} {}", wire, num);
    let gate = gates.get(wire).unwrap();
    let x = gate.inputs.0.clone();
    let y = gate.inputs.1.clone();
    let wire_x = "x".to_owned() + format!("{:0width$}", num, width = 2).as_str();
    let wire_y = "y".to_owned() + format!("{:0width$}", num, width = 2).as_str();
    let mut inputs = vec![x, y];
    inputs.sort();

    if gate.operator != Operator::And {
        return false;
    }
    inputs == vec![wire_x, wire_y]
}
fn verify(num: usize, gates: &HashMap<String, Gate>) -> bool {
    let wire = "z".to_owned() + format!("{:0width$}", num, width = 2).as_str();

    verify_z(&wire, gates, num)
}
fn verify_intermediate_xor(wire: &str, gates: &HashMap<String, Gate>, num: usize) -> bool {
    println!("vx {} {}", wire, num);
    let gate = gates.get(wire).unwrap();
    if gate.operator != Operator::Xor {
        return false;
    }
    let x = gate.inputs.0.clone();
    let y = gate.inputs.1.clone();
    let wire_x = "x".to_owned() + format!("{:0width$}", num, width = 2).as_str();
    let wire_y = "y".to_owned() + format!("{:0width$}", num, width = 2).as_str();
    let mut inputs = vec![x, y];
    inputs.sort();
    inputs == vec![wire_x, wire_y]
}

fn pp(wire: &str, gates: &HashMap<String, Gate>, depth: usize) -> String {
    if "xy".contains(wire.chars().nth(0).unwrap()) {
        return "  ".repeat(depth) + wire;
    };
    let gate = gates.get(wire).unwrap();
    "  ".repeat(depth)
        + &*gate.operator.to_string()
        + " ("
        + wire
        + ")\n"
        + &*pp(&gate.inputs.0, gates, depth + 1)
        + "\n"
        + &*pp(&gate.inputs.1, gates, depth + 1)
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
