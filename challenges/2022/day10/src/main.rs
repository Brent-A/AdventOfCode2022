use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    fmt::Display,
    rc::{Rc, Weak},
};

use aoc::{
    coordinate::{Coordinate, RowCol},
    grid::Grid,
    position::{Direction, Position, EACH_DIRECTION},
};
use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

enum OpCode {
    AddX(i32),
    Noop,
}

impl OpCode {
    pub fn from_str(s: &str) -> Option<OpCode> {
        if let Ok(i) = scan_fmt!(s, "addx {d}", i32) {
            Some(OpCode::AddX(i))
        } else if s == "noop" {
            Some(OpCode::Noop)
        } else {
            None
        }
    }

    pub fn cycles(&self) -> i32 {
        match self {
            OpCode::AddX(_) => 2,
            OpCode::Noop => 1,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::AddX(i) => f.write_fmt(format_args!("addx {}", i)),
            OpCode::Noop => f.write_fmt(format_args!("noop")),
        }
    }
}

fn parse_input(input: &str) -> Vec<OpCode> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(OpCode::from_str(line).unwrap());
    }
    instructions
}

fn part1(input: &str) -> String {
    let input = parse_input(input);

    let mut cycle_count = 1;
    let mut X = 1;

    let mut signal = Vec::new();

    for i in input {
        for _ in 0..i.cycles() {
            let strength = X * cycle_count;
            signal.push(strength);
            cycle_count += 1;
        }

        match i {
            OpCode::AddX(i) => X += i,
            OpCode::Noop => {}
        }
    }
    let sample_points = [20, 60, 100, 140, 180, 220];

    let mut sum = 0;
    for point in sample_points {
        sum += signal[point - 1];
    }
    sum.to_string()
}

struct Sprite {
    characters: String,
    start_offset: i32,
}

impl Sprite {
    pub fn new(characters: String, start_offset: i32) -> Self {
        Self {
            characters,
            start_offset,
        }
    }

    pub fn get_character(&self, offset: i32) -> Option<char> {
        let index = offset + self.start_offset;
        if index < 0 || index >= self.characters.len() as i32 {
            None
        } else {
            Some(self.characters.chars().nth(index as usize).unwrap())
        }
    }
}
fn part2(input: &str) -> String {
    let input = parse_input(input);

    let mut cycle_count = 1;
    let mut X = 1;

    let sprite = Sprite::new("###".to_string(), 1);

    let mut output = Grid::new();
    for i in input {
        for _ in 0..i.cycles() {
            let row = (cycle_count - 1) / 40;
            let col = (cycle_count - 1) % 40;

            let ch = sprite.get_character(X - col).unwrap_or('.');
            //println!("{} {},{} = {}", cycle_count, row, col, ch);

            *output.get_mut_or_default(&RowCol::new(row, col)) = ch;

            cycle_count += 1;
        }

        match i {
            OpCode::AddX(i) => X += i,
            OpCode::Noop => {}
        }
    }

    output.print(|_c, t| t.unwrap().to_string());
    "".to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "13140";
    let expected_example_part2 = "?";

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
