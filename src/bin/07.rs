use std::collections::HashMap;

advent_of_code::solution!(7);

fn pop(stack: &mut Vec<Option<Token>>) -> Option<Token> {
    stack.pop()?
}
fn push(stack: &mut Vec<Option<Token>>, t: Option<Token>) {
    stack.push(t);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Token {
    Number(u64),
    Plus,
    Multiply,
    Concatenate,
}
fn tokenize(s: &str) -> Option<Token> {
    s.parse::<u64>().ok().map(Token::Number).or(match s {
        PLUS => Some(Token::Plus),
        MULTIPLY => Some(Token::Multiply),
        CONCATENATE => Some(Token::Concatenate),
        _ => None,
    })
}
fn evaluate(input: Vec<&str>) -> Option<u64> {
    let input_copy = input.clone();
    let mut stack: Vec<Option<Token>> = vec![];
    input.iter().enumerate().for_each(|(i, s)| {
        let token = tokenize(s);
        match token {
            Some(Token::Concatenate) => push(&mut stack, token),
            Some(Token::Plus) | Some(Token::Multiply) => {
                push(&mut stack, token);
            }
            Some(Token::Number(v)) => {
                // pop the operation off the stack
                let operation = pop(&mut stack);
                match operation {
                    Some(Token::Concatenate) => {
                        let right_n = pop(&mut stack);
                        if let Some(Token::Number(rn)) = right_n {
                            let combined = format!("{}{}", rn, v);
                            let new_number = combined.parse::<u64>().unwrap();
                            push(&mut stack, Some(Token::Number(new_number)));
                        }
                    }
                    Some(Token::Multiply) => {
                        // pop the number of the stack
                        let n = pop(&mut stack);
                        if let Some(Token::Number(n)) = n {
                            let r = n.checked_mul(v);
                            if let Some(v) = r {
                                push(&mut stack, Some(Token::Number(v)))
                            }
                        }
                    }
                    Some(Token::Plus) => {
                        // pop the number of the stack
                        let n = pop(&mut stack);
                        if let Some(Token::Number(n)) = n {
                            let r = n.checked_add(v);
                            if let Some(v) = r {
                                push(&mut stack, Some(Token::Number(v)))
                            }
                        }
                    }
                    _ => push(&mut stack, Some(Token::Number(v))),
                }
            }
            None => {}
        }
    });
    let t = pop(&mut stack);
    match t {
        Some(Token::Number(v)) => Some(v),
        _ => None,
    }
}

const PLUS: &str = "+";
const MULTIPLY: &str = "*";
const CONCATENATE: &str = "~";

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for l in input.lines() {
        //parse out the value from the list of value
        let s = l.split(':').collect::<Vec<&str>>();

        let target_result = s[0].parse::<u64>();

        let target = target_result.unwrap_or(0u64);

        let numbers = s[1].trim().split(' ').collect::<Vec<&str>>();

        // now go through all the other patterns
        //generate patterns
        // how big of a number do I need?

        let pattern_len = 64 - numbers.len() as u64; // length of the operator list
        let loop_len = u64::MAX >> pattern_len; // this is the number of iterations

        for i in 0..=loop_len {
            let f = format!("{:064b}", i);

            let sub = f.split_at(pattern_len as usize);
            let vec: Vec<_> = sub
                .1
                .chars()
                .map(|c| match c {
                    '0' => PLUS,
                    _ => MULTIPLY,
                })
                .collect();

            let mut to_solve: Vec<_> = numbers
                .iter()
                .zip(vec.iter())
                .flat_map(|(&x, y)| vec![x, y])
                .collect();

            to_solve.pop();
            let result = evaluate(to_solve).unwrap_or(0);
            if result == target {
                let r = total.checked_add(target);
                match r {
                    Some(r) => total = r,
                    None => return None,
                }
                break;
            }
        }
    }
    Some(total)
}
fn generate_patterns<'a>(len: usize) -> Vec<Vec<&'a str>> {
    let base_p = (0..len).into_iter().map(|i| PLUS).collect();
    let mut list: Vec<Vec<&str>> = vec![base_p];
    let symbols = [PLUS, MULTIPLY, CONCATENATE];

    for x in 0..len {
        for n in 0..list.len() {
            let patterns = list.clone();
            let l = patterns.get(n).unwrap();

            for i in 1..3 {
                let mut new_pattern = l.clone();
                new_pattern[x % 3] = symbols[i];
                list.push(new_pattern);
            }
        }
    }
    list
}

fn generate_permutations(n: usize) -> Vec<Vec<&'static str>> {
    let operators = vec!["+", "*", "~"];
    let mut result = Vec::new();

    // For a list of length n, we'll have 3^n total permutations
    let total_permutations = 3_usize.pow(n as u32);

    for i in 0..total_permutations {
        let mut current_perm = Vec::with_capacity(n);
        let mut num = i;

        // Convert number to base-3 representation
        for _ in 0..n {
            let operator_index = num % 3;
            current_perm.push(operators[operator_index]);
            num /= 3;
        }

        result.push(current_perm);
    }

    result
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    let pattern_hash: HashMap<usize, Vec<Vec<&str>>> = HashMap::new();
    for l in input.lines() {
        println!("line: {:?}", l);
        //parse out the value from the list of value
        let s = l.split(':').collect::<Vec<&str>>();

        let target_result = s[0].parse::<u64>();

        let target = target_result.unwrap_or(0u64);

        let numbers = s[1].trim().split(' ').collect::<Vec<&str>>();
        let patterns_opt = pattern_hash.get(&numbers.len());
        let patterns = match patterns_opt {
            Some(p) => p,
            None => &generate_permutations(numbers.len()),
        };
        println!("process patterns: {}", patterns.len());
        for (i, pattern) in patterns.iter().enumerate() {
            let mut to_solve: Vec<_> = numbers
                .iter()
                .zip(pattern.iter())
                .flat_map(|(&x, y)| vec![x, y])
                .collect();
            to_solve.pop();

            let result = evaluate(to_solve).unwrap_or(0);
            if result == target {
                let r = total.checked_add(target);
                match r {
                    Some(r) => total = r,
                    None => (),
                }
                break;
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
    #[test]
    fn test_tokenize() {
        let t = tokenize(PLUS).unwrap();
        assert_eq!(t, Token::Plus);
        let t = tokenize(MULTIPLY).unwrap();
        assert_eq!(t, Token::Multiply);
        let t = tokenize("-");
        assert_eq!(t, None);
        let t = tokenize("1").unwrap();
        assert_eq!(t, Token::Number(1));
    }
    #[test]
    fn text_evaluate() {
        /*
        assert_eq!(evaluate(vec![]), None);
        let s = vec!["1"];
        assert_eq!(evaluate(s), Some(1));
        let s = vec!["1", "2"];
        assert_eq!(evaluate(s), Some(2));
        let s = vec!["1", "+", "3"];
        assert_eq!(evaluate(s), Some(4));
        let s = vec!["2", "*", "3"];
        assert_eq!(evaluate(s), Some(6));

         */
        let s = vec![
            "442", "*", "2", "*", "326", "*", "89", "+", "3", "*", "4", "*", "1", "+", "7", "+",
            "75",
        ];
        assert_eq!(evaluate(s), Some(326));
    }
    #[test]
    fn test_generate() {
        let n = 2;
        let l = generate_patterns(n);
        assert_eq!(l.len(), 3_usize.pow(n as u32));
        println!("{:?} {}", l, l.len());
    }
    #[test]
    fn test_permutations() {
        let n = 11;
        let l = generate_permutations(n);
        assert_eq!(l.len(), 3_usize.pow(n as u32));
        println!(" {}", l.len());
    }
}
