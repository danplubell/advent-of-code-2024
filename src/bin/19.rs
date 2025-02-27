advent_of_code::solution!(19);
pub fn part_one(input: &str) -> Option<u32> {
    let words = input.lines().next().unwrap();
    let split_words = words.split(", ").collect::<Vec<_>>();
    let mut total = 0;
//    let mut dictionary: HashMap<char, Vec<&str>> = HashMap::new();
    println!("dictionary {:?}", split_words);
    /*
    for word in split_words {
        let c = word.chars().next().unwrap();
        let list = dictionary.entry(c).or_default();
        list.push(word);
    }
    
     */
//    println!("dictionary {:?}", dictionary);
    
    input.lines().skip(2).for_each(|w| {
        let result = tokenize_with_trie(w, &split_words);
        println!("{:?} {:?}", w, result);
        if !result.is_empty() {
            total += 1;
        }
    });

    Some(total)
}

fn validate_design(design: &str, dictionary: &HashMap<char, Vec<&str>>) -> bool {
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
            let slice = match design.split_at_checked(idx + s.len()){
                None=> {idx += 1; break},
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
    #[test]
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
}

use std::collections::HashMap;

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
