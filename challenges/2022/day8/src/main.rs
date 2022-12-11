use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

use aoc::{
    coordinate::{Coordinate, RowCol},
    grid::Grid,
    position::{Direction, Position, EACH_DIRECTION},
};
use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

#[derive(Debug)]
enum Visibility {
    Unknown,
    Visible,
}

#[derive(Debug)]
struct Tree {
    height: u32,
    visible: Visibility,
}

type Field = Grid<Tree, RowCol>;

trait FieldTrait {
    fn total_visible(&self) -> usize;
}

impl FieldTrait for Field {
    fn total_visible(&self) -> usize {
        self.tiles()
            .map(|t| match t.visible {
                Visibility::Unknown => 0,
                Visibility::Visible => 1,
            })
            .sum()
    }
}

fn parse_input(input: &str) -> Field {
    let mut field = Grid::new();

    for (row_index, line) in input.lines().enumerate() {
        for (col_index, char) in line.chars().enumerate() {
            let height = char as u32 - '0' as u32;
            field.insert(
                RowCol::new(row_index as i32, col_index as i32),
                Tree {
                    height,
                    visible: Visibility::Unknown,
                },
            );
        }
    }
    field
}

fn part1(input: &str) -> String {
    let mut field = parse_input(input);

    for direction in EACH_DIRECTION {
        for mut position in field.range().edge_positions(direction.opposite()) {
            //println!("Position: {:?} direction {:?}", position, direction);
            let mut highest = field.get(&position).unwrap().height;
            field.get_mut(&position).unwrap().visible = Visibility::Visible;

            position = position.project(direction, 1);
            while field.range().contains(&position) {
                let tree = field.get_mut(&position).unwrap();
                if tree.height > highest {
                    tree.visible = Visibility::Visible;
                    highest = tree.height;
                }
                position = position.project(direction, 1);
            }
        }
    }
    field.total_visible().to_string()
}

fn part2(input: &str) -> String {
    let field = parse_input(input);

    let mut high_score = 0;
    for position in field.range().iter() {
        let mut visible_distances = HashMap::new();
        for direction in EACH_DIRECTION {
            let mut visible_trees = 0;
            let current_tree_height = field.get(&position).unwrap().height;

            let mut position = position.clone();
            position = position.project(direction, 1);
            while field.range().contains(&position) {
                let observed_tree = field.get(&position).unwrap();

                visible_trees += 1;

                if observed_tree.height >= current_tree_height {
                    break;
                }
                position = position.project(direction, 1);
            }
            visible_distances.insert(direction, visible_trees);
        }

        let score = visible_distances.values().product();
        if high_score < score {
            high_score = score;
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
