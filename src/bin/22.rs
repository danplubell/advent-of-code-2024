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
    let total:i64 = numbers.iter().sum();
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
    seqs: Vec<(Vec<i64>, i64)>,
}
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
                continue
            }
            secret_number.number = calc_next(secret_number.number);
            secret_number.price = secret_number.number % 10;

            if i > 0 {
                let n1 = numbers.get((i - 1) as usize).unwrap();
                secret_number.change = Some(secret_number.price - n1.price  );
            } else {
                secret_number.change = None;
            }
            numbers.push(secret_number);
        }
        println!("{:?}", numbers);
        //sequences and prices
        let seqs = numbers.split_at(0).1;
        let seqs:  Vec<Option<(Vec<Option<i64>>, i64)>>  = seqs.chunks(4).map(|c| {
            let seq = c.iter().map(|n| n.change).collect::<Vec<Option<i64>>>();
            if seq.len() == 4 {
                return Some((seq,c.last().unwrap().price));
            }
            None
        }).collect();
        let seqs: Vec<(Vec<i64>, i64)> = seqs.into_iter()
            .filter_map(|option| option)
            .map(|(vec, num)| (
                vec.into_iter().filter_map(|inner_option| inner_option).collect(),
                num
            ))
            .collect();
//        let seqs:  Vec<Option<(Vec<Option<i64>>, i64)>> = numbers.chunks(4).map(|c| {
//            let seq = c.iter().map(|n| n.change).collect::<Vec<Option<i64>>>();
//            if seq.len() == 4 {
//                return Some((seq,c.last().unwrap().price));
//            }
//            None
//        }).collect();
        
        println!("{:?}", seqs);
        Self {
            numbers,
            seqs
        }
    }
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut buyers: Vec<Buyer> = Vec::new();
    for l in input.lines() {
        let s = l.parse::<i64>().ok()?;
        buyers.push(Buyer::new(s));
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
