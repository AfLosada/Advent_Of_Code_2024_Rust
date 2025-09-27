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
    rule_trie: Vec<TrieNode>,
}

impl RuleTrie {
    fn new(page: &Page) -> RuleTrie {
        let rule_trie =
            page.raw_rules
                .iter()
                .fold(vec![], |mut map: Vec<TrieNode>, rule: &String| {
                    println!("rule inserted: {}", rule);
                    let trie_node = TrieNode::new(rule);
                    // trie_node.print(0);
                    map.push(trie_node);
                    map
                });
        RuleTrie { rule_trie }
    }
    fn find_rules(&self, rule: String) -> Option<String> {
        let is_equal = |new_rule: &str| &rule == new_rule;
        let found_rule = self.rule_trie.iter().find(|node| {
            //node.print(0, 0);
            match &node.value {
                Some(v) => is_equal(&v),
                None => {
                  //TODO: FIX only get rules that match
                    let rule_option = node.nodes.get(&rule);
                    match rule_option {
                        Some(_) => {
                          true
                        },
                        None => false,
                    }
                }
            }
        });
        match found_rule {
            Some(found_rule) => (found_rule).get_value_list().first().cloned(),
            None => None,
        }
    }
}

#[derive(Clone)]
struct TrieNode {
    nodes: HashMap<String, TrieNode>,
    value: Option<String>,
}

impl TrieNode {
    fn new(rule: &str) -> TrieNode {
        cyclical_tree_build(rule, rule.split("|").collect())
    }
    fn print(&self, level: u8, depth: u8) {
      if level > depth {
        return
      }
      println!("-----------------LEVEL: {}--------------------------------------------", level);
      println!("value: {}", self.value.clone().unwrap_or("None".to_owned()));
      println!("nodes: {:?}", self.nodes.clone().into_keys());
      self.nodes.clone().into_iter().for_each(|(key, node)|{
        println!("-----------------KEY: {}--------------------------------------------", key);
        node.print(level+1, depth);
      });
    }

    fn get_value_list(&self) -> Vec<String> {
      match &self.value {
        Some(v) => vec![v.to_string()],
        None => self.nodes.iter().map(|(key, node)|{
          node.get_value_list()
        }).flatten().collect::<Vec<_>>()
      }
    }
}

fn cyclical_tree_build(original_rule: &str, remaining_words: Vec<&str>) -> TrieNode {
    let mut nodes: HashMap<String, TrieNode> = HashMap::new();
    let first_letter = remaining_words.first().cloned();
    let a = if remaining_words.len() > 0 {
        remaining_words[1..].to_owned()
    } else {
        vec![]
    };
    let remaining_letters = a;

    match first_letter {
        Some(first_letter) => {
            let node = nodes.get(first_letter).cloned();
            match node {
                Some(n) => n,
                None => {
                    nodes.insert(
                        first_letter.to_string(),
                        cyclical_tree_build(original_rule, remaining_letters),
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

fn is_rule_valid_on_line(line: &str, rule: &str) -> Option<bool> {
    let mut split_rules = rule.split("|");
    let first = split_rules.next().unwrap();
    let last = split_rules.next().unwrap();
    let (pos, _) = line
        .split(",")
        .enumerate()
        .find(|(_, letter)| *letter == first)?;
    let (pos_second, _) = line
        .split(",")
        .enumerate()
        .find(|(_, letter)| *letter == last)?;
    if pos < pos_second {
        return Some(true);
    }
    Some(false)
}

fn get_valid_rule(node: &TrieNode, word_list: &[String]) -> Option<String> {
    if word_list.len() == 0 {
        return node.value.clone();
    }
    let current_word = &word_list[0];
    let new_node = node.nodes.get(current_word);
    match new_node {
        Some(new_node) => {
            let new_list = &word_list[1..];
            get_valid_rule(new_node, new_list)
        }
        None => node.value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let page = extract_rules_and_rows_from_input("test.txt");
        assert_eq!(page.raw_rules.len(), 21);
        assert_eq!(page.raw_lines.len(), 6);
    }
    #[test]
    fn test_build_rules() {
        let page = extract_rules_and_rows_from_input("test.txt");
        let rule_trie = RuleTrie::new(&page);
        let lines_that_comply_rules = page
            .raw_lines
            .iter()
            .filter(|line| {
                println!("checking line: ");
                let mut rules: Vec<Option<String>> = line
                    .split(",")
                    .map(|letter| rule_trie.find_rules(letter.to_string()))
                    .collect();
                rules.retain(|rule| rule.is_some());
                println!("rules that match: {:?}", rules);
                rules.iter().all(|rule| {
                    let rule = rule.clone().unwrap();
                    println!("match line {} with rule {}", line, &rule);
                    match is_rule_valid_on_line(line, rule.as_str()) {
                        Some(true) => true,
                        _ => false,
                    }
                })
            })
            .collect::<Vec<_>>();
        lines_that_comply_rules.iter().for_each(|line| {
            println!("line that complies: {}", line.as_str());
        });
    }
}
