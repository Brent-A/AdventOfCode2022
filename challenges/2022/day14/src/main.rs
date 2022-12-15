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
use nom::{
    branch::alt,
    bytes::complete::take_while,
    character::streaming::one_of,
    combinator::{cut, map, map_res, recognize},
    error::context,
    multi::{many0, many1, separated_list0},
    sequence::{preceded, terminated},
    IResult,
};
use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

type C = XY<Unit, true, false>;

type Unit = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Rock,
    Air,
    Sand,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Air
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Rock => f.write_str("#"),
            Tile::Air => f.write_str("."),
            Tile::Sand => f.write_str("o"),
        }
    }
}

// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9
fn parse_input(input: &str) -> Grid<Tile, C> {
    let mut map = Grid::new();
    for line in input.lines() {
        let mut corners = Vec::new();
        for cstr in line.split(" -> ") {
            let (x, y) = scan_fmt!(cstr, "{},{}", Unit, Unit).unwrap();
            let c = C::new(x, y);
            corners.push(c);
        }

        for i in 1..corners.len() {
            let start = &corners[i - 1];
            let end = &corners[i];

            let r = RectangularRange::from_points(&[start.clone(), end.clone()]);

            for c in r.iter() {
                *map.get_mut_or_default(&c) = Tile::Rock;
            }
        }
    }
    map
}

fn part1(input: &str) -> String {
    let mut map = parse_input(input);

    map.print(|_c, t| t.cloned().unwrap_or_default().to_string());

    let sand_origin = C::new(500, 0);

    let mut resting_sand = 0;

    // Each new sand
    'each_sand: loop {
        let mut sand = sand_origin.clone();

        // Each movement of the sand
        'each_movement: loop {
            /*
                        println!("");
                        map.print(|c, t| {
                            if c == sand {
                                "+".to_string()
                            } else {
                                t.cloned().unwrap_or_default().to_string()
                            }
                        });
            */

            let fall_options = [sand.down1(), sand.down1().left1(), sand.down1().right1()];

            let mut moved = false;
            for next in fall_options {
                if Tile::Air == map.get(&next).cloned().unwrap_or_default() {
                    sand = next;
                    moved = true;
                    break;
                }
            }

            if !moved {
                *map.get_mut_or_default(&sand) = Tile::Sand;
                resting_sand += 1;
                break 'each_movement;
            }

            // Check for the edge
            if map.range().vertical().bottom().unwrap() < sand.vertical() {
                break 'each_sand;
            }
        }
    }

    println!("");
    map.print(|_c, t| t.cloned().unwrap_or_default().to_string());

    resting_sand.to_string()
}

fn part2(input: &str) -> String {
    let mut map = parse_input(input);

    map.print(|_c, t| t.cloned().unwrap_or_default().to_string());

    let sand_origin = C::new(500, 0);

    let mut resting_sand = 0;

    let floor = map.range().bottom_left().unwrap().vertical().clone() + 2;

    // Each new sand
    'each_sand: loop {
        let mut sand = sand_origin.clone();

        if let Some(Tile::Sand) = map.get(&sand).cloned() {
            break;
        }

        // Each movement of the sand
        'each_movement: loop {
            /*
                        println!("");
                        map.print(|c, t| {
                            if c == sand {
                                "+".to_string()
                            } else {
                                t.cloned().unwrap_or_default().to_string()
                            }
                        });
            */

            let fall_options = [
                sand.project(Direction::Down, 1),
                sand.project(Direction::Down, 1).project(Direction::Left, 1),
                sand.project(Direction::Down, 1)
                    .project(Direction::Right, 1),
            ];

            let mut moved = false;
            for next in fall_options {
                if *next.vertical() >= floor {
                    break;
                }
                if Tile::Air == map.get(&next).cloned().unwrap_or_default() {
                    sand = next;
                    moved = true;
                    break;
                }
            }

            if !moved {
                *map.get_mut_or_default(&sand) = Tile::Sand;
                resting_sand += 1;
                break 'each_movement;
            }

            // Check for the edge
            //if map.range().vertical().bottom().unwrap() < sand.vertical() {
            //    break 'each_sand;
            //}
        }
    }

    println!("");
    map.print(|_c, t| t.cloned().unwrap_or_default().to_string());

    resting_sand.to_string()
}

fn main() {
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

    let run_input = true;
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
