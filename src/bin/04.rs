advent_of_code::solution!(4);
const XMAS: &[char] = &['X', 'M', 'A', 'S'];
const SAMX: &[char] = &['S', 'A', 'M', 'X'];

fn find_xmas(chars: &[char]) -> u32 {
    //    let chars = input.chars().collect::<Vec<char>>();
    let windows = chars.windows(4);
    let mut count = 0;
    for w in windows {
        if w == XMAS || w == SAMX {
            count += 1;
        }
    }
    count
}
fn diagonals1(flattened: Vec<&char>, stride: usize) -> u32 {
    //diagonals
    // convert to vector of char vectors
    // flatten
    let mut d_f = 0;
    for (i, f) in flattened.iter().enumerate() {
        let rest = flattened.get(i..).unwrap();
        let result: Vec<_> = rest.iter().step_by(stride + 1).cloned().collect();
        if result.len() < stride {
            continue;
        }
        let result_chars: Vec<char> = result.into_iter().cloned().collect();
        d_f += find_xmas(&result_chars);
    }
    d_f
}
fn diagonals(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut t = 0;
    for row_cycle in 0..lines.len() {
        println!("row_cycle: {row_cycle}");
        for iteration in 0..lines.len() {
           let mut col_index = iteration;
            let mut buffer: Vec<char> = Vec::new();
            for row in row_cycle..lines.len() {
                if col_index > 3 {
                    break;
                }
                let r = lines.get(row);
                match r {
                    Some(v) => {
                        let c = v.chars().nth(col_index);
                        match c {
                            Some(v) => buffer.push(v),
                            None => break
                        }
                    },
                    None => break
                }
                col_index += 1;
            }
           
            t += find_xmas(&buffer);
        }
    }
    t
    /*    for i in 0..lines.len() {
           for c in 0..lines.len() {
               let mut buffer: Vec<char> = vec![];
               for r in 0..lines.len() {
                   let row = lines.get(r);
                   match row {
                       Some(rw) => {
                           let v = rw.chars().nth(r + c);
                           match v {
                               Some(c) => {
                                   buffer.push(c)
                               },
                               None => break
                           }
                       },
                       None => break
                   }
               }
               println!("{:?}", buffer);
           }
       }

    */
}
pub fn part_one(input: &str) -> Option<u32> {
    // handle lines
    //    let chars = input.chars().collect::<Vec<_>>();
    let n = input
        .lines()
        .map(|l| {
            let chars = l.chars().collect::<Vec<char>>();
            find_xmas(&chars)
        })
        .reduce(|a, b| a + b)?;
    // now do rows
    let mut rows: Vec<Vec<char>> = Vec::new();
    let _: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars().enumerate().for_each(|(i, c)| {
                let row = rows.get_mut(i);
                match row {
                    Some(r) => {
                        r.push(c);
                    }
                    None => {
                        let new = vec![c];
                        rows.push(new);
                    }
                }
            })
        })
        .collect();
    let r_n = rows
        .iter()
        .map(|r| find_xmas(r))
        .reduce(|a, b| a + b)
        .unwrap();
    let stride = input.lines().next().unwrap().len();
    //    let char_vecs = input.lines().map(|line| {line.chars().collect::<Vec<char>>()}).collect::<Vec<_>>();
    //    let mut flattened = char_vecs.iter().flatten().collect::<Vec<_>>();
    let d_f = diagonals(input);

    //    flattened.reverse();
     let d_f_r = diagonals(input);

    let n = n + r_n + d_f + d_f_r;

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
    #[test]
    fn test_id() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let step = 2;
        let result: Vec<_> = vec.iter().step_by(step).cloned().collect();
        println!("{:?}", result)
    }
}
