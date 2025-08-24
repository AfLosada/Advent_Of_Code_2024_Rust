use std::collections::HashMap;

use crate::util::util;

fn extract_rules_and_rows_from_input(input: &str) -> Page {
    let lines = util::read_input("day5", input);
    lines.lines().fold(Page::new(), |mut current_page, line| {
        let is_rule = line.contains('|');
        let is_row = line.contains(',');
        if is_rule {
            current_page.raw_rules.push(line.to_string());
        }
        if is_row {
            current_page.raw_lines.push(line.to_string());
        }
        return current_page;
    })
}

struct Page {
    raw_rules: Vec<String>,
    raw_lines: Vec<String>,
}

impl Page {
    fn new() -> Page {
        return Page {
            raw_lines: vec![],
            raw_rules: vec![],
        };
    }
}

struct RuleTrie {
    rule_trie: HashMap<char, TrieNode>,
}

impl RuleTrie {
    fn new(page: Page) -> RuleTrie {
        let rule_trie =
            page.raw_rules
                .iter()
                .fold(HashMap::new(), |mut map: HashMap<char, TrieNode>, rule| {
                    map.insert(rule.chars().next().unwrap(), TrieNode::new(rule));
                    map
                });
        RuleTrie { rule_trie }
    }
    fn find_rules(letter: char) {}
}

#[derive(Clone)]
struct TrieNode {
    nodes: HashMap<char, TrieNode>,
    value: Option<String>,
}

impl TrieNode {
    fn new(rule: &str) -> TrieNode {
        cyclical_tree_build(rule, rule)
    }
}

fn cyclical_tree_build(original_rule: &str, remaining_letters: &str) -> TrieNode {
    let mut nodes: HashMap<char, TrieNode> = HashMap::new();
    let mut letters = remaining_letters.chars();
    let first_letter = letters.next();
    let remaining_letters = &remaining_letters[0..];

    match first_letter {
        Some(first_letter) => {
            let node = nodes.get(&first_letter).cloned();
            match node {
                Some(n) => n,
                None => {
                    nodes.insert(
                        first_letter.clone(),
                        cyclical_tree_build(remaining_letters, remaining_letters),
                    );
                    TrieNode { nodes, value: None }
                }
            }
        }
        None => TrieNode {
            nodes: HashMap::new(),
            value: Some(original_rule.to_string()),
        },
    }
}

fn get_valid_rule(node: &TrieNode, char_list: &[char]) -> Option<String> {
    if char_list.len() == 0 {
        return node.value.clone();
    }
    let current_char = char_list[0];
    let new_node = node.nodes.get(&current_char);
    match new_node {
        Some(new_node) => {
            let new_list = &char_list[1..];
            get_valid_rule(new_node, new_list)
        }
        None => node.value.clone(),
    }
}

fn build_trie(page: Page) -> HashMap<char, TrieNode> {
    page.raw_rules
        .iter()
        .fold(HashMap::new(), |mut map: HashMap<char, TrieNode>, rule| {
            map.insert(rule.chars().next().unwrap(), TrieNode::new(rule));
            map
        })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_read_input() {
        let page = extract_rules_and_rows_from_input("test.txt");
        assert_eq!(page.raw_rules.len(), 21);
        assert_eq!(page.raw_lines.len(), 6);
    }
    fn test_build_rules() {
        let page = extract_rules_and_rows_from_input("test.txt");
        let rule_trie = RuleTrie::new(page);
    }
}
