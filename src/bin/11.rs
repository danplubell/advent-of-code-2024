use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use rayon::iter::split;

advent_of_code::solution!(11);

enum StoneType {
    IsZero,
    IsEvenDigits,
    Multiply,
}
fn is_even_digits(n: i64) -> bool {
    n.abs().to_string().len() % 2 == 0
}
fn split_stone(n: i64) -> (i64,i64) {
    let abs_n = n.abs();
    let n_str  = abs_n.to_string();
    let s = n_str.split_at(n_str.len()/2);
    (s.0.parse::<i64>().unwrap(), s.1.parse::<i64>().unwrap())
}
pub fn part_one(input: &str) -> Option<usize> {
    let list: Vec<_> = input.lines().nth(0)?.split(" ").collect();
    let stones: Vec<i64> = list.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    let mut stone_list: Vec<i64> = stones.clone();
    let mut working_list: Vec<i64> = vec![];
    (0..25).for_each(|i| {
        
        stone_list.iter().for_each(|n| {
            let stone_type: StoneType;
            if *n == 0 {
                stone_type = StoneType::IsZero;
            } else if is_even_digits(*n) {
                stone_type = StoneType::IsEvenDigits;
            } else {
                stone_type = StoneType::Multiply;
            }
            match stone_type {
                StoneType::IsZero => {
                    working_list.push(1)
                }
                StoneType::IsEvenDigits => {
                    let result = split_stone(*n);
                    working_list.push(result.0);
                    working_list.push(result.1);
                }
                StoneType::Multiply => {
                    working_list.push(*n * 2024)
                }
            }
        });
        stone_list = working_list.clone();
        working_list = vec![];
    });
    Some(stone_list.len())
}

struct Node {
    data: Option<i64>,
    next: Option<Rc<RefCell<Node>>>,
}
impl Node {
    fn add_next(&mut self, data: i64) -> Option<> {
        let next_node = Node {
            data: Some(data),
            next: None,
        };
        self.next = Some(Rc::new(RefCell::new(next_node)));
        self.next.clone()
    }
}
pub fn part_two(input: &str) -> Option<usize> {
    
    let list: Vec<_> = input.lines().nth(0)?.split(" ").collect();
    let base_node = Node {
        data: None,
        next: None
    };
    let mut current_node = base_node;
    list.iter().for_each(|n_str|{
        let n = n_str.parse::<i64>().unwrap();
        current_node.data = Some(n);
        let  next_node = Node {
            data: None,
            next: None,
        };
        let next_node_rc = Rc::new(RefCell::new(next_node));
        current_node.next = Some(next_node_rc);
        current_node = next_node;
    });
    None
/*    let stones: Vec<i64> = list.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    let mut stone_list: Vec<i64> = stones.clone();
    let mut working_list: Vec<i64> = vec![];
    (0..75).for_each(|i| {

        stone_list.iter().for_each(|n| {
            let stone_type: StoneType;
            if *n == 0 {
                stone_type = StoneType::IsZero;
            } else if is_even_digits(*n) {
                stone_type = StoneType::IsEvenDigits;
            } else {
                stone_type = StoneType::Multiply;
            }
            match stone_type {
                StoneType::IsZero => {
                    working_list.push(1)
                }
                StoneType::IsEvenDigits => {
                    let result = split_stone(*n);
                    working_list.push(result.0);
                    working_list.push(result.1);
                }
                StoneType::Multiply => {
                    working_list.push(*n * 2024)
                }
            }
        });
        if stone_list == working_list {
            println!("equal");
        }
        stone_list = working_list.clone();
        working_list = vec![];
    });
    Some(stone_list.len())
    
 */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
