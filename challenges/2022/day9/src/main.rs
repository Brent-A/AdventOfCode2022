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

struct Instruction {
    direction: Direction,
    count: i32,
}
fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        if let Ok((d, c)) = scan_fmt!(line, "{s} {d}", String, i32) {
            instructions.push(Instruction {
                direction: match d.as_str() {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!(),
                },
                count: c,
            });
        } else {
            panic!()
        }
    }
    instructions
}

fn part1(input: &str) -> String {
    let input = parse_input(input);

    let mut grid = Grid::new();
    let mut head = RowCol::new(0, 0);
    let mut tail = RowCol::new(0, 0);

    for instruction in input {
        for _ in 0..instruction.count {
            head = head.project(instruction.direction, 1);

            let ((hdistance, hdirection), (vdistance, vdirection)) = (tail.horizontal_relative_to(&head), tail.vertical_relative_to(&head));

            if hdistance > 1 || vdistance > 1 {
                tail = tail.project(hdirection.opposite(), 1);
                tail = tail.project(vdirection.opposite(), 1);
            }

            
            grid.get_mut_or_default(&head);

            *grid.get_mut_or_default(&tail) = true;

            /* 
            println!("{:?} {}", instruction.direction, instruction.count);
            grid.print(|c, t| {
                
                if c == tail && c == head {
                    "X".to_string()
                } else
                if c == tail {
                    "T".to_string()
                } else if c == head {
                    "H".to_string()
                } else if let Some(true) = t {
                    "#".to_string()
                } else {
                    ".".to_string()
                }
            });
            println!("");
            */
        }
    }

    grid.tiles().filter(|t| **t == true).count().to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);

    let mut grid = Grid::new();
    let mut rope = [RowCol::new(0, 0); 10];
    //let mut head = RowCol::new(0, 0);
    //let mut tail = RowCol::new(0, 0);

    for instruction in input {
        for _ in 0..instruction.count {
            rope[0] = rope[0].project(instruction.direction, 1);

            for knot in 1..10 {
                let head = rope[knot-1].clone();
                let tail = &mut rope[knot];
                let ((hdistance, hdirection), (vdistance, vdirection)) = (tail.horizontal_relative_to(&head), tail.vertical_relative_to(&head));

                if hdistance > 1 || vdistance > 1 {
                    *tail = tail.project(hdirection.opposite(), 1);
                    *tail = tail.project(vdirection.opposite(), 1);
                }

                
                grid.get_mut_or_default(&head);
                grid.get_mut_or_default(tail);
            }

            *grid.get_mut_or_default(&rope[9]) = true;

            /* 
            println!("{:?} {}", instruction.direction, instruction.count);
            grid.print(|c, t| {
                
                if c == tail && c == head {
                    "X".to_string()
                } else
                if c == tail {
                    "T".to_string()
                } else if c == head {
                    "H".to_string()
                } else if let Some(true) = t {
                    "#".to_string()
                } else {
                    ".".to_string()
                }
            });
            println!("");
            */
        }
    }

    grid.tiles().filter(|t| **t == true).count().to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "13";
    let expected_example_part2 = "1";

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
