use std::collections::HashMap;

advent_of_code::solution!(24);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Wire {
    name: String,
    value: Option<i32>,
    
}
impl Wire {
    fn new(name: String, value:Option<i32>) -> Wire {
        Wire { name, value  }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operator {
    And,
    Xor,
    Or,
}
fn wire_value(wire:&str) -> Option<i32> {
    Option::from(wire.to_string().parse::<i32>().unwrap())
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate<'a> {
    inputs: (&'a Wire, &'a Wire),
    output: &'a Wire,
    operator: Operator,
}
impl<'a> Gate<'a> {
    fn new(inputs: (&'a Wire, &'a Wire), output: &'a Wire, operator: Operator) -> Gate<'a> {
        Gate { inputs, output, operator }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut wires = HashMap::new();
    let mut gates = Vec::new();
    for line in input.lines() {
        if line.contains(':') {
            let mut parts = line.split(": ").collect::<Vec<_>>();
            wires.insert(parts[0], Wire::new(parts[0].to_string(),wire_value(parts[1]) ));
            continue;
        };
        if !line.is_empty() {
            let mut parts = line.split(" ").collect::<Vec<_>>();
            let in1 = wires.get(parts[0]).unwrap_or(&Wire::new(parts[0].to_string(), None));
            let op = match parts[1] {
                "AND" => Operator::And,
                "OR" => Operator::Or,
                "Xor" => Operator::Xor,
                _ => unreachable!(),
            };
            let in2 = wires.get(parts[1]).unwrap_or(&Wire::new(parts[1].to_string(), None));
            let out = wires.get(parts[2]).unwrap_or(&Wire::new(parts[2].to_string(), None));
            let gate = Gate::new((in1,in2),out,op );
            gates.push(gate);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
