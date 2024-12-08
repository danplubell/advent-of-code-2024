advent_of_code::solution!(4);

fn count_overlapping(text: &str, pattern: &str) -> usize {
    if pattern.is_empty() || pattern.len() > text.len() {
        return 0;
    }

    text.char_indices()
        .filter(|(i, _)| text[*i..].starts_with(pattern))
        .count()
}
fn find_xmas(chars: &[char]) -> u32 {
    let s: String = chars.iter().collect();
    let slice: &str = &s;
    let xmas_cnt = count_overlapping(slice, "XMAS");
    let samx_cnt = count_overlapping(slice, "SAMX");
    (xmas_cnt + samx_cnt) as u32
}
fn diagonals_rtl(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;
    for start_r in 0..lines.len() {
        let mut buffer: Vec<char> = vec![];
        for r_idx in start_r..lines.len() {
            let row = lines.get(r_idx);
            match row {
                Some(v) => {
                    let col_idx = r_idx - start_r;
                    let c = v.chars().nth(col_idx);
                    match c {
                        Some(v) => buffer.push(v),
                        None => break,
                    }
                }
                None => break,
            }
        }
        total += find_xmas(&buffer);
    }
    for start_col in 1..lines.len() {
        let mut buffer: Vec<char> = vec![];
        for r_idx in 0..lines.len() {
            let row = lines.get(r_idx);
            match row {
                Some(v) => {
                    let col_idx = r_idx + start_col;
                    let c = v.chars().nth(col_idx);
                    match c {
                        Some(v) => buffer.push(v),
                        None => break,
                    }
                }
                None => break,
            }
        }
        total += find_xmas(&buffer);
    }

    total
}
fn diagonals_ltr(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut total = 0;
    let start_col = lines.len() - 1;
    for start_r in 0..lines.len() {
        let mut buffer: Vec<char> = vec![];
        for r_idx in start_r..lines.len() {
            let row = lines.get(r_idx);
            match row {
                Some(v) => {
                    let col_idx = start_col - (r_idx - start_r);
                    let c = v.chars().nth(col_idx);
                    match c {
                        Some(v) => buffer.push(v),
                        None => break,
                    }
                }
                None => break,
            }
        }
        total += find_xmas(&buffer);
    }
    for start_col in (1..lines.len()).rev() {
        let mut buffer: Vec<char> = vec![];
        for (r_idx, col_idx) in (0..start_col).rev().enumerate() {
            let row = lines.get(r_idx);
            match row {
                Some(v) => {
                    let c = v.chars().nth(col_idx);
                    match c {
                        Some(v) => buffer.push(v),
                        None => break,
                    }
                }
                None => break,
            }
        }
        total += find_xmas(&buffer);
    }

    total
}
fn rows(input: &str) -> u32 {
    let mut n = 0;
    input.lines().for_each(|l| {
        let c = l.matches("XMAS").count();
        if c > 0 {
            n += c;
        }
    });
    input.lines().for_each(|l| {
        let c = l.matches("SAMX").count();
        if c > 0 {
            n += c;
        }
    });
    n as u32
}
pub fn part_one(input: &str) -> Option<u32> {
    let row_count = rows(input);
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
    let d_rtl = diagonals_rtl(input);
    let d_ltr = diagonals_ltr(input);

    println!("rows: {row_count} cols: {r_n} rtl: {d_rtl} lrt: {d_ltr}");
    let n = row_count + r_n + d_rtl + d_ltr;

    Some(n)
}

fn valid_char(char: char) -> bool {
    char == 'S' || char == 'M'
}
pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let length = lines.len();
    let mut total = 0;
    for row_idx in 0..length {
        let row = lines.get(row_idx)?;
        let matches = row.match_indices("A");
        for (i, m) in matches {
            //check to the upper
            //check lower
            let idx = row_idx.checked_sub(1);
            if idx.is_none() {
                break;
            }
            let upper_row_opt = lines.get(idx.unwrap());
            if upper_row_opt.is_none() {
                break;
            }
            let idx = row_idx + 1;
            let upper_row = upper_row_opt.unwrap();
            let lower_row_opt = lines.get(idx);
            if lower_row_opt.is_none() {
                break;
            }
            let lower_row = lower_row_opt.unwrap();
            //upper left
            let idx = i.checked_sub(1);
            if idx.is_none() {
                continue;
            }
            let upper_left_opt = upper_row.chars().nth(idx.unwrap());
            if upper_left_opt.is_none() {
                continue;
            }
            let upper_left = upper_left_opt.unwrap();
            if !valid_char(upper_left) {
                continue;
            }
            let idx = i + 1;
            let upper_right_opt = upper_row.chars().nth(idx);
            if upper_right_opt.is_none() {
                continue;
            }
            let upper_right = upper_right_opt.unwrap();
            if !valid_char(upper_right) {
                continue;
            }
            let idx = i.checked_sub(1);
            if idx.is_none() {
                continue;
            }
            let lower_left_opt = lower_row.chars().nth(idx.unwrap());
            if lower_left_opt.is_none() {
                continue;
            }
            let lower_left = lower_left_opt.unwrap();
            if !valid_char(lower_left) {
                continue;
            }
            let idx = i + 1;
            let lower_right_opt = lower_row.chars().nth(idx);
            if lower_right_opt.is_none() {
                continue;
            }
            let lower_right = lower_right_opt.unwrap();
            if !valid_char(lower_right) {
                continue;
            }
            println!("{upper_left} {upper_right} {lower_left} {lower_right}");
            match (upper_left, upper_right, lower_left, lower_right) {
                ('M', 'S', 'M', 'S') => total += 1,
                ('M', 'M', 'S', 'S') => total += 1,
                ('S', 'S', 'M', 'M') => total += 1,
                ('S', 'M', 'S', 'M') => total += 1,
                _ => (),
            }
        }
    }
    Some(total)
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
        assert_eq!(result, Some(9));
    }
    #[test]
    fn test_find_xmas() {
        let chars = "SAMXefXMAS".chars().collect::<Vec<char>>();
        println!("{}", find_xmas(&chars));
    }
    #[test]
    fn test_id() {
        let vec = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let step = 2;
        let result: Vec<_> = vec.iter().step_by(step).cloned().collect();
        println!("{:?}", result)
    }
}
