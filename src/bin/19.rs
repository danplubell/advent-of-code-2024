advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let words = input.lines().next().unwrap();
    let word_list = words.split(", ").collect::<Vec<_>>();
    // Build the trie from word_list
    let mut root = TrieNode::new();
    let mut total: u32 = 0;
    for word in word_list {
        root.insert(word.as_bytes());
    }

    input.lines().skip(2).for_each(|w| {
        let r = can_construct(w, &root);
        if r {
            total += 1;
        }
    });

    Some(total)
}
pub fn part_one_hash(input: &str) -> Option<u32> {
    let words = input.lines().next().unwrap();
    let word_list = words.split(", ").collect::<Vec<_>>();
    let mut total = 0;
    let mut dictionary: HashMap<char, Vec<&str>> = HashMap::new();
    println!("dictionary {:?}", dictionary);

    for word in word_list {
        let c = word.chars().next().unwrap();
        let list = dictionary.entry(c).or_default();
        list.push(word);
    }
    
    input.lines().skip(2).for_each(|w| {
        if validate_design(w, &dictionary) {
            println!("{}", w);
            total += 1
        }
    });

    Some(total)
}
fn validate_design(w: &str, dictionary: &HashMap<char, Vec<&str>>) -> bool {
    let mut pos: usize = 0;
    let mut design_valid = false;
    while pos < w.len() {
        let c = match w.chars().nth(pos) {
            Some(c) => c,
            None => {
                return design_valid;
            }
        };
        let towels = match dictionary.get(&c) {
            Some(towels) => towels,
            None => return design_valid,
        };
        for towel in towels {
            let range = pos..pos + towel.len();
            let slice_opt = &w.get(range);
            let slice = match slice_opt {
                Some(slice) => slice,
                None=> return design_valid
            };
            if *slice == *towel {
                pos += towel.len() -1;
                design_valid = true;
                break;
            }
        };
        pos += 1;
    }
    design_valid
}
/*
pub fn part_one1(input: &str) -> Option<u32> {
    let words = input.lines().next().unwrap();
    let dictionary = words.split(", ").collect::<Vec<_>>();
    let mut total = 0;
    //    let mut dictionary: HashMap<char, Vec<&str>> = HashMap::new();
    //    println!("dictionary {:?}", dictionary);
    /*
    for word in split_words {
        let c = word.chars().next().unwrap();
        let list = dictionary.entry(c).or_default();
        list.push(word);
    }

     */
    //    println!("dictionary {:?}", dictionary);
    let mut trie = Trie::new();
    for word in dictionary {
        trie.insert(word);
    }
    println!("trie: {:?}", trie);
    input.lines().skip(2).for_each(|w| {
        let mut pos = 0;
        let mut valid: Option<usize> = None;
        loop {
            valid = validate_design2(w, &trie.root, pos);
            if valid.is_none() {
                if pos >= w.len() {
                    println!("w={:?}", w);
                    total += 1;
                }
                break;
            }
            pos = valid.unwrap();
            pos += 1;
        }
        if let Some(pos) = valid {
            total += 1;
        }
    });

    Some(total)
}


 */
/*
fn validate_design2(design: &str, node: &TrieNode, pos: usize) -> Option<usize> {
    let c = design.chars().nth(pos)?;
    let next_node = node.children.get(&c)?;
    let next_pos = pos + 1;
    let result = validate_design2(design, next_node, next_pos);
    if result.is_none() && next_node.is_word {
        return Some(next_pos);
    }
    result
}

 

fn validate_design1(design: &str, dictionary: &HashMap<char, Vec<&str>>) -> bool {
    let mut idx = 0;
    while let Some(curr_c) = design.chars().nth(idx) {
        let curr_list = match dictionary.get(&curr_c) {
            Some(list) => list,
            None => {
                idx += 1;
                continue;
            }
        };

        let mut found_match = false;
        for s in curr_list {
            let slice = match design.split_at_checked(idx + s.len()) {
                None => {
                    idx += 1;
                    break;
                }
                Some(s) => s,
            };
            if slice.0.contains(s) {
                idx += s.len();
                found_match = true;
                break;
            }
        }
        if !found_match {
            return false;
        }
    }
    true
}


 */
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
/*    #[test]
    fn test_part_one_2() {
        //        let dictionary = vec!["ice", "cream", "icecream", "man", "go", "mango"];
        let dictionary = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        // let s = "icecreamman";
        let s = "brwrr";
        let result = tokenize_with_trie(s, &dictionary);

        println!("All possible tokenizations:");
        for (i, tokenization) in result.iter().enumerate() {
            println!("{}: {}", i + 1, tokenization);
        }
    }
    
 */
}


fn can_construct(target: &str, root: &TrieNode) -> bool {
    let target_bytes = target.as_bytes();
    let n = target_bytes.len();


    // Dynamic programming array
    let mut dp = vec![false; n + 1];
    dp[0] = true; // Empty string can always be constructed

    for i in 0..n {
        // Skip if we can't reach this position
        if !dp[i] {
            continue;
        }

        // Try to find words starting at position i
        let mut current = root;
        for j in i..n {
            let byte = target_bytes[j];

            // If there's no path in the trie for this character, break
            if !current.children.contains_key(&byte) {
                break;
            }

            // Move to the next node in the trie
            current = &current.children[&byte];

            // If we've found a complete word, mark the position after it as reachable
            if current.is_end_of_word {
                dp[j + 1] = true;
            }
        }
    }

    dp[n] // Final answer
}

use std::collections::HashMap;

// Trie node structure
struct TrieNode {
    is_end_of_word: bool,
    word: Option<String>,
    children: HashMap<u8, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_end_of_word: false,
            word: None,
            children: HashMap::new(),
        }
    }

    // Insert a word into the trie
    fn insert(&mut self, word: &str) {
        let mut current = self;

        for &byte in word.as_bytes() {
            current = current.children.entry(byte).or_insert_with(TrieNode::new);
        }

        current.is_end_of_word = true;
        current.word = Some(word.to_string());
    }
}

fn find_all_constructions(target: &str, word_list: &[&str]) -> Vec<Vec<String>> {
    let target_bytes = target.as_bytes();
    let n = target_bytes.len();

    // Build the trie from word_list
    let mut root = TrieNode::new();
    for &word in word_list {
        root.insert(word);
    }

    // For each position, store all valid words that end at that position
    let mut dp: Vec<Vec<String>> = vec![vec![]; n + 1];
    dp[0] = vec![String::new()]; // Empty string as the base case

    for i in 0..n {
        // Skip positions that can't be reached
        if dp[i].is_empty() {
            continue;
        }

        let prev_words = dp[i].clone(); // Clone to avoid borrowing conflicts

        // Try to find words starting at position i
        let mut current = &root;
        let mut j = i;

        while j < n {
            let byte = target_bytes[j];

            // If there's no path in the trie for this character, break
            if !current.children.contains_key(&byte) {
                break;
            }

            // Move to the next node in the trie
            current = &current.children[&byte];
            j += 1;

            // If we've found a complete word, update dp[j]
            if current.is_end_of_word {
                if let Some(ref word) = current.word {
                    for prev_construction in &prev_words {
                        let new_construction = if prev_construction.is_empty() {
                            word.clone()
                        } else {
                            format!("{} {}", prev_construction, word)
                        };
                        dp[j].push(new_construction);
                    }
                }
            }
        }
    }

    // Convert space-separated strings to vectors of words
    dp[n].iter()
        .map(|s| s.split_whitespace().map(String::from).collect())
        .collect()
}

/*
#[derive(Clone, PartialEq, Eq, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}


impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.is_word = true;
    }
}

fn find_words(
    s: &str,
    pos: usize,
    path: &mut Vec<String>,
    result: &mut Vec<String>,
    trie: &TrieNode,
) {
    if pos == s.len() {
        result.push(path.join(" "));
        return;
    }

    let mut node = trie;
    let chars: Vec<char> = s.chars().collect();

    for i in pos..s.len() {
        if let Some(next_node) = node.children.get(&chars[i]) {
            node = next_node;
            if node.is_word {
                path.push(s[pos..=i].to_string());
                find_words(s, i + 1, path, result, trie);
                path.pop();
            }
        } else {
            break;
        }
    }
}

fn tokenize_with_trie(s: &str, dictionary: &[&str]) -> Vec<String> {
    // Build trie
    let mut trie = Trie::new();
    for &word in dictionary {
        trie.insert(word);
    }

    let mut result = Vec::new();
    let mut path = Vec::new();
    find_words(s, 0, &mut path, &mut result, &trie.root);
    result
}


 */