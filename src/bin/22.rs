advent_of_code::solution!(22);
fn calc_next(s:u64) -> u64 {
    let mut new_s = s;
    let mut r = s * 64;
    new_s = (r ^ new_s) % 16777216;
    r = new_s / 32;
    new_s = (r ^ new_s) % 16777216;
    r = new_s * 2048;
    new_s = (r ^ new_s) % 16777216;
    new_s
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut numbers = Vec::new();
    for l in input.lines() {
        let s = l.parse::<u64>().ok()?;
        let mut new_s:u64 = s;
        for _ in 0..2000 {
            new_s = calc_next(new_s);
        }
        numbers.push(new_s);
    }
    let total = numbers.iter().sum();
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
