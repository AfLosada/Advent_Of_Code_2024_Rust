use core::panic;
use num::BigInt;
use std::fmt::{Debug, format};
use std::{
    ops::{Add, Mul},
    str::FromStr,
};

use crate::util::util;
use itertools::Itertools;

pub fn main(input: &str, part: &str) {
    match part {
        "1" => {
            part1(input);
        }
        "2" => part2(input),
        _ => {
            println!("day 7 no part selected")
        }
    }
}

fn extract_rules_and_rows_from_input(input: &str) -> String {
    util::read_input("day7", input)
}

#[derive(Clone, Copy)]
enum OPERATOR {
    ADD,
    MULTIPLY,
}

fn get_possible_operands() -> Vec<OPERATOR> {
    return vec![OPERATOR::ADD, OPERATOR::MULTIPLY];
}

struct Operation<T: Add<Output = T> + Mul<Output = T>> {
    operator: OPERATOR,
    first: T,
    second: T,
}

impl<T: Add<Output = T> + Mul<Output = T> + FromStr + Clone> Operation<T>
where
    BigInt: From<T>,
{
    fn new_from_string(string_operation: String, operator: OPERATOR) -> Operation<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let split_line = string_operation
            .split(&operator.symbol())
            .collect::<Vec<_>>();
        let first = split_line[0];
        let second = split_line[1];
        let first_number = first.parse::<T>();
        let second_number = second.parse::<T>();
        Operation {
            operator: operator,
            first: first_number.expect("error parsing first number"),
            second: second_number.expect("error parsing second number"),
        }
    }
    fn apply_operation(&self) -> BigInt {
        return self
            .operator
            .apply_operation::<T>(self.first.clone(), self.second.clone());
    }
}

fn string_to_operator(st: String) -> OPERATOR {
    match st.as_str() {
        "+" => OPERATOR::ADD,
        "*" => OPERATOR::MULTIPLY,
        _ => panic!(),
    }
}

impl OPERATOR {
    fn symbol(&self) -> String {
        match self {
            OPERATOR::ADD => "+".to_string(),
            OPERATOR::MULTIPLY => "*".to_string(),
        }
    }
    fn symbol_byte(&self) -> u8 {
        match self {
            OPERATOR::ADD => b'+',
            OPERATOR::MULTIPLY => b'*',
        }
    }
    fn apply_operation<T: Add<Output = T> + Mul<Output = T>>(&self, first: T, second: T) -> BigInt
    where
        BigInt: From<T>,
    {
        match self {
            OPERATOR::ADD => BigInt::from(first).add(BigInt::from(second)),
            OPERATOR::MULTIPLY => BigInt::from(first).mul(BigInt::from(second)),
        }
    }
}

fn generate_operand_string_list_from_pair(first: String, second: String) -> Vec<String> {
    get_possible_operands()
        .iter()
        .map(|operand| format!("{}{}{}", first, operand.symbol(), second))
        .collect()
}

fn recursive_walk(remainder: Vec<String>, acc: String) -> Vec<String> {
    if remainder.len() < 2 {
        return vec![acc];
    }

    // if remainder.len() < 2 {
    //     let first = &remainder[0];
    //     let equations = get_possible_operands()
    //         .iter()
    //         .map(|operator| format!("{}{}{}", acc, operator.symbol(), first).to_string())
    //         .collect::<Vec<String>>();
    //     return equations;
    // }

    let first = &remainder[0];
    let second = &remainder[1];
    let reduced_remainder = remainder[2..].to_vec();
    let equations = get_possible_operands()
        .iter()
        .map(|operator| format!("{}{}{}", first, operator.symbol(), second).to_string())
        .collect::<Vec<String>>();

    let equations = equations
        .iter()
        .map(|new_equation| {
            let reduced_remainder =
                vec![vec![new_equation.clone()], reduced_remainder.clone()].concat();
            recursive_walk(reduced_remainder.to_vec(), new_equation.to_string())
        })
        .flatten()
        .collect::<Vec<_>>();
    equations
}

fn generate_equations_for_line(line: String) -> Vec<String> {
    let split_equation = line
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    recursive_walk(split_equation, "".to_string())
}

// fn recursive_permutation(numbers: Vec<String>) -> Vec<String> {
//     if numbers.len() == 1 {
//         return numbers.clone();
//     }
//     let number_pairs = numbers.into_iter().permutations(2);
// 	numbers.iter().multi_cartesian_product();
//     number_pairs.map(|pair| {
//         // In my mind, we only need one iteration for the permutations.
//         // and we shouldn't resolve it, we only need to create the string
//         let composed_operations = get_possible_operands().iter().map(|op| {
// 			let first = pair[0];
// 			let second = pair[1];
// 			format!("{}{}{}", first, op.symbol(), second)
// 		}).collect::<Vec<_>>();
// 		composed_operations.iter().map(|composed_operation|{
// 			recursive_permutation(numbers)
// 		});
//     })
//     // Then, I have to apply a different operation between each window
// }

// number_pairs.map(|pair| {
// 	let first: i32 = pair[0].parse().unwrap();
// 	let second: i32 = pair[1].parse().unwrap();
// 	// I can apply any operation to this pair, will store it as an array
// 	get_operations().iter().map(|op| op.apply_operation(first, second)).collect::<Vec<_>>()
// });

fn walk_and_resolve_equation(line: String) -> BigInt {
    let bytes = line.as_bytes();
    let mut first_operand = None;
    let mut second_operand = None;
    let mut operator: Option<OPERATOR> = None;
    let mut acc = BigInt::from(0);
    for byte in bytes {
        match byte {
            b'+' | b'*' => match operator {
                Some(old_operator) => {
                    let operation = format!(
                        "{}{}{}",
                        first_operand.clone().unwrap(),
                        old_operator.symbol(),
                        second_operand.clone().unwrap()
                    );
                    let op = Operation::<BigInt>::new_from_string(operation, old_operator);
                    acc = op.apply_operation();
                    first_operand = Some(acc.to_string());
                    second_operand = None;
                    operator = Some(string_to_operator(String::from_utf8(vec![*byte]).unwrap()));
                }
                None => {
                    operator = Some(string_to_operator(String::from_utf8(vec![*byte]).unwrap()));
                }
            },
            _ => match first_operand {
                Some(ref first) => match operator {
                    Some(operator) => match second_operand {
                        Some(second) => {
                            second_operand = Some(format!(
                                "{}{}",
                                second,
                                String::from_utf8(vec![byte.clone()]).unwrap()
                            ))
                        }
                        None => {
                            second_operand = Some(String::from_utf8(vec![byte.clone()]).unwrap())
                        }
                    },
                    None => {
                        first_operand = Some(format!(
                            "{}{}",
                            first,
                            String::from_utf8(vec![byte.clone()]).unwrap()
                        ))
                    }
                },
                None => {
                    first_operand = Some(format!(
                        "{}",
                        String::from_utf8(vec![byte.clone()]).unwrap()
                    ))
                }
            },
        }
    }
    if first_operand.is_some() && second_operand.is_some() && operator.is_some() {
        let operation = format!(
            "{}{}{}",
            first_operand.clone().unwrap(),
            operator.unwrap().symbol(),
            second_operand.clone().unwrap()
        );
        let op = Operation::<BigInt>::new_from_string(operation, operator.unwrap());
        acc = op.apply_operation();
    }
    acc
}

fn part1(input: &str) {
    let file = extract_rules_and_rows_from_input(input);
    let lines = file.lines();
    let lines_that_match: BigInt = lines
        .filter_map(|line| {
            let split = line.split(":").collect::<Vec<_>>();
            let result = split[0].to_string();
            let operation = split[1].to_string();
            let ops = generate_equations_for_line(operation.to_string());
            println!(
                "==============================================================================="
            );
            println!("There are this many options: {}", ops.len());
            let results_that_match = ops
                .iter()
                .filter_map(|op| {
                    let parsed_result = result.parse::<BigInt>().unwrap();
                    let resolved_equation = walk_and_resolve_equation(op.to_string());
                    return if parsed_result == resolved_equation {
                        Some(resolved_equation)
                    } else {
                        None
                    };
                })
                .collect::<Vec<_>>();
            results_that_match.iter().for_each(|op| println!("{}", op));
            println!(
                "Result {} is matched by this many lines {}",
                result,
                results_that_match.len()
            );
            if results_that_match.len() > 1 {
                Some(result.parse::<BigInt>().unwrap())
            } else {
                None
            }
        })
        .reduce(|acc, curr| acc.add(curr))
        .unwrap();
    println!("lines that match: {}", lines_that_match)
}

fn part2(input: &str) {}
