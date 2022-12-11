use std::collections::{HashMap, HashSet};

use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

// scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize)

fn part1(input: &str) -> String {
    for i in 4..input.len() {
        let sop = &input[i - 4..i];
        let chars: HashSet<char> = sop.chars().collect();
        if chars.len() == 4 {
            return i.to_string();
        }
    }
    "".to_string()
}

fn part2(input: &str) -> String {
    for i in 14..input.len() {
        let sop = &input[i - 14..i];
        let chars: HashSet<char> = sop.chars().collect();
        if chars.len() == 14 {
            return i.to_string();
        }
    }
    "".to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "7";
    let expected_example_part2 = "19";

    println!("AOC 2022 Day 5");
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
