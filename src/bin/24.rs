use std::collections::HashMap;

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
pub fn part_one(input: &str) -> Option<u32> {
    let mut wires = HashMap::new();
    let mut gates = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            let parts = line.split(": ").collect::<Vec<_>>();
            wires.insert(
                parts[0],
                Wire::new(parts[0].to_string(), wire_value(parts[1])),
            );
            continue;
        };
        let parts = line.split(" ").collect::<Vec<_>>();
        wires.entry(parts[0]).or_insert_with(|| Wire::new(parts[0].to_string(), wire_value(parts[0])));
        wires.entry(parts[2]).or_insert_with(|| Wire::new(parts[2].to_string(), wire_value(parts[2])));
        wires.entry(parts[4]).or_insert_with(|| Wire::new(parts[4].to_string(), wire_value(parts[4])));

        let op = match parts[1] {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "Xor" => Operator::Xor,
            _ => unreachable!(),
        };
        let gate = Gate::new((parts[0].to_string(), parts[2].to_string()), parts[4].to_string(), op);
        gates.push(gate);
    }
    let mut done = false;
    // loop through the gates and apply the operations until there aren't anymore None values
    while !done {
        done = true;
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
