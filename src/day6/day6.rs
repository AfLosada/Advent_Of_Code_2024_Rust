use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    ptr::eq,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::util::util;
use rayon::prelude::*;

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

#[derive(Clone, PartialEq, Eq, Hash)]
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

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
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

    fn navigate_and_get_direction(
        &mut self,
        direction: &Direction,
        guard_position: &Position<i32>,
    ) -> (Option<Direction>, Option<Position<i32>>) {
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
            return (None, None);
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
            return (Some(direction), Some(guard_position.clone()));
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
            let guard_path = &mut self.guard_path;
            let last_path = guard_path.last_mut();
            match last_path {
                Some(last_path) => {
                    last_path.nodes.push(current_node.clone());
                }
                None => {
                    let mut new_path: Path = Path { nodes: Vec::new() };
                    new_path.nodes.push(current_node.clone());
                    guard_path.push(new_path.clone());
                }
            };
            // self.navigate_guard(direction);
            return (Some(direction.clone()), Some(new_position.clone()));
        }
    }
}

fn navigate(
    mut matrices: Matrices,
    direction: Option<&Direction>,
    current_position: Option<&Position<i32>>,
    can_continue: impl Fn(&Matrices, Option<&Position<i32>>, Option<&Direction>) -> bool,
) -> Matrices {
    let mut curr_direction = direction.cloned();
    let mut current_position = current_position.cloned();
    while can_continue(
        &matrices,
        current_position.as_ref(),
        curr_direction.as_ref(),
    ) {
        let dir_non_option = curr_direction.unwrap();
        let pos_non_option = current_position.unwrap();
        let (direction, position) =
            matrices.navigate_and_get_direction(&dir_non_option, &pos_non_option);
        curr_direction = direction;
        current_position = position;
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
) -> Option<Position<i32>> {
    let obstruction_position = guard_position.move_to_direction(direction);
    if obstruction_position.x as usize >= matrix.node_matrix.len()
        || obstruction_position.y as usize >= matrix.node_matrix.len()
    {
        return None;
    }
    matrix.node_matrix[obstruction_position.x as usize][obstruction_position.y as usize]
        .node_type = NodeType::OBSTACLE;
    Some(obstruction_position)
}

fn remove_obstruction_from_position(matrix: &mut Matrices, position: &Position<i32>) {
    matrix.node_matrix[position.x as usize][position.y as usize].node_type = NodeType::EMPTY;
}

pub fn part_2() {
    let mut matrices = extract_matrices_from_input("input.txt");
    let guard_position = matrices.find_guard().unwrap();
    let (direction, guard_position) =
        matrices.navigate_and_get_direction(&Direction::UP, &guard_position);
    let matrices = navigate(
        matrices,
        direction.as_ref(),
        guard_position.as_ref(),
        |_, _, direction| direction.is_some(),
    );
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

    let total_obstructions: usize = uniques
        .par_iter()
        .map(|(start_position, directions)| {
            let mut unique_directions = directions.0.clone();
            unique_directions.dedup_by(|a, b| a == b);
            println!("Directions to parallelize: {:#}", unique_directions.len());
            unique_directions
                .par_iter()
                .filter(|direction| {
                    // creating a new matrix each time so I can use my navigate function without any concerns
                    // side effects, what?! who?
                    let mut matrix = matrices.clone();
                    // also add the position of the guard in this instant
                    matrix.node_matrix[start_position.x as usize][start_position.y as usize]
                        .node_type = NodeType::GUARD;

                    let obstruction_position =
                        add_obstruction_in_front(&mut matrix, start_position, direction);
                    if obstruction_position.is_none() {
                        return false;
                    }
                    let obstruction_position = obstruction_position.unwrap();

                    let did_loop = Arc::new(Mutex::new(false));
                    let (direction, guard_position) =
                        matrix.navigate_and_get_direction(direction, &start_position);
                    let (direction, guard_position) = matrix
                        .navigate_and_get_direction(&direction.unwrap(), &guard_position.unwrap());

                    if direction.is_some() {
                        let new_guard_position = start_position;
                        if new_guard_position.x >= 0
                            || new_guard_position.y >= 0
                            || new_guard_position.x < matrix.node_matrix.len() as i32
                            || new_guard_position.y < matrix.node_matrix.len() as i32
                        {
                            if matrix.node_matrix[new_guard_position.x as usize]
                                [new_guard_position.y as usize]
                                .node_type
                                != NodeType::OBSTACLE
                            {
                                let visit_map: Arc<
                                    Mutex<HashMap<(Direction, Position<i32>), bool>>,
                                > = Arc::new(Mutex::new(HashMap::new()));
                                navigate(
                                    matrix.clone(),
                                    direction.as_ref(),
                                    guard_position.as_ref(),
                                    |matrix, guard_position, direction| match direction {
                                        Some(_) => {
                                            // if *guard_position.unwrap() == start_position.clone()
                                            //     && matrix.guard_path.last().unwrap().nodes.len() > 1
                                            // {
                                            // }
                                            if visit_map
                                                .lock()
                                                .unwrap()
                                                .get(&(
                                                    direction.unwrap().clone(),
                                                    guard_position.unwrap().clone(),
                                                ))
                                                .is_some()
                                            {
                                                *did_loop.lock().unwrap() = true;
                                                return false;
                                            }
                                            println!(
                                                "Can continue at {:?} from position {:?}",
                                                guard_position.unwrap(),
                                                start_position
                                            );
                                            visit_map.lock().unwrap().insert(
                                                (
                                                    direction.unwrap().clone(),
                                                    guard_position.unwrap().clone(),
                                                ),
                                                true,
                                            );
                                            return true;
                                        }
                                        None => {
                                            *did_loop.lock().unwrap() = false;
                                            false
                                        }
                                    },
                                );
                            }
                        }
                    }
                    remove_obstruction_from_position(&mut matrix, &obstruction_position);
                    let did_loop = *did_loop.lock().unwrap();
                    if did_loop {
                        println!("Obstruction in position: {:?}", &obstruction_position);
                    }
                    did_loop
                })
                .count()
        })
        .sum();
    println!("Total obstructions: {}", total_obstructions);
    assert_eq!(total_obstructions, 6);
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
        let mut matrices = extract_matrices_from_input("test.txt");
        let (direction, guard_position) =
            matrices.navigate_and_get_direction(&Direction::UP, &matrices.find_guard().unwrap());
        let matrices = navigate(
            matrices,
            direction.as_ref(),
            guard_position.as_ref(),
            |_, _, direction| direction.is_some(),
        );
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
        let (direction, guard_position) =
            matrices.navigate_and_get_direction(&Direction::UP, &matrices.find_guard().unwrap());
        let matrices = navigate(
            matrices,
            direction.as_ref(),
            guard_position.as_ref(),
            |_, _, direction| direction.is_some(),
        );
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
        let (direction, guard_position) =
            matrices.navigate_and_get_direction(&Direction::UP, &matrices.find_guard().unwrap());
        let matrices = navigate(
            matrices,
            direction.as_ref(),
            guard_position.as_ref(),
            |_, _, direction| direction.is_some(),
        );
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
        let mut matrices = extract_matrices_from_input("test.txt");
        let (direction, guard_position) =
            matrices.navigate_and_get_direction(&Direction::UP, &matrices.find_guard().unwrap());
        let matrices = navigate(
            matrices,
            direction.as_ref(),
            guard_position.as_ref(),
            |_, _, direction| direction.is_some(),
        );
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

        let total_obstructions: usize = uniques
            .par_iter()
            .map(|(position, directions)| {
                let mut unique_directions = directions.0.clone();
                unique_directions.dedup_by(|a, b| a == b);
                println!("Directions to parallelize: {:#}", unique_directions.len());
                unique_directions
                    .par_iter()
                    .filter(|direction| {
                        // creating a new matrix each time so I can use my navigate function without any concerns
                        // side effects, what?! who?
                        let mut matrix = matrices.clone();
                        // also add the position of the guard in this instant
                        matrix.node_matrix[position.x as usize][position.y as usize].node_type =
                            NodeType::GUARD;

                        let obstruction_position =
                            add_obstruction_in_front(&mut matrix, position, direction);
                        if obstruction_position.is_none() {
                            return false;
                        }
                        let obstruction_position = obstruction_position.unwrap();

                        let did_loop = Arc::new(Mutex::new(false));
                        let (direction, guard_position) =
                            matrix.navigate_and_get_direction(direction, &position);
                        let (direction, guard_position) = matrix.navigate_and_get_direction(
                            &direction.unwrap(),
                            &guard_position.unwrap(),
                        );
                        if direction.is_some() {
                            let new_guard_position = position;
                            if new_guard_position.x >= 0
                                || new_guard_position.y >= 0
                                || new_guard_position.x < matrix.node_matrix.len() as i32
                                || new_guard_position.y < matrix.node_matrix.len() as i32
                            {
                                if matrix.node_matrix[new_guard_position.x as usize]
                                    [new_guard_position.y as usize]
                                    .node_type
                                    != NodeType::OBSTACLE
                                {
                                    navigate(
                                        matrix.clone(),
                                        direction.as_ref(),
                                        guard_position.as_ref(),
                                        |matrix, guard_position, direction| match direction {
                                            Some(_) => {
                                                if *guard_position.unwrap() == position.clone()
                                                    && matrix.guard_path.last().unwrap().nodes.len()
                                                        > 1
                                                {
                                                    *did_loop.lock().unwrap() = true;
                                                    return false;
                                                }
                                                return true;
                                            }
                                            None => {
                                                *did_loop.lock().unwrap() = false;
                                                false
                                            }
                                        },
                                    );
                                }
                            }
                        }
                        remove_obstruction_from_position(&mut matrix, &obstruction_position);
                        let did_loop = *did_loop.lock().unwrap();
                        if did_loop {
                            println!("Obstruction in position: {:?}", &obstruction_position);
                        }
                        did_loop
                    })
                    .count()
            })
            .sum();
        println!("Total obstructions: {}", total_obstructions);
        assert_eq!(total_obstructions, 6);
    }
    #[test]
    fn navigate_part_2_with_answer() {
        let mut matrices = extract_matrices_from_input("input.txt");
        let (direction, guard_position) =
            matrices.navigate_and_get_direction(&Direction::UP, &matrices.find_guard().unwrap());
        let matrices = navigate(
            matrices,
            direction.as_ref(),
            guard_position.as_ref(),
            |_, _, direction| direction.is_some(),
        );
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

        let total_obstructions: usize = uniques
            .iter()
            .map(|(position, directions)| {
                let mut unique_directions = directions.0.clone();
                unique_directions.dedup_by(|a, b| a == b);
                println!("Directions to parallelize: {:#}", unique_directions.len());
                unique_directions
                    .par_iter()
                    .filter(|direction| {
                        // creating a new matrix each time so I can use my navigate function without any concerns
                        // side effects, what?! who?
                        let mut matrix = matrices.clone();
                        // also add the position of the guard in this instant
                        matrix.node_matrix[position.x as usize][position.y as usize].node_type =
                            NodeType::GUARD;

                        let obstruction_position =
                            add_obstruction_in_front(&mut matrix, position, direction);
                        if obstruction_position.is_none() {
                            return false;
                        }
                        let obstruction_position = obstruction_position.unwrap();

                        let did_loop = Arc::new(Mutex::new(false));
                        let (direction, guard_position) =
                            matrix.navigate_and_get_direction(direction, &position);
                        let (direction, guard_position) = matrix.navigate_and_get_direction(
                            &direction.unwrap(),
                            &guard_position.unwrap(),
                        );
                        if direction.is_some() {
                            let new_guard_position = position;
                            if new_guard_position.x >= 0
                                || new_guard_position.y >= 0
                                || new_guard_position.x < matrix.node_matrix.len() as i32
                                || new_guard_position.y < matrix.node_matrix.len() as i32
                            {
                                if matrix.node_matrix[new_guard_position.x as usize]
                                    [new_guard_position.y as usize]
                                    .node_type
                                    != NodeType::OBSTACLE
                                {
                                    navigate(
                                        matrix.clone(),
                                        direction.as_ref(),
                                        guard_position.as_ref(),
                                        |matrix, guard_position, direction| match direction {
                                            Some(_) => {
                                                if *guard_position.unwrap() == position.clone()
                                                    && matrix.guard_path.last().unwrap().nodes.len()
                                                        > 1
                                                {
                                                    *did_loop.lock().unwrap() = true;
                                                    return false;
                                                }
                                                println!(
                                                    "Can continue at {:?}",
                                                    guard_position.unwrap()
                                                );
                                                return true;
                                            }
                                            None => {
                                                *did_loop.lock().unwrap() = false;
                                                false
                                            }
                                        },
                                    );
                                }
                            }
                        }
                        remove_obstruction_from_position(&mut matrix, &obstruction_position);
                        let did_loop = *did_loop.lock().unwrap();
                        if did_loop {
                            println!("Obstruction in position: {:?}", &obstruction_position);
                        }
                        did_loop
                    })
                    .count()
            })
            .sum();
        println!("Total obstructions: {}", total_obstructions);
        assert_eq!(total_obstructions, 6);
    }
}
