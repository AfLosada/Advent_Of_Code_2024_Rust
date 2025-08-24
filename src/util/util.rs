use std::fs;

pub fn read_input(day: &str,file: &str) -> String {
  fs::read_to_string(format!("src/{}/{}", day, file)).unwrap()
}
