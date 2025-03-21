advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut buffer: Vec<i64> = Vec::new();

    input.lines().for_each(|l| {
        // get pairs of values
        let v: Vec<_> = l
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| match chunk.len() == 2 {
                true => (chunk[0], chunk[1]),
                false => (chunk[0], '0'),
            })
            .collect::<Vec<_>>();
        for (i, (c1, c2)) in v.iter().enumerate() {
            // for each pair expand into the buffer
            let f_blocks = c1.to_digit(10).unwrap_or(0);
            let s_blocks = c2.to_digit(10).unwrap_or(0);
            (0..f_blocks).for_each(|_e| buffer.push(i as i64));

            // collect fragment
            (0..s_blocks).for_each(|_s| buffer.push(-1));
        }
        //        println!("{:?}", buffer);
        // defragment
        let mut i: usize = 0;
        let mut j: usize = buffer.len() - 1;
        while i < buffer.len() && i < j {
            if buffer[i] == -1 {
                //println!("i: {i} {j}");
                while buffer[j] == -1 {
                    j -= 1;
                }
                //println!("j: {i} {j} {} {}", buffer[i], buffer[j]);
                buffer[i] = buffer[j];
                buffer[j] = -1;
            }
            i += 1;
        }
    });
    //    println!("buffer {:?}", buffer);
    let t: Vec<_> = buffer
        .iter()
        .enumerate()
        .map(|(i, n)| if *n > -1_i64 { i as i64 * n } else { 0 })
        .collect();
    let total = t.iter().fold(0_i64, |acc, x| acc + x);
    Some(total)
}

fn group_identical_with_index(data: &Vec<i64>) -> Vec<(usize, Vec<i64>)> {
    let mut result: Vec<(usize, Vec<i64>)> = Vec::new();
    if data.is_empty() {
        return result;
    }

    let mut current_group = vec![data[0]];
    let mut current_index = 0;
    for (i, &value) in data.iter().enumerate().skip(1) {
        if value == current_group[0] {
            current_group.push(value);
        } else {
            result.push((current_index, current_group));
            current_group = vec![value];
            current_index = i;
        }
    }
    result.push((current_index, current_group));

    result
}
fn build_buffer(input: &str) -> Vec<i64> {
    let mut buffer: Vec<i64> = Vec::new();

    input.lines().take(1).for_each(|l| {
        //expand into the buffer
        let v: Vec<_> = l
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| match chunk.len() == 2 {
                true => (chunk[0], chunk[1]),
                false => (chunk[0], '0'),
            })
            .collect::<Vec<_>>();
        for (i, (c1, c2)) in v.iter().enumerate() {
            // for each pair expand into the buffer
            let f_blocks = c1.to_digit(10).unwrap_or(0);
            let s_blocks = c2.to_digit(10).unwrap_or(0);
            (0..f_blocks).for_each(|_e| buffer.push(i as i64));

            // collect fragment
            (0..s_blocks).for_each(|_s| buffer.push(-1));
        }
    });
    buffer
}
fn find_file(buffer: &Vec<i64>, block_size: i32) -> Option<(usize, Vec<i64>)> {
    let mut tmp_buffer = buffer.clone();
    tmp_buffer.reverse();
    let mut groups = group_identical_with_index(&tmp_buffer)
        .into_iter()
        .filter(|(i, g)| g[0] != -1);
    let found = groups.find(|(i, g)| g.len() <= block_size as usize);
    found
}
pub fn part_two(input: &str) -> Option<i64> {
    let mut buffer: Vec<i64> = build_buffer(input);
    let groups = group_identical_with_index(&buffer);
    let mut files: Vec<_> = groups.iter().filter(|(i, g)| g[0] != -1).collect();
    files.reverse();
    files.iter().for_each(|(i, f)| {
        // go through each file and try to move it
        let new_groups = group_identical_with_index(&buffer);
        let gap_list: Vec<_> = new_groups.iter().filter(|(i, g)| g[0] == -1).collect();
        let result = gap_list.iter().find(|(i, g)| g.len() >= f.len());
        if let Some((j, v)) = result {
            if i > j {
                // replace the gap with the block
                replace_block(&mut buffer, *j, f);
                replace_block_repeat(&mut buffer, *i, f.len(), -1);
            }
        }
    });
    let t: Vec<_> = buffer
        .iter()
        .enumerate()
        .map(|(i, n)| if *n > -1_i64 { i as i64 * n } else { 0 })
        .collect();
    let total = t.iter().fold(0_i64, |acc, x| acc + x);
    Some(total)

}

fn replace_block_repeat(buffer: &mut [i64], p1: usize, block_size: usize, replace_with: i64) {
    let start = p1;
    let end = p1 + block_size;
    (start..end).for_each(|n| buffer[n] = replace_with);
}

fn replace_block(buffer: &mut [i64], block_size: usize, replace_with: &Vec<i64>) {
    replace_with.iter().enumerate().for_each(|(i, n)| {
        buffer[block_size + i] = *n;
    })
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
        assert_eq!(result, Some(2858));
    }
    #[test]
    fn test_find_file() {
        let test_vec: Vec<i64> = vec![0, 0, 0, -1, -1, -1, 2, 2, -1, -1, 4, 4, 4, 4, -1, -1];
        let result = find_file(&test_vec, 4);
        println!("{:?}", result);
    }

    #[test]
    fn test_replace_block() {
        let mut test_vec: Vec<i64> = vec![0, 0, 0, -1, -1, -1, 2, 2, -1, -1, 4, 4, 4, 4, -1, -1];
        let replacement = vec![3, 3];
        replace_block(&mut test_vec, 6, &replacement);
        println!("{:?}", test_vec);
    }
    #[test]
    fn test_replace_block_repeat() {
        let mut test_vec: Vec<i64> = vec![0, 0, 0, -1, -1, -1, 2, 2, -1, -1, 4, 4, 4, 4, -1, -1];
        replace_block_repeat(&mut test_vec, 6, 2, -1);
        println!("{:?}", test_vec);
    }
}
