use std::{env, num::ParseIntError};

use Advent_of_Code_2024::{day4, day5, day6, day7};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].as_str();
    let file_name = args.get(2).map_or("test", |v| v).to_string();
    let part = args.get(3).map_or("1", |v| v);
    match day {
        &"day4" => day4::day4::main(&file_name),   
        &"day5" => day5::day5::main(file_name),   
        &"day6" => day6::day6::main(file_name),   
        &"day7" => day7::day7::main(&file_name, part),   
        _ => {
            assert!(false, "there is no matching day")
        }
    }
}
