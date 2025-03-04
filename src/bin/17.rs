
advent_of_code::solution!(17);

fn parse_register(s: &str) -> Register {
    let parts = s.split(' ').collect::<Vec<&str>>();
    assert_eq!(parts.len(), 3);
    parts[2].parse().unwrap()
}
fn parse_program(s: &str, program: &mut Vec<Instruction>) {
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
                program.push(*s);
                program.push(*c);
            }
        });
}
type Register = u64;
type Operand = u64;
type Opcode = u64;
type OutValue = u64;
type InstructionPointer = usize;
type Instruction = u64;
const INSTRUCTION_SIZE: usize = 2_usize;

pub fn part_one(input: &str) -> Option<String> {
    let mut register_a: Register = 0_u64;
    let mut register_b: Register = 0_u64;
    let mut register_c: Register = 0_u64;
    let mut program: Vec<Instruction> = Vec::new();
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
        ra: register_a,
        rb: register_b,
        rc: register_c,
        ip: 0,
        operand: 0,
        opcode: 0,
    };
    while state.ip < program.len() {
        let opcode = program.get(state.ip)?;
        let operand = program.get(state.ip + 1)?;
        state.opcode = *opcode;
        state.operand = *operand;
        state = process_instruction(&state, &program, &mut out_buffer)?;
    }
    println!("out: {:?}", out_buffer);
    let out_str = out_buffer
        .iter()
        .map(|o| (*o).to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("out: {}", out_str);

    Some(out_str)
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
    program: &Vec<Instruction>,
    out_buffer: &mut Vec<OutValue>,
) -> Option<MachineState> {
    if let Some(opcode) = program.get(state.ip) {
        return match opcode {
            0 => adv(state),
            1 => bxl(state),
            2 => bst(state),
            3 => jnx(state),
            4 => bxc(state),
            5 => out(state, out_buffer),
            6 => bdv(state),
            7 => cdv(state),
            _ => None,
        };
    };
    None
}

fn cdv(state: &MachineState) -> Option<MachineState> {
    let value = get_operand_value(state);
    let new_rc = state.ra / (2_u64.pow(value as u32));
    let mut new_state = state.clone();
    new_state.rc = new_rc;
    new_state.ip += INSTRUCTION_SIZE;
    Some(new_state)
}

fn bdv(state: &MachineState) -> Option<MachineState> {
    let value = get_operand_value(state);
    let new_rb = state.ra / (2_u64.pow(value as u32));
    let mut new_state = state.clone();
    new_state.rb = new_rb;
    new_state.ip += INSTRUCTION_SIZE;
    Some(new_state)
}

fn out(state: &MachineState, out_buffer: &mut Vec<OutValue>) -> Option<MachineState> {
    let v = get_operand_value(state);
    let out_v = v % 8;
    let mut new_state = state.clone();
    new_state.ip += INSTRUCTION_SIZE;
    out_buffer.push(out_v);
    Some(new_state)
}

fn bxc(state: &MachineState) -> Option<MachineState> {
    let mut new_state = state.clone();
    let v = state.rb ^ state.rc;
    new_state.rb = v;
    new_state.ip += INSTRUCTION_SIZE;
    Some(new_state)
}

fn jnx(state: &MachineState) -> Option<MachineState> {
    let mut new_state = state.clone();
    if state.ra == 0 {
        new_state.ip += INSTRUCTION_SIZE;
    } else {
        new_state.ip = new_state.operand as InstructionPointer;
    }
    Some(new_state)
}

fn bst(state: &MachineState) -> Option<MachineState> {
    let mut new_state = state.clone();
    let v = get_operand_value(state);
    let new_rb = v % 8;
    new_state.rb = new_rb;
    new_state.ip += INSTRUCTION_SIZE;
    Some(new_state)
}

fn bxl(state: &MachineState) -> Option<MachineState> {
    let mut new_state = state.clone();
    let v = state.operand;
    let new_rb = v ^ state.rb;
    new_state.rb = new_rb;
    new_state.ip += INSTRUCTION_SIZE;
    Some(new_state)
}

fn adv(state: &MachineState) -> Option<MachineState> {
    let value = get_operand_value(state);
    let new_ra = state.ra / (2_u64.pow(value as u32));
    let mut new_state = state.clone();
    new_state.ra = new_ra;
    new_state.ip += INSTRUCTION_SIZE;
    Some(new_state)
}

fn get_operand_value(state: &MachineState) -> Operand {
    match state.operand {
        0..=3 => state.operand,
        4 => state.ra,
        5 => state.rb,
        6 => state.rc,
        _ => unreachable!(),
    }
}

fn find(target: Vec<OutValue>, ans: Register, program: &Vec<Instruction>) -> Option<Register> {
    if target.is_empty() {
        return Some(ans);
    }
    for t in 0..8 {
        let a =  ans <<3 | t;
        let mut state: MachineState = MachineState {
            ra: a,
            rb: 0,
            rc: 0,
            ip: 0,
            operand: 0,
            opcode: 0,
        };
        while state.ip < program.len() - 2 {
            let mut out_buffer = Vec::new();
            let opcode = program.get(state.ip)?;
            let operand = program.get(state.ip + 1)?;
            state.opcode = *opcode;
            state.operand = *operand;
            state = process_instruction(&state, &program, &mut out_buffer)?;
            if !out_buffer.is_empty() && !target.is_empty(){
                let last = target.last()?;
                let out = out_buffer.last()?;
                if  out == last {
                    let (next_target,_)  = target.split_at(target.len()-1);
                    let sub = find(next_target.to_vec(), a, program );
                    if sub.is_none() {
                        continue;
                    }
                    return sub
                }
            }
        }
    }
    None

}
// derived from https://github.com/hyperneutrino/advent-of-code/blob/main/2024/day17p2.py
pub fn part_two(input: &str) -> Option<u64> {
    let mut register_a: Register = 0_u64;
    let mut register_b: Register = 0_u64;
    let mut register_c: Register = 0_u64;
    let mut program: Vec<Instruction> = Vec::new();
    let out_buffer: Vec<OutValue> = Vec::new();
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
    let a = find(program.clone(),0, &program);
    println!("{:?}", a);
    a
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
