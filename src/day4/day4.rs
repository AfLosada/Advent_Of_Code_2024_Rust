use std::fs::{self};

fn read_input() -> String {
    fs::read_to_string("src/day4/input.txt").unwrap()
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

#[derive(Clone)]
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
}

struct Cell {
    position: Position,
    value: u8,
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
                        value: *c,
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

fn calculate_position_diff(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::TopLeft => (-1, -1),
        Direction::TopRight => (1, -1),
        Direction::BottomLeft => (-1, 1),
        Direction::BottomRight => (1, 1),
    }
}

fn get_cell_for_direction_in_position<'a>(
    direction: &'a Direction,
    position: &'a Position,
    grid: &'a Grid,
) -> Option<&'a Cell> {
    let (x, y) = calculate_position_diff(direction);
    let new_x = position.x as i32 + x;
    let new_y = position.y as i32 + y;
    let new_position = Position { x: new_x, y: new_y };
    if !new_position.is_valid(grid) {
        return None;
    }
    Some(&grid.cells[new_position.y as usize][new_position.x as usize])
}

fn dfs_navigate_to_find_word(
    position: &Position,
    grid: &Grid,
    word: &str,
    current_word: &str,
    positions_visited: Vec<Position>,
) -> Vec<Option<Vec<Position>>> {
    if word == current_word {
        println!("{}", word);
        return vec![Some([positions_visited, vec![position.clone()]].concat())];
      }
      
    println!("current word: {}", current_word);
    ALL_DIRECTIONS
        .iter()
        .map(|direction| {
            let cell = get_cell_for_direction_in_position(direction, &position, grid);
            match cell {
                Some(cell) => {
                    let new_current_word = (current_word.to_owned()
                        + String::from_utf8(vec![cell.value]).unwrap().as_str())
                    .to_string();
                    let mut new_positions = positions_visited.clone();
                    new_positions.push(cell.position.clone());
                    dfs_navigate_to_find_word(
                        &cell.position,
                        grid,
                        word,
                        new_current_word.as_str(),
                        new_positions,
                    )
                }
                None => vec![None],
            }
        })
        .collect::<Vec<Vec<Option<Vec<Position>>>>>()
        .concat()
}

fn find_xmas(grid: &Grid) -> Vec<Option<Vec<Position>>> {
    let mut ans: Vec<Vec<Option<Vec<Position>>>> = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.cells[y as usize][x as usize].value == b'X' {
                let position = Position {
                    x: (x as i32),
                    y: (y as i32),
                };

            }
        })
        .collect::<Vec<Vec<Option<Vec<Position>>>>>()
        .concat()
                ans.push(
                    dfs_navigate_to_find_word(&position, grid, "XMAS", "X", vec![position.clone()])
                        .clone(),
                );
            }
        }
    }
    ans.concat()
}

pub fn main() {
    let input = read_input();
    let clean_input = remove_useless_characters(&input);
    let grid = Grid::new(&clean_input);
    find_xmas(&grid);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_clean_input() {
        let input = read_input();
        let clean_input = remove_useless_characters(&input);

        println!("{}", clean_input);
    }

    #[test]
    fn test_do_everything() {
        let input = read_input();
        let clean_input = remove_useless_characters(&input);
        let grid = Grid::new(&clean_input);
        find_xmas(&grid);
    }
}
