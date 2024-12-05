advent_of_code::solution!(2);

fn isSafe(report:Vec<u32>) ->bool {
    for i in 1..report.len() {
        let diff = match report[i-1].checked_sub(report[i]) {
            Some(diff) => diff as i32,
            None => -(report[i].checked_sub(report[i-2]).unwrap() as i32)
        };
        if diff > 0 {
            
        }
        let increase = report[i-1] > report[i];
    }
    true
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut reports: std::vec::Vec<_> = vec![];
    for line in input.lines() {
        let report: Vec<_> = line.split_whitespace().into_iter().map(|v| {
            v.parse::<u32>().unwrap()
        }).collect();
        reports.push(report);
    }
    let mut t = 0;
    for report in reports {
        if isSafe(report) {
            t += 1;
        }
    }
    Some(t)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
