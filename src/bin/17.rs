advent_of_code::solution!(17);

fn parse_register(s: &str) -> i32 {
    let parts = s.split(' ').collect::<Vec<&str>>();
    assert_eq!(parts.len(), 3);
    parts[2].parse().unwrap()
}
fn parse_program(s: &str, program: &mut Vec<(i32, i32)>) {
    let parts = s.split(' ').collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    parts[1]
        .split(',')
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|s| (s[0].parse::<i32>().ok(), s[1].parse::<i32>().ok()))
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(s, c)| {
            if let (Some(s), Some(c)) = (s, c) {
                program.push((*s, *c));
            }
        });
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut register_a = 0_i32;
    let mut register_b = 0_i32;
    let mut register_c = 0_i32;
    let mut program = Vec::new();
    let mut i_ptr: usize = 0;
    let mut out_buffer: Vec<i32> = Vec::new();
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
    while i_ptr < program.len() {
        i_ptr = process_instruction(
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut i_ptr,
            &program,
            &mut out_buffer,
        )?;
    }
    None
}

fn process_instruction(
    ra: &mut i32,
    rb: &mut i32,
    rc: &mut i32,
    i_ptr: &mut usize,
    program: &[(i32, i32)],
    out_buffer: &mut Vec<i32>,
) -> Option<usize> {
    println!("instruction: {:?}", program.get(*i_ptr));
    
     if let Some((opcode,operand)) =  program.get(*i_ptr){
         return match opcode {
            0 => adv(ra, rb, rc, operand,i_ptr),
            1 => bxl(ra, rb, rc, operand,*i_ptr),
            2 => bst(ra, rb, rc, operand,*i_ptr),
            3 => jnx(ra, rb, rc, operand,*i_ptr),
            4 => bxc(ra, rb, rc, operand,*i_ptr),
            5 => out(ra, rb, rc, operand, out_buffer, *i_ptr),
            6 => bdv(ra, rb, rc, operand,*i_ptr),
            7 => cdv(ra, rb, rc, operand,*i_ptr),
            _ => None
        };
    };
    None
}

fn cdv(p0: &mut i32, p1: &mut i32, p2: &mut i32, p3: &i32, p4: usize) -> Option<usize> {
    todo!()
}

fn bdv(p0: &mut i32, p1: &mut i32, p2: &mut i32, p3: &i32, p4: usize) -> Option<usize> {
    todo!()
}

fn out(p0: &mut i32, p1: &mut i32, p2: &mut i32, p3: &i32, p4: &mut Vec<i32>, p5: usize) -> Option<usize> {
    todo!()
}

fn bxc(p0: &mut i32, p1: &mut i32, p2: &mut i32, p3: &i32, p4: usize) -> Option<usize> {
    todo!()
}

fn jnx(p0: &mut i32, p1: &mut i32, p2: &mut i32, p3: &i32, p4: usize) -> Option<usize> {
    todo!()
}

fn bst(p0: &mut i32, p1: &mut i32, p2: &mut i32, p3: &i32, p4: usize) -> Option<usize> {
    todo!()
}

fn bxl(p0: &mut i32, p1: &mut i32, p2: &mut i32, p3: &i32, p4: usize) -> Option<usize> {
    todo!()
}

fn adv(ra: &mut i32, rb: &mut i32, rc: &mut i32, operand: &i32, i_ptr: &mut usize) -> Option<usize> {
    let value = get_operand_value(*operand, *ra, *rb, *rc)?;
    ra = 1;//ra/(2^value)
    let i = *i_ptr;
    Some(i + 1)
}

fn get_operand_value(operand: i32 , ra: i32, rb: i32, rc: i32) -> Option<i32> {
    match operand {
        0_i32..=3_i32=> Some(operand),
        4 => Some(ra),
        5 => Some(rb),
        6 => Some(rc),
        _ => None
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
