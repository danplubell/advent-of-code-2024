advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let pairs: Vec<&str> = input.split("\n").collect();
    let number_pairs: Vec<_> = pairs
        .iter()
        .map(|s| s.split("   ").collect::<Vec<_>>())
        .collect();
    println!("{:?}", number_pairs);
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
