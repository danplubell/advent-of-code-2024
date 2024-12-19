advent_of_code::solution!(9);

fn check_for_gaps(buffer: &mut Vec<i64>)->bool {
    let mut gap_flag:bool = true;
    let mut copy = buffer.clone();
    copy.reverse();
    copy.iter().for_each(|n|{ gap_flag = *n > -1;});
    gap_flag
}
fn take_and_replace(vec: &mut Vec<i64>, needed: usize) -> Vec<i64> {
    println!("needed {}",needed);
    let mut count = 0;
    let mut j = vec.len()-1;
    let mut taken = vec![];
    while j > 0 && count <= needed {
        if vec[j] != -1 {
            taken.push(vec[j]);
            count += 1;
            vec[j] = -1;
        }
        j -= 1;
    }
    taken
}
pub fn part_one(input: &str)  -> Option<i64> {
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

        let mut i:usize = 0;
        let mut j: usize = buffer.len() -1;
        while i < buffer.len() && i < j {
            if buffer[i] == -1 {
                //println!("i: {i} {j}");
                while buffer[j] == -1 {
                    j-=1;
                };
                //println!("j: {i} {j} {} {}", buffer[i], buffer[j]);
                buffer[i] = buffer[j];
                buffer[j] = -1;
                
            }
            i+=1;
        }
    });
//    println!("buffer {:?}", buffer);
    let t: Vec<_> = buffer.iter().enumerate().map(|(i,n)|{
        if *n > -1_i64 {
            i as i64 *n
        } else {
            0
        }
    }).collect();
    let total = t.iter().fold(0_i64,|acc,x| acc + x);
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
