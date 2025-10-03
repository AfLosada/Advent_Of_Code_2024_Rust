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
#[derive(Clone)]
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
    rule_map_trie: HashMap<(String, String), TrieNode>,
    page: Page,
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
        let rule_map_trie = page
            .raw_rules
            .iter()
            .fold(HashMap::new(), |mut map, rule: &String| {
                let split_rules = rule.split("|").collect::<Vec<&str>>();
                let first = split_rules.first().unwrap();
                let second = split_rules.get(1).unwrap();
                map.insert(
                    (first.to_owned().to_owned(), second.to_owned().to_owned()),
                    TrieNode::new(rule),
                );
                map
            });
        RuleTrie {
            rule_trie,
            rule_map_trie,
            page: page.clone(),
        }
    }
    fn find_rules_map(&self, combination: (String, String)) -> Option<&TrieNode> {
        self.rule_map_trie.get(&combination)
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
                        Some(_) => true,
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
    fn find_rule_from_pair(&self, (first_rule, second_rule): (String, String)) -> Option<String> {
        self.rule_trie
            .iter()
            .find_map(|node| match node.nodes.get(&first_rule) {
                Some(first_trie) => {
                    // first_trie.print(0, 3);
                    match first_trie.nodes.get(&second_rule) {
                        Some(second_trie) => second_trie.value.clone(),
                        None => None,
                    }
                }
                None => None,
            })
    }

    fn get_lines_that_comply_rules(&self) -> Vec<(String, Vec<Option<&TrieNode>>)> {
        let lines_that_comply_rules = self
            .page
            .raw_lines
            .iter()
            .filter_map(|line| {
                println!("checking line: {}", line);
                let combinations =
                    create_combinations(vec![], line.split(",").collect::<Vec<&str>>(), 2);
                println!("combinations: {:?}", combinations);
                let mut rules = combinations
                    .into_iter()
                    .map(|pair| {
                        //rule_trie.find_rule_from_pair((pair.0.to_owned(), pair.1.to_owned()))
                        self.find_rules_map((pair.0.to_owned(), pair.1.to_owned()))
                    })
                    .collect::<Vec<_>>();
                rules.retain(|rule| rule.is_some());
                println!("rules that match: {:?}", rules.len());

                if rules.iter().len() == 0 {
                    return None;
                }
                let comply = match {
                    rules.iter().all(|rule| {
                        let binding = rule.unwrap().get_value_list();
                        let rule = binding.first().unwrap();
                        println!("match line {} with rule {}", line, &rule);
                        match is_rule_valid_on_line(line, rule.as_str()) {
                            Some(true) => true,
                            _ => false,
                        }
                    })
                } {
                    true => Some((line.to_string(), rules.to_owned())),
                    false => None,
                };
                comply
            })
            .collect::<Vec<_>>();
        lines_that_comply_rules.iter().for_each(|line| {
            println!("line that complies: {}", line.0.as_str());
        });
        lines_that_comply_rules
    }
    fn get_lines_that_dont_comply_rules(&self) -> Vec<(String, Vec<Option<&TrieNode>>)> {
        let lines_that_comply_rules = self
            .page
            .raw_lines
            .iter()
            .filter_map(|line| {
                println!("checking line: {}", line);
                let combinations =
                    create_combinations(vec![], line.split(",").collect::<Vec<&str>>(), 2);
                println!("combinations: {:?}", combinations);
                let mut rules = combinations
                    .into_iter()
                    .map(|pair| {
                        //rule_trie.find_rule_from_pair((pair.0.to_owned(), pair.1.to_owned()))
                        self.find_rules_map((pair.0.to_owned(), pair.1.to_owned()))
                    })
                    .collect::<Vec<_>>();
                rules.retain(|rule| rule.is_some());
                println!("rules that match: {:?}", rules.len());

                if rules.iter().len() == 0 {
                    return None;
                }
                let comply = match {
                    rules.iter().any(|rule| {
                        let binding = rule.unwrap().get_value_list();
                        let rule = binding.first().unwrap();
                        println!("match line {} with rule {}", line, &rule);
                        !match is_rule_valid_on_line(line, rule.as_str()) {
                            Some(true) => true,
                            _ => false,
                        }
                    })
                } {
                    true => Some((line.to_string(), rules.to_owned())),
                    false => None,
                };
                comply
            })
            .collect::<Vec<_>>();
        lines_that_comply_rules.iter().for_each(|line| {
            println!("line that complies: {}", line.0.as_str());
        });
        lines_that_comply_rules
    }
}

fn create_combinations<T: Clone>(mut acc: Vec<(T, T)>, vec: Vec<T>, size: u8) -> Vec<(T, T)> {
    if vec.len() == 1 {
        return acc;
    }
    let first = vec.first().unwrap();
    let remainder = &vec[1..];
    let mut combinations = remainder
        .iter()
        .flat_map(|curr| vec![(first.clone(), curr.clone()), (curr.clone(), first.clone())])
        .collect::<Vec<_>>();
    acc.append(&mut combinations);
    create_combinations(acc, remainder.to_vec(), size)
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
            return;
        }
        println!(
            "-----------------LEVEL: {}--------------------------------------------",
            level
        );
        println!("value: {}", self.value.clone().unwrap_or("None".to_owned()));
        println!("nodes: {:?}", self.nodes.clone().into_keys());
        self.nodes.clone().into_iter().for_each(|(key, node)| {
            println!(
                "-----------------KEY: {}--------------------------------------------",
                key
            );
            node.print(level + 1, depth);
        });
    }

    fn get_value_list(&self) -> Vec<String> {
        match &self.value {
            Some(v) => vec![v.to_string()],
            None => self
                .nodes
                .iter()
                .map(|(key, node)| node.get_value_list())
                .flatten()
                .collect::<Vec<_>>(),
        }
    }
}

fn cyclical_tree_build(original_rule: &str, remaining_words: Vec<&str>) -> TrieNode {
    let mut nodes: HashMap<String, TrieNode> = HashMap::new();
    let first_letter = remaining_words.first().cloned();
    let second_letter = remaining_words.first().cloned();
    let a = if remaining_words.len() > 0 {
        remaining_words[1..].to_owned()
    } else {
        vec![]
    };
    let remaining_letters = a;

    let mut curr_trie = match first_letter {
        Some(first_letter) => {
            let node = nodes.get(first_letter).cloned();
            match node {
                Some(n) => n,
                None => {
                    nodes.insert(
                        first_letter.to_string(),
                        cyclical_tree_build(original_rule, remaining_letters.clone()),
                    );
                    TrieNode { nodes, value: None }
                }
            }
        }
        None => TrieNode {
            nodes: HashMap::new(),
            value: Some(original_rule.to_string()),
        },
    };
    match second_letter {
        Some(second_letter) => {
            let node = curr_trie.nodes.get(second_letter).cloned();
            match node {
                Some(n) => n,
                None => {
                    curr_trie.nodes.insert(
                        second_letter.to_string(),
                        cyclical_tree_build(original_rule, remaining_letters),
                    );
                    TrieNode {
                        nodes: curr_trie.nodes.clone(),
                        value: None,
                    }
                }
            }
        }
        None => TrieNode {
            nodes: HashMap::new(),
            value: Some(original_rule.to_string()),
        },
    };
    curr_trie
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

fn get_midpoints_of_lines(lines: &Vec<String>) -> Vec<i32> {
    lines
        .into_iter()
        .map(|curr: &String| {
            let split_lines = curr.split(",").collect::<Vec<_>>();
            split_lines[split_lines.len() / 2].parse::<i32>().unwrap()
        })
        .collect::<Vec<_>>()
}

fn fix_line(line: &Vec<String>, rules: &Vec<String>) -> String {
    println!("line to fix: {}", line.join(","));
    let invalid_rules = rules
        .iter()
        .filter(|rule| {
            let split_line = &line.join(",");
            !is_rule_valid_on_line(split_line, rule.as_str()).is_some()
        })
        .collect::<Vec<_>>();
    if invalid_rules.len() == 0 {
        return line.join(",");
    }
    let rule_to_fix = rules[0].clone();
    println!("rule to fix: {}", rule_to_fix);
    let fixed_line_after_rule = fix_rule(&line, &rule_to_fix)
        .split(",")
        .map(|l| l.to_string())
        .collect::<Vec<_>>();
    println!("old line: {}", line.join(","));
    println!("fixed line: {}", fixed_line_after_rule.join(","));
    fix_line(&fixed_line_after_rule, rules)
}

fn fix_rule(line: &Vec<String>, rule: &String) -> String {
    let split_rules = rule.split("|").collect::<Vec<_>>();
    let first_number_of_rule = split_rules[0];
    let second_number_of_rule = split_rules[1];
    let first_number_position_in_vec = line.iter().position(|n| n == first_number_of_rule).unwrap();
    let second_number_position_in_vec = line
        .iter()
        .position(|n| n == second_number_of_rule)
        .unwrap();
    let new_string = move_item_in_vec(
        line,
        second_number_position_in_vec,
        first_number_position_in_vec + 1,
    );
    new_string.join(",")
}

fn move_item_in_vec<T: Clone>(vec: &Vec<T>, origin: usize, destination: usize) -> Vec<T> {
    let item = vec[origin].clone();
    let slice_without_item = [&vec[..origin], &vec[origin + 1..]].concat();
    [
        &slice_without_item[..destination],
        &[item],
        &slice_without_item[destination..],
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use core::fmt;
    use std::string;

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
        let lines = rule_trie
            .get_lines_that_comply_rules()
            .iter()
            .map(|line| line.0.clone())
            .collect::<Vec<String>>();
        let sum: i32 = get_midpoints_of_lines(&lines).into_iter().sum();
        println!("Sum of midpoints: {}", sum);
        assert_eq!(sum, 143);
    }
    #[test]
    fn test_build_rules_2() {
        let page = extract_rules_and_rows_from_input("test.txt");
        let rule_trie = RuleTrie::new(&page);
        let lines = rule_trie.get_lines_that_dont_comply_rules();
        println!(
            "lines to fix: {:?}",
            lines
                .iter()
                .map(|l| {
                    let aas = l.0.clone();
                    aas
                })
                .collect::<Vec<_>>()
        );
        let fixed_lines = lines
            .iter()
            .map(|(line, rules)| {
                let split_line = line.split(",").map(|l| l.to_string()).collect::<Vec<_>>();
                let rules = rules
                    .iter()
                    .map(|rule| rule.unwrap().get_value_list().first().unwrap().to_string())
                    .collect::<Vec<_>>();
                println!("rules to fix: {}", rules.join(","));
                println!("line to fix: {}", line);
                fix_line(&split_line, &rules)
            })
            .collect::<Vec<_>>();
        println!("{:#}", fixed_lines.join("\n"));
        let sum: i32 = get_midpoints_of_lines(&fixed_lines).into_iter().sum();
        println!("Sum of midpoints: {}", sum);
        assert_eq!(sum, 123);
    }
    #[test]
    fn test_answer() {
        let page = extract_rules_and_rows_from_input("input.txt");
        let rule_trie = RuleTrie::new(&page);
        let lines = rule_trie
            .get_lines_that_comply_rules()
            .iter()
            .map(|line| line.0.clone())
            .collect::<Vec<String>>();
        let sum: i32 = get_midpoints_of_lines(&lines).into_iter().sum();
        println!("Sum of midpoints: {}", sum);
        assert_eq!(sum, 4609);
    }
}
