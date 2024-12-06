advent_of_code::solution!(4);
const XMAS: &[char] = &['X', 'M', 'A', 'S'];
const SAMX: &[char] = &['S', 'A', 'M', 'X'];

fn find_xmas(chars: &[char]) -> u32 {
//    let chars = input.chars().collect::<Vec<char>>();
    let windows = chars.windows(4);
    let mut count = 0;
    for w in windows {
        if w==XMAS || w==SAMX {
            count += 1;
        }
    }
    count
}
pub fn part_one(input: &str) -> Option<u32> {
    // handle lines
//    let chars = input.chars().collect::<Vec<_>>();
    let n = input.lines().map(|l|{
        let chars = l.chars().collect::<Vec<char>>();
        find_xmas(&chars)}).reduce(|a, b| a + b)?;
    // now do rows
    let mut rows: Vec<Vec<char>> = Vec::new();
    let _: Vec<_> = input.lines().map(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            let row = rows.get_mut(i);
            match row {
                Some(r) => { r.push(c); }
                None => {
                    let new = vec![c];
                    rows.push(new);
                }
            }
        })
    }).collect();
    let r_n = rows.iter().map(|r|find_xmas(r)).reduce(|a, b| a + b).unwrap();
    println!(" rows: {:?}", rows);
    let n = n + r_n;
    
    //diagonals
    Some(n)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_find_xmas() {
        let chars = "SAMXefXMAS".chars().collect::<Vec<char>>();
        println!("{}", find_xmas(&chars));
    }
}
