use std::ptr::eq;

use crate::util::util;

fn extract_matrices_from_input(input: &str) -> Matrices {
    let lines = util::read_input("day6", input);
    println!(
        "lines: {}",
        lines.lines().enumerate().collect::<Vec<_>>().len()
    );
    lines.lines().enumerate().fold(
        Matrices {
            visit_matrix: Vec::new(),
            guard_path: Vec::new(),
            node_matrix: Vec::new(),
        },
        |mut acc, (x, curr)| {
            // println!("matrix len: {}", acc.visit_matrix.len());
            // println!("matrix len: {}", acc.node_matrix.len());
            acc.visit_matrix.push(vec![0; curr.len()]);
            // println!(
            //     "visit matrix {}: {:?}, len: {}",
            //     x,
            //     acc.visit_matrix[x],
            //     acc.visit_matrix[x].len()
            // );
            acc.node_matrix.push(vec![
                Node {
                    node_type: NodeType::EMPTY,
                    position: Position { x: 0, y: 0 }
                };
                curr.len()
            ]);
            curr.char_indices().fold(acc, |mut acc, (y, c)| {
                let position = Position {
                    x: x as i32,
                    y: y as i32,
                };
                let node = Node::from(c, position);
                acc.visit_matrix[x][y] = 0;
                acc.node_matrix[x][y] = node;
                Matrices {
                    visit_matrix: acc.visit_matrix,
                    guard_path: acc.guard_path,
                    node_matrix: acc.node_matrix,
                }
            })
        },
    )
}

type Matrix<T> = Vec<Row<T>>;
type Row<T> = Vec<T>;

struct Matrices {
    visit_matrix: Vec<Vec<i32>>,
    guard_path: Vec<Path>,
    node_matrix: Vec<Vec<Node>>,
}

#[derive(Clone, PartialEq)]
enum NodeType {
    GUARD,
    OBSTACLE,
    EMPTY,
}

#[derive(Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Node {
    pub fn from(c: char, position: Position<i32>) -> Node {
        match c {
            '#' => Node {
                node_type: NodeType::OBSTACLE,
                position,
            },
            '.' => Node {
                node_type: NodeType::EMPTY,
                position,
            },
            '^' => Node {
                node_type: NodeType::GUARD,
                position,
            },
            '>' => Node {
                node_type: NodeType::GUARD,
                position,
            },
            '<' => Node {
                node_type: NodeType::GUARD,
                position,
            },
            'v' => Node {
                node_type: NodeType::GUARD,
                position,
            },
            _ => Node {
                node_type: NodeType::EMPTY,
                position,
            },
        }
    }
    fn calculate_new_direction(&self, current_direction: &Direction) -> Direction {
        match current_direction {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        }
    }
}

#[derive(Clone)]
struct Position<T> {
    x: T,
    y: T,
}

impl Position<i32> {
    pub fn move_to_direction(&self, direction: &Direction) -> Position<i32> {
        match direction {
            Direction::LEFT => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::RIGHT => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::UP => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::DOWN => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone)]
struct Node {
    node_type: NodeType,
    position: Position<i32>,
}

struct Path {
    nodes: Vec<Node>,
}

impl Matrices {
    fn find_guard(&self) -> Option<Position<i32>> {
        for x in 0..self.node_matrix.len() {
            for y in 0..self.node_matrix.len() {
                if self.node_matrix[x][y].node_type == NodeType::GUARD {
                    return Some(Position {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        None
    }

    fn extract_intersection(
        &self,
        guard_position: &Position<i32>,
        direction: &Direction,
    ) -> (Row<Node>, Row<Node>) {
      match direction {
        Direction::UP | Direction::DOWN => {
          let first_row = self.node_matrix[guard_position.x as usize].clone();
          let rotated_matrix = rotate_matrix_90(self.node_matrix.clone());
          let second_row = rotated_matrix[guard_position.x as usize].clone();
          (first_row, second_row)
        },
        Direction::LEFT | Direction::RIGHT => {
          let first_row = self.node_matrix[guard_position.x as usize].clone();
          let rotated_matrix = rotate_matrix_90(self.node_matrix.clone());
          let second_row = rotated_matrix[guard_position.x as usize].clone();
          (second_row, first_row)
        }
      }
    }

    fn has_candidate_for_obstruction(
        &self,
        guard_position: &Position<i32>,
        direction: &Direction,
    ) -> Option<Node> {
        let (current_row, perpendicular_row) = self.extract_intersection(guard_position, direction);
        let current_guard_position = current_row.iter().position(|node| node.node_type == NodeType::GUARD).unwrap();
        match direction {
          Direction::UP | Direction::LEFT => {
            let has_obstacle_before = current_row[current_guard_position..].iter().any(|node|{
              node.node_type == NodeType::OBSTACLE
            });
            let has_2_obstacles_perpendicular = perpendicular_row.iter().filter(|node|{
              node.node_type == NodeType::OBSTACLE
            }).count() >= 2;
            
            None
          },
          Direction::DOWN | Direction::RIGHT => {
            None
          }
        }
    }

    fn navigate_and_get_direction(&mut self, direction: &Direction) -> Option<Direction> {
        let guard_position = self.find_guard().unwrap();
        let new_position = guard_position.move_to_direction(direction);
        if new_position.x < 0
            || new_position.y < 0
            || new_position.x >= self.node_matrix.len() as i32
            || new_position.y >= self.node_matrix.len() as i32
        {
            // Exit map
            let current_node =
                &mut self.node_matrix[guard_position.x as usize][guard_position.y as usize];
            current_node.node_type = NodeType::EMPTY;
            self.visit_matrix[guard_position.x as usize][guard_position.y as usize] += 1;
            let last_pos = self.guard_path.len() - 1;
            let last_path_nodes = &mut self.guard_path[last_pos].nodes;
            last_path_nodes.push(current_node.clone());
            return None;
        } else if self.node_matrix[new_position.x as usize][new_position.y as usize].node_type
            == NodeType::OBSTACLE
        {
            // Turn around
            let current_node =
                &self.node_matrix[guard_position.x as usize][guard_position.y as usize];
            self.visit_matrix[guard_position.x as usize][guard_position.y as usize] += 1;
            let mut new_path = Path { nodes: Vec::new() };
            let last_path_nodes = &mut new_path.nodes;
            last_path_nodes.push(current_node.clone());
            self.guard_path.push(new_path);
            let direction = (&current_node).calculate_new_direction(direction);
            // self.navigate_guard();
            return Some(direction);
        } else {
            // Move forward
            let current_node =
                &mut self.node_matrix[guard_position.x as usize][guard_position.y as usize];
            current_node.node_type = NodeType::EMPTY;
            let new_node = &mut self.node_matrix[new_position.x as usize][new_position.y as usize];
            new_node.node_type = NodeType::GUARD;
            let current_node =
                &self.node_matrix[guard_position.x as usize][guard_position.y as usize];
            self.visit_matrix[guard_position.x as usize][guard_position.y as usize] += 1;
            let mut new_path = Path { nodes: Vec::new() };
            let last_path_nodes = &mut new_path.nodes;
            last_path_nodes.push(current_node.clone());
            self.guard_path.push(new_path);
            // self.navigate_guard(direction);
            return Some(direction.clone());
        }
    }
}

fn navigate(mut matrices: Matrices) -> Matrices {
    let mut direction = matrices.navigate_and_get_direction(&Direction::UP);
    while direction.is_some() {
        direction = matrices.navigate_and_get_direction(&direction.unwrap());
    }
    matrices
}

fn rotate_matrix_90<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut new_matrix: Vec<Vec<T>> = vec![];
    for x in 0..matrix.len() {
        for y in 0..matrix.len() {
            new_matrix[y][x] = matrix[x][y].clone();
        }
    }
    matrix
}

fn print_matrix<T: ToString>(matrix: &Vec<Vec<T>>) -> String {
    matrix
        .iter()
        .map(|row| {
            format!(
                "{}\n",
                row.iter()
                    .fold("".to_string(), |acc, curr| acc + &curr.to_string())
            )
        })
        .fold("".to_string(), |a, curr| a + curr.as_str())
}
fn count_obstructions_in_vector(vector: &Vec<Node>) -> usize {
    vector
        .iter()
        .filter(|node| node.node_type == NodeType::OBSTACLE)
        .count()
}

#[cfg(test)]
mod tests {
    use core::fmt;
    use std::string;

    use super::*;
    #[test]
    fn test_read_input() {
        let matrices = extract_matrices_from_input("test.txt");
        assert_eq!(matrices.visit_matrix.len(), 10);
        assert_eq!(matrices.visit_matrix[0].len(), 10);
    }
    #[test]
    fn navigate_test() {
        let mut matrices = extract_matrices_from_input("test.txt");
        let matrices = navigate(matrices);
        let matrix_string = print_matrix(&matrices.visit_matrix);
        print!("{}", matrix_string);
        assert_eq!(
            matrix_string,
            "\
0000000000\n\
0000211120\n\
0000100010\n\
0000100010\n\
0021212010\n\
0010101010\n\
0021212120\n\
0211112200\n\
0211112100\n\
0000000100\n"
        );
    }
    #[test]
    fn navigate_with_test_answer() {
        let matrices = extract_matrices_from_input("test.txt");
        let matrices = navigate(matrices);
        let uniques = matrices
            .visit_matrix
            .iter()
            .flatten()
            .filter(|number| number > &&0)
            .count();
        println!("uniques: {}", uniques);
        assert_eq!(uniques, 41);
    }
    #[test]
    fn navigate_with_real_answer() {
        let matrices = extract_matrices_from_input("input.txt");
        let matrices = navigate(matrices);
        let matrix_string = print_matrix(&matrices.visit_matrix);
        print!("{}", matrix_string);
        let uniques = matrices
            .visit_matrix
            .iter()
            .flatten()
            .filter(|number| number > &&0)
            .count();
        println!("uniques: {}", uniques);
        assert_eq!(uniques, 5199);
    }
    #[test]
    fn navigate_part_2_with_test() {
        // a position that loops the guard has:
        // 1. 1 obstacle in the same direction of the guard, but behind the guard.
        // 2. 2 obstacles in the perpendicular direction of the guard.
        // the obstacle has to be put 1 step away from the 2 obstacles in the direction of the guard.
    }
}
