use itertools::Itertools;

advent_of_code::solution!(13);

fn find_match(target: i32, a: i32, b: i32) -> Vec<(i32, i32)> {
    let mut list: Vec<(i32, i32)> = vec![];
    for i in (0..=target).rev() {
        if i % a == 0 {
            for j in 0..=target {
                if j % b == 0 {
                    let total = i + j;
                    if total == target {
                        list.push((i / a, j / b))
                    }
                }
            }
        }
    }
    list
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Button {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Prize {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Claw {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}
pub fn part_one(input: &str) -> Option<u32> {
    // build the list of claw machines
    let mut total = 0;
    for chunk in &input.lines().chunks(4) {
        let claw_parts = chunk.collect::<Vec<&str>>();
        let a: Button = parse_button(claw_parts[0]);
        let b: Button = parse_button(claw_parts[1]);
        let prize: Prize = parse_prize(claw_parts[2]);
        //make buttonA
        //make buttonB
        //make prize
        total += get_cost(a, b, prize);
    }
    // for each claw machine get a list of x tuples that match that total the x value
    // tuple is (a clicks, b clicks)
    // for each claw machine get a list of y tuples that match that total the y value
    // if y or x is empty then continue
    // look for an x tuple that matches a y tuple
    // calculate the cost of tokens a_clicks * 3, b_clicks * 1
    // add cost to total
    Some(total as u32)
}

fn get_cost(button_a: Button, button_b: Button, prize: Prize) -> i32 {
    let x_matches =  find_match(prize.x, button_a.x, button_b.x);
    let y_matches = find_match(prize.y, button_a.y, button_b.y);
//    println!("x_matches: {:?}, y_matches: {:?}", x_matches, y_matches);
    if x_matches.is_empty() || y_matches.is_empty() {
        return 0;
    }
    let mut common_matches:Vec<(i32,i32)> = vec![];
    let mut total_cost = 0;
    for m in x_matches {
        let found = y_matches.contains(&m);
        if found {
            let cost = m.0 * 3 + m.1;
            if cost > total_cost {
                total_cost = cost;
                common_matches.push(m);
            }
        }
    }
    total_cost
}

fn parse_prize(prize_str: &str) -> Prize {
    let parts = parse_claw(prize_str);
    Prize {
        x: parts.0,
        y: parts.1,
    }

}

fn parse_claw(part_str: &str) -> (i32,i32) {
    let tokens = part_str
        .split(' ')
        .filter(|x| x.starts_with('X') || x.starts_with('Y'))
        .collect::<Vec<&str>>();
    let x_y: Vec<_> = tokens.iter().map(|t|{
        t.chars().filter(|&c| c.is_ascii_digit()).collect::<String>()
    }).collect();
    let x = x_y[0].parse::<i32>().unwrap();
    let y = x_y[1].parse::<i32>().unwrap();
    (x, y)
}
fn parse_button(button_str: &str) -> Button {
    let parts = parse_claw(button_str);
    Button {
        x: parts.0,
        y: parts.1,
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_parse_button() {
        let button_str = "Button A: X+94, Y+34\n";
        let button = parse_button(button_str);
        assert_eq!(button.x, 94);
        assert_eq!(button.y, 34);
    }
    #[test]
    fn test_parse_prize() {
        let prize_str = "Prize: X=7870, Y=6450\n";
        let prize = parse_prize(prize_str);
        assert_eq!(prize.x, 7870);
        assert_eq!(prize.y, 6450);
    }

}
