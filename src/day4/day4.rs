use std::{
    collections::HashSet,
    fs::{self},
};

fn read_input(file: &str) -> String {
    fs::read_to_string(format!("src/day4/{}", file)).unwrap()
}

fn remove_useless_characters(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            String::from_utf8(
                line.as_bytes()
                    .iter()
                    .map(|c| match c {
                        b'X' => *c,
                        b'M' => *c,
                        b'A' => *c,
                        b'S' => *c,
                        _ => b'.',
                    })
                    .collect::<Vec<u8>>(),
            )
            .unwrap()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_valid(&self, grid: &Grid) -> bool {
        let x = match self.x {
            x if x >= 0 && (x as u32) < grid.width => true,
            _ => false,
        };
        let y = match self.y {
            y if y >= 0 && (y as u32) < grid.height => true,
            _ => false,
        };
        x && y
    }
    fn add(&self, position: &Position) -> Position {
        Position {
            x: self.x + position.x,
            y: self.y + position.y,
        }
    }
}

struct Cell {
    position: Position,
    value: char,
}

struct Grid {
    height: u32,
    width: u32,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(str_input: &str) -> Grid {
        let cells = str_input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let char_arr = line.as_bytes();
                char_arr
                    .iter()
                    .enumerate()
                    .map(|(x, c)| Cell {
                        position: Position {
                            x: (x as i32),
                            y: (y as i32),
                        },
                        value: *c as char,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = (&cells).len() as u32;
        let width = (&cells[0]).len() as u32;

        Grid {
            cells,
            height,
            width,
        }
    }

    fn new_from_position_vec(&self, positions_to_keep: &HashSet<Position>) -> Grid {
        let new_cells: Vec<Vec<Cell>> = self
            .cells
            .iter()
            .map(|line| {
                line.iter()
                    .map(|cell| {
                        let position = (&cell.position).clone();
                        let will_keep = positions_to_keep.contains(&position);
                        Cell {
                            position,
                            value: if will_keep { cell.value } else { '.' },
                        }
                    })
                    .collect()
            })
            .collect();
        let height = (&new_cells).len() as u32;
        let width = (&new_cells[0]).len() as u32;
        Grid {
            cells: new_cells,
            height,
            width,
        }
    }
    fn print_grid(&self) {
        self.cells.iter().for_each(|row| {
            let line = row
                .iter()
                .map(|cell| cell.value.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", line)
        })
    }
}

const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::TopLeft,
    Direction::TopRight,
    Direction::BottomLeft,
    Direction::BottomRight,
];

fn calculate_position_diff_position(direction: &Direction) -> Position {
    match direction {
        Direction::Up => Position { x: 0, y: -1 },
        Direction::Down => Position { x: 0, y: 1 },
        Direction::Left => Position { x: -1, y: 0 },
        Direction::Right => Position { x: 1, y: 0 },
        Direction::TopLeft => Position { x: -1, y: -1 },
        Direction::TopRight => Position { x: 1, y: -1 },
        Direction::BottomLeft => Position { x: -1, y: 1 },
        Direction::BottomRight => Position { x: 1, y: 1 },
    }
}

fn get_cell_for_direction_in_position<'a>(
    direction: &'a Direction,
    position: &'a Position,
    grid: &'a Grid,
) -> Option<&'a Cell> {
    let new_position = calculate_position_diff_position(direction).add(position);
    if !new_position.is_valid(grid) {
        return None;
    }
    Some(&grid.cells[new_position.y as usize][new_position.x as usize])
}

fn dfs_navigate_to_find_word(
    position: &Position,
    direction: &Direction,
    grid: &Grid,
    word: &str,
    current_word: &str,
    positions_visited: Vec<Position>,
) -> Vec<Option<Vec<Position>>> {
    if word == current_word {
        return vec![Some([positions_visited, vec![position.clone()]].concat())];
    }

    if word.len() < current_word.len() {
        return vec![None];
    }

    let expected_letter = word.as_bytes()[positions_visited.len()] as char;

    let cell = get_cell_for_direction_in_position(direction, &position, grid);
    match cell {
        Some(cell) => {
            let current_letter = cell.value.to_string();
            let new_current_word = (current_word.to_owned() + current_letter.as_str()).to_string();

            if current_letter != expected_letter.to_string() {
                return vec![None];
            }

            let mut new_positions = positions_visited.clone();
            new_positions.push(cell.position.clone());
            dfs_navigate_to_find_word(
                &cell.position,
                direction,
                grid,
                word,
                new_current_word.as_str(),
                new_positions,
            )
        }
        None => vec![None],
    }
}

fn find_xmas(grid: &Grid) -> Vec<Option<Vec<Position>>> {
    let mut ans: Vec<Vec<Option<Vec<Position>>>> = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.cells[y as usize][x as usize].value == 'X' {
                let position = Position {
                    x: (x as i32),
                    y: (y as i32),
                };
                ALL_DIRECTIONS.iter().for_each(|direction| {
                    ans.push(
                        dfs_navigate_to_find_word(
                            &position,
                            &direction,
                            grid,
                            "XMAS",
                            "X",
                            vec![position.clone()],
                        )
                        .clone(),
                    )
                });
            }
        }
    }
    ans.concat()
}

fn print_xmas(xmas: &Vec<Option<Vec<Position>>>, grid: &Grid) -> Grid {
    let mut xmas_non_null = xmas.clone();
    xmas_non_null.retain(|word| word.is_some());
    let positions_to_keep = HashSet::from_iter(xmas_non_null.iter().flatten().cloned().flatten());
    let new_grid = grid.new_from_position_vec(&positions_to_keep);
    println!("Amount of xmas found: {}", xmas_non_null.len());
    new_grid.print_grid();
    new_grid
}

fn find_x_mas(grid: &Grid) -> Vec<Option<Vec<Position>>> {
    let mut ans: Vec<Vec<Option<Vec<Position>>>> = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.cells[y as usize][x as usize].value == 'A' {
                let position = Position {
                    x: (x as i32),
                    y: (y as i32),
                };

                let top_left = match get_cell_for_direction_in_position(
                    &Direction::TopLeft,
                    &position,
                    grid,
                ) {
                    Some(a) => Some(a.value),
                    _ => None,
                };
                if top_left.is_none() {
                    continue;
                }

                let top_right =
                    match get_cell_for_direction_in_position(&Direction::TopRight, &position, grid)
                    {
                        Some(a) => Some(a.value),
                        _ => None,
                    };
                if top_right.is_none() {
                    continue;
                }

                let bottom_left = match get_cell_for_direction_in_position(
                    &Direction::BottomLeft,
                    &position,
                    grid,
                ) {
                    Some(a) => Some(a.value),
                    _ => None,
                };
                if bottom_left.is_none() {
                    continue;
                }

                let bottom_right = match get_cell_for_direction_in_position(
                    &Direction::BottomRight,
                    &position,
                    grid,
                ) {
                    Some(a) => Some(a.value),
                    _ => None,
                };
                if bottom_right.is_none() {
                    continue;
                }

                let top_left_non_set = [
                    top_left.unwrap().to_string(),
                    "A".to_owned(),
                    bottom_right.unwrap().to_string(),
                ];
                let top_left_diagonal = HashSet::from(top_left_non_set.clone());

                let bottom_left_non_set = [
                    bottom_left.unwrap().to_string(),
                    "A".to_owned(),
                    top_right.unwrap().to_string(),
                ];
                let bottom_left_diagonal = HashSet::from(bottom_left_non_set.clone());

                let mas_set = HashSet::from(["M".to_owned(), "A".to_owned(), "S".to_owned()]);

                if top_left_diagonal.eq(&bottom_left_diagonal)
                    && top_left_diagonal.eq(&mas_set)
                    && bottom_left_diagonal.eq(&mas_set)
                {
                    continue;
                }

                let top_str = top_left_non_set.join(" ");
                let bot_str = bottom_left_non_set.join(" ");
                println!("top_left: {}. bottom_left: {}", top_str, bot_str);
                println!("X is equal to S: {} {} {}", "");

                let top_left_position =
                    calculate_position_diff_position(&Direction::TopLeft).add(&position);
                let bottom_left_position =
                    calculate_position_diff_position(&Direction::BottomLeft).add(&position);
                let bottom_right_position =
                    calculate_position_diff_position(&Direction::BottomRight).add(&position);
                let top_right_position =
                    calculate_position_diff_position(&Direction::TopRight).add(&position);

                ans.push(vec![Some(vec![
                    top_left_position,
                    bottom_left_position,
                    position,
                    bottom_right_position,
                    top_right_position,
                ])]);
            }
        }
    }
    ans.concat()
}

pub fn main(input: &str) {
    let input = read_input(input);
    let clean_input = remove_useless_characters(&input);
    let grid = Grid::new(&clean_input);
    let xmas = find_xmas(&grid);
    print_xmas(&xmas, &grid);
}

pub fn main_2(input: &str) {
    let input = read_input(input);
    let clean_input = remove_useless_characters(&input);
    let grid = Grid::new(&clean_input);
    let xmas = find_x_mas(&grid);
    print_xmas(&xmas, &grid);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_clean_input() {
        let input = read_input("test");
        let clean_input = remove_useless_characters(&input);

        println!("{}", clean_input);
    }

    #[test]
    fn test_input() {
        main("test.txt");
    }
    #[test]
    fn test_all() {
        main("input.txt");
    }
    #[test]
    fn test_input_2() {
        main_2("test.txt");
    }

    #[test]
    fn test_all_2() {
        main_2("input.txt");
    }
}
