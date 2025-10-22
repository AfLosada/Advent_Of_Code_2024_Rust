use std::{fmt, ptr::eq};

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
            acc.visit_matrix
                .push(vec![VisitDirections(Vec::new()); curr.len()]);
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

#[derive(Clone)]
struct Matrices {
    visit_matrix: Vec<Vec<VisitDirections>>,
    guard_path: Vec<Path>,
    node_matrix: Vec<Vec<Node>>,
}

#[derive(Clone)]
struct VisitDirections(Vec<Direction>);

impl ToString for VisitDirections {
    fn to_string(&self) -> String {
        self.0.len().to_string()
    }
}

#[derive(Clone, PartialEq)]
enum NodeType {
    GUARD,
    OBSTACLE,
    EMPTY,
}

#[derive(Clone, PartialEq)]
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

#[derive(Clone)]
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
        let column = get_column_of_matrix(&self.node_matrix, guard_position.y as usize);
        let row = self.node_matrix[guard_position.x as usize].to_vec();
        match direction {
            Direction::UP | Direction::DOWN => (column, row),
            Direction::LEFT | Direction::RIGHT => (row, column),
        }
    }

    fn has_candidate_for_obstruction(
        &self,
        guard_position: &Position<i32>,
        direction: &Direction,
    ) -> Option<Position<i32>> {
        let (mut current_row, perpendicular_row) =
            self.extract_intersection(guard_position, direction);

        let current_guard_position = current_row
            .iter()
            .position(|node| node.node_type == NodeType::GUARD)
            .unwrap();
        match direction {
            Direction::UP | Direction::LEFT => {
                current_row.reverse();
            }
            Direction::DOWN | Direction::RIGHT => {}
        };
        let has_obstacle_before = current_row[current_guard_position..]
            .iter()
            .any(|node| node.node_type == NodeType::OBSTACLE);
        let has_2_obstacles_perpendicular = perpendicular_row
            .iter()
            .filter(|node| node.node_type == NodeType::OBSTACLE)
            .count()
            >= 2;
        if has_2_obstacles_perpendicular && has_obstacle_before {
            let current_guard_node = &current_row[current_guard_position];
            let obstacle_node_position = current_guard_node.position.move_to_direction(direction);
            return Some(obstacle_node_position);
        }
        None
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
            self.visit_matrix[guard_position.x as usize][guard_position.y as usize]
                .0
                .push(direction.clone());
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
            self.visit_matrix[guard_position.x as usize][guard_position.y as usize]
                .0
                .push(direction.clone());
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
            self.visit_matrix[guard_position.x as usize][guard_position.y as usize]
                .0
                .push(direction.clone());
            let mut new_path = Path { nodes: Vec::new() };
            let last_path_nodes = &mut new_path.nodes;
            last_path_nodes.push(current_node.clone());
            self.guard_path.push(new_path);
            // self.navigate_guard(direction);
            return Some(direction.clone());
        }
    }
}

fn navigate(
    mut matrices: Matrices,
    direction: Option<&Direction>,
    can_continue: impl Fn(&Matrices, Option<&Direction>) -> bool,
) -> Matrices {
    let mut direction = direction.cloned();
    while can_continue(&matrices, direction.as_ref()) {
        let dir_non_option = direction.unwrap();
        direction = matrices.navigate_and_get_direction(&dir_non_option);
    }
    matrices
}

fn get_column_of_matrix<T: Clone>(matrix: &Vec<Vec<T>>, column: usize) -> Vec<T> {
    let mut vec: Vec<T> = Vec::with_capacity(matrix.len());
    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            if y == column {
                vec[x] = matrix[x][y].clone();
            }
        }
    }
    vec
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

fn add_obstruction_in_front(
    matrix: &mut Matrices,
    guard_position: &Position<i32>,
    direction: &Direction,
) -> Position<i32> {
    let obstruction_position = guard_position.move_to_direction(direction);
    matrix.node_matrix[obstruction_position.x as usize][obstruction_position.y as usize]
        .node_type = NodeType::OBSTACLE;
    obstruction_position
}

fn remove_obstruction_from_position(matrix: &mut Matrices, position: &Position<i32>) {
    matrix.node_matrix[position.x as usize][position.y as usize].node_type = NodeType::EMPTY;
}

fn obstruction_navigation(matrix: &Matrices, position: &Position<i32>, direction: &Direction) {
    let mut unique_directions = directions.0.clone();
    unique_directions.dedup_by(|a, b| a == b);
    navigate(matrices.clone(), |matrix, direction| {
        // creating a new matrix each time so I can use my navigate function without any concerns
        // side effects, what?! who?
        match direction {
            Some(direction) => {
                let mut matrix = matrix.clone();
                add_obstruction_in_front(&mut matrix, position, direction);
                false
            }
            None => true,
        }
    });
}

#[cfg(test)]
mod tests {
    use core::fmt;
    use std::{cell::RefCell, rc::Rc, string};

    use super::*;
    #[test]
    fn test_read_input() {
        let matrices = extract_matrices_from_input("test.txt");
        assert_eq!(matrices.visit_matrix.len(), 10);
        assert_eq!(matrices.visit_matrix[0].len(), 10);
    }
    #[test]
    fn navigate_test() {
        let matrices = extract_matrices_from_input("test.txt");
        let matrices = navigate(matrices, |_, direction| direction.is_some());
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
        let mut matrices = extract_matrices_from_input("test.txt");
        let direction = &matrices.navigate_and_get_direction(&Direction::UP);
        let matrices = navigate(matrices, direction.as_ref(), |_, direction| {
            direction.is_some()
        });
        let uniques = matrices
            .visit_matrix
            .iter()
            .flatten()
            .filter(|number| number.0.len() > 0)
            .count();
        println!("uniques: {}", uniques);
        assert_eq!(uniques, 41);
    }
    #[test]
    fn navigate_with_real_answer() {
        let mut matrices = extract_matrices_from_input("input.txt");
        let direction = matrices.navigate_and_get_direction(&Direction::UP);
        let matrices = navigate(matrices, direction.as_ref(), |_, direction| {
            direction.is_some()
        });
        let matrix_string = print_matrix(&matrices.visit_matrix);
        print!("{}", matrix_string);
        let uniques = matrices
            .visit_matrix
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(y, col)| match col.0.len() > 0 {
                        true => Some((
                            Position {
                                x: x.clone(),
                                y: y.clone(),
                            },
                            col.clone(),
                        )),
                        false => None,
                    })
            })
            .flatten()
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
        // let matrices = extract_matrices_from_input("test.txt");
        // let candidates_for_obstuction: Rc<RefCell<Vec<Position<i32>>>> =
        //     Rc::new(RefCell::new(vec![]));
        // let matrices = navigate(matrices, |matrix, position, direction| {
        //     let candidate = matrix.has_candidate_for_obstruction(position, direction);
        //     if candidate.is_some() {
        //         candidates_for_obstuction
        //             .borrow_mut()
        //             .push(candidate.clone().unwrap());
        //     }
        //     candidate.clone()
        // });
        //     .visit_matrix
        //     .iter()
        // let uniques = matrices
        //     .flatten()
        //     .filter(|number| number > &&0)
        //     .count();
        // println!("uniques: {}", uniques);

        // The actual heuristic implies a simulation.
        // If I turn right, will I return to the same spot in the same direction? If it means yes the simulation can stop.

        let matrices = extract_matrices_from_input("input.txt");
        let direction = matrices.navigate_and_get_direction(&Direction::UP);
        let matrices = navigate(matrices, direction.as_ref(), |_, direction| {
            direction.is_some()
        });
        let matrix_string = print_matrix(&matrices.visit_matrix);
        print!("{}", matrix_string);
        let uniques: Vec<(Position<i32>, VisitDirections)> = matrices
            .visit_matrix
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(y, col)| match col.0.len() > 0 {
                        true => Some((
                            Position {
                                x: x.clone() as i32,
                                y: y.clone() as i32,
                            },
                            col.clone(),
                        )),
                        false => None,
                    })
            })
            .flatten()
            .collect();
        uniques.iter().map(|(position, directions)| {
            let mut unique_directions = directions.0.clone();
            unique_directions.dedup_by(|a, b| a == b);
            unique_directions.iter().for_each(|direction| {
                // creating a new matrix each time so I can use my navigate function without any concerns
                // side effects, what?! who?
                let mut matrix = matrices.clone();
                let obstruction_position =
                    add_obstruction_in_front(&mut matrix, position, direction);
                let did_loop = Rc::new(RefCell::new(false))
                navigate(
                    matrices.clone(),
                    Some(direction),
                    |matrix, direction| match direction {
                        Some(direction) => {
                            
                        },
                        None => {
                            did_loop.borrow_mut().to_owned() = false;
                            true
                        },
                    },
                );
                remove_obstruction_from_position(&mut matrix, &obstruction_position);
            });
        });
    }
}
