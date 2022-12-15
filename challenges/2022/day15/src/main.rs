#![allow(dead_code, unused_variables, unused_imports)]

use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    ops::{Index, Sub},
    rc::{Rc, Weak},
    slice::SliceIndex,
};

use aoc::{
    coordinate::{Coordinate, HorizontalRange, RectangularRange, RowCol, VerticalRange, XY},
    grid::Grid,
    position::{Direction, Position, EACH_DIRECTION},
};

use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

type Unit = i32;
type C = XY<Unit, true, false>;

fn parse_input(input: &str) -> () {

}

fn part1(input: &str) -> String {
    let input = parse_input(input);
    "".to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);
    "".to_string()
}

fn main() {
    let run_input = true;

    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "24";
    let expected_example_part2 = "93";

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

    if run_input {
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
}
