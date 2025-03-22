use std::collections::{BTreeMap, HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(22);
fn calc_next(s: i64) -> i64 {
    let mut new_s = s;
    let mut r = s * 64;
    new_s = (r ^ new_s) % 16777216;
    r = new_s / 32;
    new_s = (r ^ new_s) % 16777216;
    r = new_s * 2048;
    new_s = (r ^ new_s) % 16777216;
    new_s
}
pub fn part_one(input: &str) -> Option<i64> {
    let mut numbers = Vec::new();
    for l in input.lines() {
        let s = l.parse::<i64>().ok()?;
        let mut new_s: i64 = s;
        for _ in 0..2000 {
            new_s = calc_next(new_s);
        }
        numbers.push(new_s);
    }
    let total: i64 = numbers.iter().sum();
    Some(total)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct Number {
    number: i64,
    price: i64,
    change: Option<i64>,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Buyer {
    numbers: Vec<Number>,
    seqs: Vec<(Vec<Change>, Price)>,
}
type Price = i64;
type Change = i64;
impl Buyer {
    fn new(first_number: i64) -> Buyer {
        let mut secret_number = Number {
            number: first_number,
            price: first_number % 10,
            change: None,
        };
        let mut numbers = Vec::new();
        //get sequence of numbers
        for i in 0..10 {
            if i == 0 {
                numbers.push(secret_number);
                continue;
            }
            secret_number.number = calc_next(secret_number.number);
            secret_number.price = secret_number.number % 10;

            if i > 0 {
                let n1 = numbers.get((i - 1) as usize).unwrap();
                secret_number.change = Some(secret_number.price - n1.price);
            } else {
                secret_number.change = None;
            }
            numbers.push(secret_number);
        }
        //sequences and prices
        let seqs = numbers.split_at(1).1;
        let seqs: Vec<Option<(Vec<Option<Change>>, Price)>> = seqs
            .iter()
            .enumerate()
            .skip(4)
            .map(|(idx, number)| {
                let prev_4 = seqs.get((idx - 4)..idx).unwrap();
                
                let seq = prev_4
                    .iter()
                    .map(|n| n.change)
                    .collect::<Vec<Option<Change>>>();
                if seq.len() == 4 {
                    return Some((seq, prev_4.last().unwrap().price));
                }
                None
            })
            .collect();
        // wrap the options
        let seqs: Vec<(Vec<Change>, Price)> = seqs
            .into_iter()
            .flatten()
            .map(|(vec, num)| (vec.into_iter().flatten().collect(), num))
            .collect();
        // sort numbers in reverse order by price
        let seqs = seqs
            .into_iter()
            .sorted_by(|a, b| Ord::cmp( &b.1,&a.1))
            .collect();
        
       // println!("{:?}", seqs);
        Self { numbers, seqs }
    }
    fn get_price_for_sequence(&self, seq: &[Change]) -> Price {
        
    }
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut buyers: Vec<Buyer> = Vec::new();
    for l in input.lines() {
        let s = l.parse::<i64>().ok();
        if let Some(sn)  = s{
            buyers.push(Buyer::new(s.unwrap()));
        }
    }
    // BTreeMap keeps them sorted
    let mut seqs: BTreeMap<Price, HashSet<(Change, Change, Change, Change)>> = BTreeMap::new();
    buyers.iter().for_each(|buyer| {
        buyer.seqs.iter().for_each(|(vec, price)| {
            let s = (vec[0], vec[1], vec[2], vec[3]);
            seqs.entry(*price).or_default().insert(s);
        })
    });
    for (key,value) in seqs {
        
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_calc_next() {
        let n = calc_next(123);
        println!("{}", n);
    }
}
