use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

enum Visibility {
    Unknown,
    Visible,
}

struct Tree {
    height: u32,
    visible: Visibility,
}

struct Field {
    trees: Vec<Vec<Tree>>,
}

impl Field {
    fn get(&self, row: usize, col: usize) -> &Tree {
        self.trees.get(row).unwrap().get(col).unwrap()
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Tree {
        self.trees.get_mut(row).unwrap().get_mut(col).unwrap()
    }

    fn rows(&self) -> usize {
        self.trees.len()
    }

    fn cols(&self) -> usize {
        self.trees[0].len()
    }

    fn total_visible(&self) -> usize {
        self.trees
            .iter()
            .flatten()
            .map(|t| match t.visible {
                Visibility::Unknown => 0,
                Visibility::Visible => 1,
            })
            .sum()
    }

    fn valid_position(&self, position: &Position) -> bool {
        if position.row() < 0 || position.row() >= self.rows().try_into().unwrap() {
            false
        } else {
            if position.col() < 0 || position.col() >= self.cols().try_into().unwrap() {
                false
            } else {
                true
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Position {
    row: i32,
    col: i32,
    orientation: Option<Direction>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Movement {
    Forward,
    Left,
    Right,
    Back,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Rotation {
    Left,
    Right,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
            orientation: None,
        }
    }
    fn new_oriented(row: i32, col: i32, orientation: Direction) -> Self {
        Self {
            row,
            col,
            orientation: Some(orientation),
        }
    }

    fn row(&self) -> i32 {
        self.row
    }
    fn col(&self) -> i32 {
        self.col
    }
    fn move_absolute(&self, direction: Direction, distance: i32) -> Position {
        match direction {
            Direction::Up => Position {
                row: self.row - distance,
                col: self.col,
                orientation: self.orientation,
            },
            Direction::Down => Position {
                row: self.row + distance,
                col: self.col,
                orientation: self.orientation,
            },
            Direction::Left => Position {
                row: self.row,
                col: self.col - distance,
                orientation: self.orientation,
            },
            Direction::Right => Position {
                row: self.row,
                col: self.col + distance,
                orientation: self.orientation,
            },
        }
    }
    fn move_relative(&self, movement: Movement, distance: i32) -> Position {
        if let Some(orientation) = self.orientation {
            match movement {
                Movement::Forward => match orientation {
                    Direction::Up => self.move_absolute(Direction::Up, distance),
                    Direction::Down => self.move_absolute(Direction::Down, distance),
                    Direction::Left => self.move_absolute(Direction::Left, distance),
                    Direction::Right => self.move_absolute(Direction::Right, distance),
                },
                Movement::Left => match orientation {
                    Direction::Up => self.move_absolute(Direction::Left, distance),
                    Direction::Down => self.move_absolute(Direction::Right, distance),
                    Direction::Left => self.move_absolute(Direction::Down, distance),
                    Direction::Right => self.move_absolute(Direction::Up, distance),
                },
                Movement::Right => match orientation {
                    Direction::Up => self.move_absolute(Direction::Right, distance),
                    Direction::Down => self.move_absolute(Direction::Left, distance),
                    Direction::Left => self.move_absolute(Direction::Up, distance),
                    Direction::Right => self.move_absolute(Direction::Down, distance),
                },
                Movement::Back => match orientation {
                    Direction::Up => self.move_absolute(Direction::Down, distance),
                    Direction::Down => self.move_absolute(Direction::Up, distance),
                    Direction::Left => self.move_absolute(Direction::Right, distance),
                    Direction::Right => self.move_absolute(Direction::Left, distance),
                },
            }
        } else {
            panic!("Position has no orientation")
        }
    }

    fn rotate(&self, rotation: Rotation) -> Self {
        let new_orientation = match self.orientation {
            Some(Direction::Up) => match rotation {
                Rotation::Left => Direction::Left,
                Rotation::Right => Direction::Right,
            },
            Some(Direction::Right) => match rotation {
                Rotation::Left => Direction::Up,
                Rotation::Right => Direction::Down,
            },
            Some(Direction::Down) => match rotation {
                Rotation::Left => Direction::Right,
                Rotation::Right => Direction::Left,
            },
            Some(Direction::Left) => match rotation {
                Rotation::Left => Direction::Down,
                Rotation::Right => Direction::Up,
            },
            None => panic!("Position has no orientation"),
        };
        Self {
            row: self.row,
            col: self.col,
            orientation: Some(new_orientation),
        }
    }
}

fn parse_input(input: &str) -> Field {
    let mut rows = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            let height = char as u32 - '0' as u32;
            row.push(Tree {
                height,
                visible: Visibility::Unknown,
            });
        }
        rows.push(row);
    }
    Field { trees: rows }
}

fn part1(input: &str) -> String {
    let mut field = parse_input(input);

    // Looking right
    for row in 0..field.rows() {
        let mut highest = field.get(row, 0).height;
        field.get_mut(row, 0).visible = Visibility::Visible;

        for col in 1..field.cols() {
            let tree = field.get_mut(row, col);
            if tree.height > highest {
                tree.visible = Visibility::Visible;
                highest = tree.height;
            }
        }
    }

    // Looking left
    for row in 0..field.rows() {
        let mut highest = field.get(row, field.cols() - 1).height;
        field.get_mut(row, field.cols() - 1).visible = Visibility::Visible;

        for col in (0..field.cols() - 1).rev() {
            let tree = field.get_mut(row, col);
            if tree.height > highest {
                tree.visible = Visibility::Visible;
                highest = tree.height;
            }
        }
    }

    // Looking down
    for col in 0..field.cols() {
        let mut highest = field.get(0, col).height;
        field.get_mut(0, col).visible = Visibility::Visible;

        for row in 1..field.rows() {
            let tree = field.get_mut(row, col);
            if tree.height > highest {
                tree.visible = Visibility::Visible;
                highest = tree.height;
            }
        }
    }

    // Looking up
    for col in 0..field.cols() {
        let mut highest = field.get(field.rows() - 1, col).height;
        field.get_mut(field.rows() - 1, col).visible = Visibility::Visible;

        for row in (0..field.rows() - 1).rev() {
            let tree = field.get_mut(row, col);
            if tree.height > highest {
                tree.visible = Visibility::Visible;
                highest = tree.height;
            }
        }
    }

    field.total_visible().to_string()
}

fn part2(input: &str) -> String {
    let field = parse_input(input);

    let mut high_score = 0;
    for row in 0..field.rows() {
        for col in 0..field.cols() {
            let mut visible_distances = HashMap::new();
            for direction in [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ] {
                let mut visible_trees = 0;
                let current_tree_height = field.get(row, col).height;

                let mut position = Position::new(row.try_into().unwrap(), col.try_into().unwrap());
                position = position.move_absolute(direction, 1);
                while field.valid_position(&position) {
                    let observed_tree = field.get(
                        position.row().try_into().unwrap(),
                        position.col().try_into().unwrap(),
                    );

                    visible_trees += 1;

                    if observed_tree.height >= current_tree_height {
                        break;
                    }
                    position = position.move_absolute(direction, 1);
                }
                visible_distances.insert(direction, visible_trees);
            }

            let score = visible_distances.values().product();
            if high_score < score {
                high_score = score;
            }
        }
    }

    high_score.to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "21";
    let expected_example_part2 = "8";

    println!("AOC 2022 {}", env!("CARGO_PKG_NAME"));
    println!("Sample Part 1:");
    let sample_result_part1 = part1(&example_input);
    if sample_result_part1 != expected_example_part1 {
        println!(
            "  Answer: {} (expected {})",
            sample_result_part1, expected_example_part1
        );
    } else {
        println!("  Answer: {} CORRECT!!!", sample_result_part1);
    }
    println!("Sample Part 2:");
    let sample_result_part2 = part2(&example_input);
    if sample_result_part2 != expected_example_part2 {
        println!(
            "  Answer: {} (expected {})",
            sample_result_part2, expected_example_part2
        );
    } else {
        println!("  Answer: {} CORRECT!!!", sample_result_part2);
    }
    println!("");

    let input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "input.txt");

    println!("Puzzle Part 1:");
    let result = part1(&input);
    println!("  Answer: {}", result);
    println!("Puzzle Part 2:");
    let result = part2(&input);
    println!("  Answer: {}", result);
    println!("");

    if sample_result_part1 == expected_example_part1 {
        println!("EXAMPLE PART 1 CORRECT");
    } else {
        println!("EXAMPLE PART 1 FAILED");
    }

    if sample_result_part2 == expected_example_part2 {
        println!("EXAMPLE PART 2 CORRECT");
    } else {
        println!("EXAMPLE PART 2 FAILED");
    }
}
