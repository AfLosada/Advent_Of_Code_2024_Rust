use core::panic;
use std::fmt::Debug;
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
        "2" => {}
        _ => {
            println!("day 7 no part selected")
        }
    }
}

fn extract_rules_and_rows_from_input(input: &str) -> String {
    util::read_input("day7", input)
}

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

impl<T: Add<Output = T> + Mul<Output = T> + FromStr> Operation<T> {
    fn new_from_string(string_operation: String, operation_position: usize) -> Operation<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let (first, second) = string_operation.split_at(operation_position);
        let first_number = first.parse::<T>();
        let second_number = second.parse::<T>();
        Operation {
            operator: string_to_operator(
                string_operation
                    .chars()
                    .nth(operation_position)
                    .unwrap()
                    .to_string(),
            ),
            first: first_number.expect("error parsing first number"),
            second: second_number.expect("error parsing second number"),
        }
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
    fn apply_operation<T: Add<Output = T> + Mul<Output = T>>(&self, first: T, second: T) -> T {
        match self {
            OPERATOR::ADD => first + second,
            OPERATOR::MULTIPLY => first * second,
        }
    }
}

fn recursive_permutation(numbers: Vec<String>) -> Vec<String> {
    if numbers.len() == 1 {
        return numbers.clone();
    }
    let number_pairs = numbers.into_iter().permutations(2);
	numbers.iter().multi_cartesian_product();
    number_pairs.map(|pair| {
        // In my mind, we only need one iteration for the permutations.
        // and we shouldn't resolve it, we only need to create the string
        let composed_operations = get_possible_operands().iter().map(|op| {
			let first = pair[0];
			let second = pair[1];
			format!("{}{}{}", first, op.symbol(), second)
		}).collect::<Vec<_>>();
		composed_operations.iter().map(|composed_operation|{
			recursive_permutation(numbers)
		});
    })
    // Then, I have to apply a different operation between each window
}

// number_pairs.map(|pair| {
// 	let first: i32 = pair[0].parse().unwrap();
// 	let second: i32 = pair[1].parse().unwrap();
// 	// I can apply any operation to this pair, will store it as an array
// 	get_operations().iter().map(|op| op.apply_operation(first, second)).collect::<Vec<_>>()
// });

fn part1(input: &str) {
    let operations = extract_rules_and_rows_from_input(input);
    let split = operations.split(":").collect::<Vec<_>>();
    let left = split[0];
    let right = split[1];
    let numbers = right.split_whitespace().collect::<Vec<_>>();
}
