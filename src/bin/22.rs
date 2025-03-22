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
#[derive(Debug, Clone, PartialEq, Eq)]
struct Buyer {
    numbers: Vec<Number>, // list of secret numbers
    seqs: Vec<(Vec<Change>, Price)>, // all the sequences that are associated with a price
    map: HashMap<(Change,Change,Change,Change), Price>, // map used to lookup the first instance of a sequence
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
        for i in 0..2000 {
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
        // build a list of all the sequences for a price
        // a sequence is the 4 previous numbers
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
        // unwrap the options
        let seqs: Vec<(Vec<Change>, Price)> = seqs
            .into_iter()
            .flatten()
            .map(|(vec, num)| (vec.into_iter().flatten().collect(), num))
            .collect();
        let mut map = HashMap::new();
        // We want a map so that we can look up the first price for a sequence
        seqs.iter().for_each(|(vec, price)| {
            let key = (vec[0], vec[1], vec[2], vec[3]);
            let e = map.get(&key);
            if e.is_none() {
                map.insert(key, *price);
            }
        });
        Self { numbers, seqs, map }
    }
    // get the first price in the list of sequences of the buyer
    fn get_first_price_for_sequence(&self, seq: (Change, Change, Change, Change)) -> Option<&Price> {
        self.map.get(&seq)
    }
}
pub fn part_two(input: &str) -> Option<i64> {
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
    let mut total = 0;
    let mut total_seq: (Change, Change, Change, Change) = (0,0,0,0);
    let mut visited = HashSet::new();
    // go through all the sequences for a key
    println!("seq count: {} {:?}", seqs.len(), seqs.keys() );
    println!("buyers count: {}", buyers.len() );
    for (key,value) in seqs.into_iter().rev() {
        println!("key: {} {}", key, value.len());
        for seq in value {
            let mut total_for_seq = 0;
            if visited.contains(&seq) {
                continue;
            }
            for buyer in &mut buyers {
                let p = buyer.get_first_price_for_sequence(seq);
                if let Some(p) = p {
                    total_for_seq += p;
                }
            }
            if total_for_seq > total {
                total = total_for_seq;
                total_seq= seq;
            }
            visited.insert(seq);
        }
    }
    println!("Part two: {} {:?}", total, total_seq);
    Some(total)
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
