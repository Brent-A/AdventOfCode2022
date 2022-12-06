#![feature(mixed_integer_ops)]

use std::{collections::HashSet, hash::Hash};

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct SubPosition {
    x: u32,
    aim: i32,
    depth: u32,
}

impl SubPosition {
    pub fn new() -> Self {
        Self { x: 0, aim: 0, depth: 0 }
    }

    pub fn move_by(&self, vector: &SubDirectionVector) -> Self {
        match vector.direction {
            SubDirection::Forward => Self { x: self.x + vector.count, aim: self.aim, depth: self.depth },
            SubDirection::Down => Self { x: self.x, aim: self.aim, depth: self.depth + vector.count },
            SubDirection::Up => Self { x: self.x, aim: self.aim, depth: self.depth - vector.count },
        }
    }

    pub fn move_by_aim(&self, vector: &SubDirectionVector) -> Self {
        match vector.direction {
            SubDirection::Forward => Self { x: self.x + vector.count, aim: self.aim, depth: self.depth.checked_add_signed(self.aim * vector.count as i32).unwrap()},
            SubDirection::Down => Self { x: self.x, aim: self.aim + vector.count as i32, depth: self.depth},
            SubDirection::Up => Self { x: self.x, aim: self.aim - vector.count as i32, depth: self.depth },
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum SubDirection {
    Forward,
    Down,
    Up
}


#[derive(Debug, Deserialize)]
struct SubDirectionVector {
    direction : SubDirection,
    count: u32,
}

fn part1(input: &str) -> u32
{
    let mut position = SubPosition::new();

    for line in input.lines() {
        let vector : SubDirectionVector = serde_scan::from_str(line).unwrap();
        position = position.move_by(&vector);
    }
    
    position.x * position.depth
}

fn part2(input: &str) -> u32
{
    let mut position = SubPosition::new();

    for line in input.lines() {
        let vector : SubDirectionVector = serde_scan::from_str(line).unwrap();
        position = position.move_by_aim(&vector);
    }
    
    position.x * position.depth
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");
    
    let expected_example_part1 = 150;
    let expected_example_part2 = 900;

    println!("AOC 2021 Day 2");
    println!("Sample Part 1:");
    let sample_result_part1 = part1(&example_input);
    if sample_result_part1 != expected_example_part1
    {
        println!("  Answer: {} (expected {})", sample_result_part1, expected_example_part1);
    }
    else {
        println!("  Answer: {} CORRECT!!!", sample_result_part1);
    }
    println!("Sample Part 2:");
    let sample_result_part2 = part2(&example_input);
    if sample_result_part2 != expected_example_part2
    {
        println!("  Answer: {} (expected {})", sample_result_part2, expected_example_part2);
    }
    else {
        println!("  Answer: {} CORRECT!!!", sample_result_part2);
    }println!("");

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
