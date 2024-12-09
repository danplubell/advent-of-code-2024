advent_of_code::solution!(5);

use std::collections::HashMap;

fn is_rule(line: &str) -> bool{
    line.chars().any(|c| c == '|')

}
fn is_update(line: &str) -> bool {
    line.chars().any(|c| c == ',')
}
fn are_rules_satisfied(page: &str, update_pages: &Vec<&str>, rules: &Vec<&str>)-> bool {
    let page_idx = update_pages.iter().position(|p| *p == page).unwrap();
    
    for r in rules.iter() {
        let r_opt = update_pages.iter().position(|p| *p == *r);
        if let Some(r_idx) = r_opt {
            if page_idx > r_idx {
                return false;
            }
        }
    }
    true
}
fn is_update_ok(rules: &HashMap<&str, Vec<&str>>, update_pages: &Vec<&str>) -> bool{
    //for each rule check to see if it is satisfied
    for page in update_pages.iter() {
        let rules_for_page_opt = rules.get(page);
        if let Some(rules_for_page) = rules_for_page_opt {
            if !are_rules_satisfied(page, update_pages, rules_for_page) {
                return false
            }
        }
    }
    true
}
fn find_middle(update_pages: &Vec<&str>)->u32 {
    let len = update_pages.len() as f64;
    let idx = (len / 2_f64) as usize;
    let page = update_pages.get(idx).unwrap();
    page.parse::<u32>().unwrap()
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut rules:HashMap<&str, Vec<&str>> = HashMap::new();
    let mut total = 0;
    input.lines().for_each(|l| {
        // is it a rule or updates
        if is_rule(l) {
            // add to hashMap
            // get key
            let rule_parts: Vec<_> = l.split("|").collect();
            let new_rule_key = rule_parts.get(0).unwrap();
            let new_rule_page = rule_parts.get(1).unwrap();
            match rules.contains_key(new_rule_key) {
                true => {
                    let rule_list = rules.get_mut(new_rule_key).unwrap();
                    rule_list.push(new_rule_page);
                    let new_rule_list = rule_list.clone();
                    rules.insert(new_rule_key, (*new_rule_list).to_owned());
                }
                false => {
                    let mut new_rule_list: Vec<&str> = vec![new_rule_key];
                    new_rule_list.push(new_rule_page);
                    rules.insert(new_rule_key, new_rule_list);
                }
            }
        }
        if is_update(l) {
            let update_pages: Vec<_> = l.split(",").collect();
            if is_update_ok(&rules, &update_pages)  {
                let middle: u32 = find_middle(&update_pages);
                total += middle;
            }
        }
    });
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
