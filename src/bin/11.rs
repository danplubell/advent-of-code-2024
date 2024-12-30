use std::cell::RefCell;
use std::ops::DerefMut;
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
use std::fmt::Display;

/*
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Link<T>,
}

impl<T: Display> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
    }

    fn insert(&mut self, value: T, index: usize) {
        if index == 0 {
            let mut new_node = Box::new(Node::new(value));
            new_node.next = self.head.take();
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            let mut count = 0;
            while let Some(node) = current {
                if count == index - 1 {
                    let mut new_node = Box::new(Node::new(value));
                    new_node.next = node.next.take();
                    node.next = Some(new_node);
                    return;
                }
                current = &mut node.next;
                count += 1;
            }
            panic!("Index out of bounds");
        }
    }
    fn update(&mut self, index: usize, new_value: T) {
        if index == 0 {
            if let Some(node) = &mut self.head {
                node.value = new_value;
            } else {
                panic!("Index out of bounds");
            }
        } else {
            let mut current = &mut self.head;
            let mut count = 0;
            while let Some(node) = current {
                if count == index {
                    node.value = new_value;
                    return;
                }
                current = &mut node.next;
                count += 1;
            }
            panic!("Index out of bounds");
        }
    }
    fn count(&mut self) -> usize {
        let mut count:usize = 0;
        let mut current = &mut self.head;

        while let Some(node) = current {
            current = &mut node.next;
            count += 1;
        }
        count
    }
    fn print_list(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} ", node.value);
            current = &node.next;
        }
        println!();
    }
}

 */

#[derive(Debug, PartialEq, Clone)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, PartialEq, Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
impl<T> Node<T>{
    fn new(value:T) -> Self {
        Self {
            elem: value,
            next: None,
        }
    }
    fn set_elem(&mut self, elem: T) {
        self.elem = elem;
    }
    fn get_elem(&mut self) -> &T {
        &self.elem
    }
    fn get_next(&mut self) -> Link<T>{
        self.next.clone()
    }
    fn set_next(&mut self, new_next: Link<T>) {
        self.next = new_next;
    }
}

impl<T> List<T> {
    pub fn append(&mut self, elem: T) {
        let new_node = Rc::new(RefCell::new(Node::new(elem)));
        match self.head.as_ref() {
            None => {
                // If the list is empty, set the head to the new node
                self.head = Some(new_node);
            }
            Some(_) => {
                // Traverse to the end of the list and append the new node
                let mut current = self.head.clone();
                while let Some(node) = current {
                    let mut node_ref = node.borrow_mut(); // Borrow mutably
                    if node_ref.next.is_none() {
                        node_ref.next = Some(new_node); // Set the next node
                        break;
                    }
                    current = node_ref.next.clone(); // Move to the next node
                }
            }
        }
    }
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(RefCell::new(Node {
            elem: elem,
            next: self.head.clone(),
        })))}
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| {let node_ref = node.borrow(); node_ref.next.clone()}) }
    }
    pub fn head(&self) -> Option<T>
    where
        T: Clone
    {
        self.head.as_ref().map(|node| {
            let node_ref = node.borrow();
            node_ref.elem.clone()
        })
    }
    /*
    pub fn append(&mut self, elem: T) {
        let new_node = Rc::new(RefCell::new(Node {
            elem,
            next: None,
        }));

        match &self.head {
            None => self.head = Some(new_node),
            Some(head) => {
                let mut current = Rc::clone(head);
                loop {
                    let mut current_ref = current.borrow_mut();
                    if current_ref.next.is_none() {
                        current_ref.next = Some(new_node);
                        break;
                    }
                    let next = Rc::clone(current_ref.next.as_ref().unwrap());
                    drop(current_ref);
                    current = next;
                }
            }
        }
    }
    
     */
}

pub fn part_two(input: &str) -> Option<usize> {
    let list: Vec<_> = input.lines().nth(0)?.split(" ").collect();
    let mut linked_list = List::new();
    list.iter().for_each(|n_str|{
        let n = n_str.parse::<i64>().unwrap();
        linked_list.append(n);
    });
    
    (0..1).for_each(|n|{
        let mut current = &mut linked_list.head;
        while let Some(node) = current {
            println!("node: {:?}", node);
            let stone_type: StoneType;
            let n = node.as_ref().borrow().get_elem();
            if *n == 0 {
                stone_type = StoneType::IsZero;
            } else if is_even_digits(*n) {
                stone_type = StoneType::IsEvenDigits;
            } else {
                stone_type = StoneType::Multiply;
            }
            match stone_type {
                StoneType::IsZero => {
                   node.as_ref().borrow().set_elem(1);
                }
                StoneType::IsEvenDigits => {
                    let result = split_stone(*n);
                    node.as_ref().borrow_mut().set_elem(result.0);
                    let mut new_node = Rc::new(RefCell::new(Node::new(result.1)));
                    new_node.next = node.as_ref().borrow().next.clone();
                    node.as_ref().borrow().set_next(Some(new_node));
                }
                StoneType::Multiply => {
                    node.elem = n * 2024;
                }
            }
            current = &mut node.next;
            println!("current: {:?}", current);
        }
    });
//    linked_list.print_list();
    println!("list: {:?}", linked_list);
//    let count = linked_list.count();
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

    #[test]
    fn basics() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.print_list();
    }
}
