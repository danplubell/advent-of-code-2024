advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    //expand input into blocks and files
    let mut buffer: Vec<i64> = Vec::new();
    input.lines().for_each(|l| {
        // get pairs of values
        let v: Vec<_> = l
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                match chunk.len() == 2 {
                    true => (chunk[0], chunk[1]),
                    false => (chunk[0], 0 as char)
                }
                })
            .collect::<Vec<_>>();
        for (i, (c1, c2)) in v.iter().enumerate() {
            println!("{i}, {},{}", c1, c2);
            // for each pair expand into the buffer
            let f_blocks = c1.to_digit(10).unwrap_or(0);
            let s_blocks = c2.to_digit(10).unwrap_or(0);
            (0..f_blocks).for_each(|_e| buffer.push(i as i64));
            (0..s_blocks).for_each(|_s| buffer.push(-1));
        }
    });
    println!("{:?}", buffer);
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

/*
fn pair_elements(s: &str) -> Vec<(char, char)> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let s = "123456";
    let result = pair_elements(s);

    for (a, b) in result {
        println!("({}, {})", a, b);
    }
}
 */
