use std::cell::RefCell;
use std::rc::Rc;

advent_of_code::solution!(11);

enum StoneType {
    IsZero,
    IsEvenDigits,
    Multiply,
}
fn is_even_digits(n: i64) -> bool {
    n.abs().to_string().len() % 2 == 0
}
fn split_stone(n: i64) -> (i64, i64) {
    let abs_n = n.abs();
    let n_str = abs_n.to_string();
    let s = n_str.split_at(n_str.len() / 2);
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
                StoneType::IsZero => working_list.push(1),
                StoneType::IsEvenDigits => {
                    let result = split_stone(*n);
                    working_list.push(result.0);
                    working_list.push(result.1);
                }
                StoneType::Multiply => working_list.push(*n * 2024),
            }
        });
        stone_list = working_list.clone();
        working_list = vec![];
    });
    Some(stone_list.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let list: Vec<_> = input.lines().nth(0)?.split(" ").collect();
    let mut stones: Vec<i64> = list.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    let mut work_list: &mut std::vec::Vec<i64> = &mut vec![];
    let mut ref_list: &mut Vec<i64>;
    let mut even_list: std::vec::Vec<i64> = vec![];
    let mut odd_list: Vec<i64> = stones.clone();
    let mut total: usize = 0;
    for i in 0..75 {
        println!("iteration: {}", i);
        if i % 2_usize == 0 {
            even_list = vec![];
            work_list = &mut even_list;
            ref_list = &mut odd_list;
        } else {
            odd_list = vec![];
            work_list = &mut odd_list;
            ref_list = &mut even_list;
        }
        for j in 0..ref_list.len() {
            let stone = ref_list[j];
            let stone_type: StoneType;
            if stone == 0 {
                stone_type = StoneType::IsZero;
            } else if is_even_digits(stone) {
                stone_type = StoneType::IsEvenDigits;
            } else {
                stone_type = StoneType::Multiply;
            }
            match stone_type {
                StoneType::IsZero => {
                    work_list.push(1);
                }
                StoneType::IsEvenDigits => {
                    let result = split_stone(stone);
                    work_list.push(result.0);
                    work_list.push(result.1);
                }
                StoneType::Multiply => work_list.push(stone * 2024),
            }
            total = work_list.len();
        }
    }
    Some(total)
}
/*
pub fn part_two2(input: &str) -> Option<usize> {
    let list: Vec<_> = input.lines().nth(0)?.split(" ").collect();
    let mut linked_list = List::new();
    list.iter().for_each(|n_str| {
        let n = n_str.parse::<i64>().unwrap();
        linked_list.append(n);
    });

    (0..25).for_each(|n| {
        let mut current = linked_list.head.clone();
        while let Some(node) = current {
            let stone_type: StoneType;
            let mut node_ref = &mut node.as_ref().borrow_mut();
            let n = node_ref.get_elem();
            if *n == 0 {
                stone_type = StoneType::IsZero;
            } else if is_even_digits(*n) {
                stone_type = StoneType::IsEvenDigits;
            } else {
                stone_type = StoneType::Multiply;
            }
            match stone_type {
                StoneType::IsZero => {
                   node_ref.set_elem(1);
                    current = node_ref.next.clone(); // Move to the next node

                }
                StoneType::IsEvenDigits => {
                    let result = split_stone(*n);
                    node_ref.set_elem(result.0);

                    let mut new_node = Rc::new(RefCell::new(Node::new(result.1)));
                    new_node.as_ref().borrow_mut().set_next(node_ref.get_next());
                    node_ref.set_next(Some(new_node));
                    let n_node = node_ref.get_next(); // Move to the next node
                    match n_node {
                        Some(l) => {
                            let nl = l.as_ref().borrow_mut().get_next();
                            match nl {
                                Some(next_l) => {
                                    current = next_l.as_ref().borrow_mut().get_next();
                                }
                                None => {
                                    current = None
                                }
                            }
                        }
                        None=> {
                            current = None
                        }
                    }
                }
                StoneType::Multiply => {
                    let x = n * 2024;
                    node_ref.set_elem(x);
                    current = node_ref.next.clone(); // Move to the next node

                }
            }
            println!("while: {:?}", linked_list);
        }

    });

    //    linked_list.print_list();
    println!("list: {:?}", linked_list);
    let count = linked_list.count();
    Some(count)
}

 */

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
        assert_eq!(result, Some(55312));
    }
}
