advent_of_code::solution!(2);

enum Direction {
    Increase,
    Decrease,
}
fn is_safe(report:Vec<u32>) ->bool {
    let mut changes: Vec<i32> = vec![];
    for i in 0..(report.len()-1) {
        let diff = match report[i].checked_sub(report[i+1]) {
            Some(diff) => diff as i32,
            None => -(report[i+1].checked_sub(report[i]).unwrap() as i32)
        };
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
        let sign = diff >> 31 & 1;
        changes.push(sign);
    }
    changes.iter().all(|&sign| sign == 1) || changes.iter().all(|&sign| sign == 0)
}
fn is_safe_with_damper(report:Vec<u32>) ->bool {
    if is_safe(report.clone()) {
        return true;
    }
    
    for index in 0..report.len() {
        let mut curr_report = report.clone();
        curr_report.remove(index);
        if is_safe(curr_report) {
            return true;
        }
    }
    false
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
        if  is_safe(report) {
            t += 1;
        }
    }
    Some(t)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut reports: std::vec::Vec<_> = vec![];
    for line in input.lines() {
        let report: Vec<_> = line.split_whitespace().into_iter().map(|v| {
            v.parse::<u32>().unwrap()
        }).collect();
        reports.push(report);
    }
    let mut t = 0;
    for report in reports {
        if !report.is_empty() && is_safe_with_damper(report) {
            t += 1;
        }
    }
    Some(t)
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
        assert_eq!(result, Some(1));
    }
}
