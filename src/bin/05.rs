advent_of_code::solution!(5);

use std::collections::HashMap;

fn is_rule(line: &str) -> bool{
    line.chars().any(|c| c == '|')

}
fn is_update(line: &str) -> bool {
    line.chars().any(|c| c == ',')
}
fn are_rules_satisfied(page: &str, updated_pages: &Vec<&str>, rules: &Vec<&str>)-> bool {
    let page_idx = updated_pages.iter().position(|p| *p == page).unwrap();
    
    for r in rules.iter() {
        let r_opt = updated_pages.iter().position(|p| *p == *r);
        if let Some(r_idx) = r_opt {
            if page_idx > r_idx {
                return false;
            }
        }
    }
    true
}
fn is_update_ok(rules: &HashMap<&str, Vec<&str>>, updated_pages: &Vec<&str>) -> bool{
    //for each rule check to see if it is satisfied
    for page in updated_pages.iter() {
        let rules_for_page_opt = rules.get(page);
        if let Some(rules_for_page) = rules_for_page_opt {
            if !are_rules_satisfied(page, updated_pages, rules_for_page) {
                return false
            }
        }
    }
    true
}
fn find_middle(updated_pages: &Vec<&str>)->u32 {
    let len = updated_pages.len() as f64;
    let idx = (len / 2_f64) as usize;
    let page = updated_pages.get(idx).unwrap();
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
            let updated_pages: Vec<_> = l.split(",").collect();
            if is_update_ok(&rules, &updated_pages)  {
                let middle: u32 = find_middle(&updated_pages);
                total += middle;
            }
        }
    });
    Some(total)
}


fn fix_pages<'a>(rules: &HashMap<&str, Vec<&str>>, updated_pages: &Vec<&'a str>) -> Vec<&'a str> {
    let mut working_pages = updated_pages.clone();
    'fix: loop {
        //for each rule check to see if it is satisfied
        for idx in 0..working_pages.len() {
            let page = working_pages[idx];
            let rules_for_page_opt = rules.get(page);
            if let Some(rules_for_page) = rules_for_page_opt {
                let mut cur_working_pages = working_pages.clone();
                let page_idx = cur_working_pages.iter().position(|p| *p == page).unwrap();
                for r in rules_for_page.iter() {
                    let r_opt = cur_working_pages.iter().position(|p| *p == *r);
                    if let Some(r_idx) = r_opt {
                        if page_idx > r_idx {
                            let removed = cur_working_pages.remove(r_idx);
                            let new_ids = cur_working_pages.iter().position(|p| *p == page).unwrap();
                            cur_working_pages.insert(new_ids + 1, removed);
                            if !is_update_ok(&rules, &cur_working_pages) {
                                working_pages = cur_working_pages.clone();
                                continue 'fix;
                            } else {
                               working_pages = cur_working_pages.clone();
                                break 'fix;
                            }
                        }
                    }
                }
            }
        }
    }
    working_pages
}
pub fn part_two(input: &str) -> Option<u32> {
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
            let updated_pages: Vec<_> = l.split(",").collect();
            if !is_update_ok(&rules, &updated_pages)  {
                let corrected_pages = fix_pages(&rules, &updated_pages);
                let is_fixed = is_update_ok(&rules, &corrected_pages);
                let middle: u32 = find_middle(&corrected_pages);
                total += middle;
            }
        }
    });
    Some(total)
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
