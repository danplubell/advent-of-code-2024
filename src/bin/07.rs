advent_of_code::solution!(7);

fn pop(stack: &mut Vec<Option<Token>>) -> Option<Token> {
    stack.pop()?
}
fn push(stack: &mut Vec<Option<Token>>, t: Option<Token>) {
    stack.push(t);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Token {
    Number(u32),
    Plus,
    Multiply,
}
fn tokenize(s: &str) -> Option<Token> {
    s.parse::<u32>().ok().map(Token::Number).or(match s {
        "+" => Some(Token::Plus),
        "*" => Some(Token::Multiply),
        _ => None,
    })
}
fn evaluate(input: Vec<&str>) -> Option<u32> {
    let mut stack: Vec<Option<Token>> = vec![];
    input.iter().for_each(|s| {
        let token = tokenize(s);
        match token {
            Some(Token::Plus) | Some(Token::Multiply) => {
                push(&mut stack, token);
            }
            Some(Token::Number(v)) => {
                // pop the operation off the stack
                let operation = pop(&mut stack);
                match operation {
                    Some(Token::Multiply) => {
                        // pop the number of the stack
                        let n = pop(&mut stack);
                        if let Some(Token::Number(n)) = n {
                            push(&mut stack, Some(Token::Number(n * v)))
                        }
                    }
                    Some(Token::Plus) => {
                        // pop the number of the stack
                        let n = pop(&mut stack);
                        if let Some(Token::Number(n)) = n {
                            push(&mut stack, Some(Token::Number(n + v)))
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

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for l in input.lines() {
        //parse out the value from the list of value
        let s = l.split(':').collect::<Vec<&str>>();
        let target = s[0].parse::<u32>().ok()?;
        let numbers = s[1].trim().split(' ').collect::<Vec<&str>>();
        let plus = vec!["+"; numbers.len()];
        let mut all_plus: Vec<&str> = numbers
            .iter()
            .zip(plus.iter())
            .flat_map(|(&x, &y)| vec![x, y])
            .collect();
        all_plus.pop();
        let result = evaluate(all_plus)?;
        println!("add {} {} ", target, result);
        if result == target {
            total += target;
            continue;
        }

        let mul = vec!["*"; numbers.len()];
        let mut all_mul: Vec<&str> = numbers
            .iter()
            .zip(mul.iter())
            .flat_map(|(&x, &y)| vec![x, y])
            .collect();
        all_mul.pop();
        let result = evaluate(all_mul)?;
        println!("mul {} {} ", target, result);
        if result == target {
            total += target;
            continue;
        }
        // now go through all the other patterns
    }
    Some(total)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_tokenize() {
        let t = tokenize("+").unwrap();
        assert_eq!(t, Token::Plus);
        let t = tokenize("*").unwrap();
        assert_eq!(t, Token::Multiply);
        let t = tokenize("-");
        assert_eq!(t, None);
        let t = tokenize("1").unwrap();
        assert_eq!(t, Token::Number(1));
    }
    #[test]
    fn text_evaluate() {
        assert_eq!(evaluate(vec![]), None);
        let s = vec!["1"];
        assert_eq!(evaluate(s), Some(1));
        let s = vec!["1", "2"];
        assert_eq!(evaluate(s), Some(2));
        let s = vec!["1", "+", "3"];
        assert_eq!(evaluate(s), Some(4));
        let s = vec!["2", "*", "3"];
        assert_eq!(evaluate(s), Some(6));
    }
}
