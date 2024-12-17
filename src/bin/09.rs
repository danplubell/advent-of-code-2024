advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    //expand input into blocks and files
    let buffer = Vec::new()
    input.lines().for_each(|l|{
        // get pairs of values
       for (i,(c1,c2)) in l.chars().zip(l.chars().take(1)).enumerate() {
           // for each pair expand into the buffer
           let f_blocks = c1.to_digit(10).unwrap_or(0);
           let s_blocks = c2.to_digit(10).unwrap_or(0);
           for e in 0..f_blocks {
               
           }
       }
    });
    //defragment
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
