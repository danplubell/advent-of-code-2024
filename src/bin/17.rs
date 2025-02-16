advent_of_code::solution!(17);

fn parse_register(s: &str) -> Register {
    let parts = s.split(' ').collect::<Vec<&str>>();
    assert_eq!(parts.len(), 3);
    parts[2].parse().unwrap()
}
fn parse_program(s: &str, program: &mut Vec<(Opcode, Operand)>) {
    let parts = s.split(' ').collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    parts[1]
        .split(',')
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|s| (s[0].parse::<Opcode>().ok(), s[1].parse::<Operand>().ok()))
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(s, c)| {
            if let (Some(s), Some(c)) = (s, c) {
                program.push((*s, *c));
            }
        });
}
type Register = u64;
type Operand = u64;
type Opcode = u64;
type OutValue = u64;
type InstructionPointer = usize;

pub fn part_one(input: &str) -> Option<u32> {
    let mut register_a:Register = 0_u64;
    let mut register_b:Register = 0_u64;
    let mut register_c:Register = 0_u64;
    let mut program = Vec::new();
    let mut out_buffer: Vec<OutValue> = Vec::new();
    input.lines().for_each(|l| {
        if l.contains("Register A:") {
            register_a = parse_register(l.trim());
        }
        if l.contains("Register B:") {
            register_b = parse_register(l.trim());
        }
        if l.contains("Register C:") {
            register_c = parse_register(l.trim());
        }
        if l.contains("Program:") {
            parse_program(l.trim(), &mut program);
        }
    });
    println!("Register A: {}", register_a);
    println!("Register B: {}", register_b);
    println!("Register C: {}", register_c);
    println!("Program: {:?}", program);
    let mut state: MachineState = MachineState {
        ra: 0,
        rb: 0,
        rc: 0,
        ip: 0,
        operand: 0,
        opcode: 0,
    };
    while state.ip < program.len() {
        state = process_instruction(
            &state,
            &program,
            &mut out_buffer,
        )?;
    }
    None
}
#[derive(Debug, Clone, PartialEq)]
struct MachineState {
    ra: Register,
    rb: Register,
    rc: Register,
    ip: InstructionPointer,
    operand: Operand,
    opcode: Opcode,
}


fn process_instruction(
    state: &MachineState,
    program: &Vec<(Opcode,Operand)>,
    out_buffer: &mut Vec<OutValue>,
) -> Option<MachineState> {
    println!("instruction: {:?}", program.get(state.ip));
    
     if let Some((opcode,operand)) =  program.get(state.ip) {
         return match opcode {
            0 => adv(state),
            1 => bxl(state),
            2 => bst(state),
            3 => jnx(state),
            4 => bxc(state),
            5 => out(state, out_buffer),
            6 => bdv(state),
            7 => cdv(state),
            _ => None
        };
    };
    None
}

fn cdv(p0: &MachineState) -> Option<MachineState> {
    todo!()
}

fn bdv(p0: &MachineState) -> Option<MachineState> {
    todo!()
}

fn out(state: &MachineState, out_buffer: &mut Vec<OutValue>) -> Option<MachineState> {
    let v = get_operand_value(state);
    let out_v = v%8;
    let mut new_state = state.clone();
    new_state.ip += 1;
    out_buffer.push(out_v);
    Some(new_state)
}

fn bxc(p0: &MachineState) -> Option<MachineState> {
    todo!()
}

fn jnx(p0: &MachineState) -> Option<MachineState> {
    todo!()
}

fn bst(p0: &MachineState) -> Option<MachineState> {
    todo!()
}

fn bxl(p0: &MachineState) -> Option<MachineState> {
    todo!()
}

fn adv(state: &MachineState) -> Option<MachineState> {
    let value = get_operand_value(state);
    let new_ra = state.ra/(2^value);
    let mut new_state = state.clone();
    new_state.ra = new_ra;
    new_state.ip += 1;
    Some(new_state)
}

fn get_operand_value(state: &MachineState) -> Operand {
    match state.operand {
        0..=3=> state.operand,
        4 => state.ra,
        5 => state.rb,
        6 => state.rc,
        _ => unreachable!()
    }
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
