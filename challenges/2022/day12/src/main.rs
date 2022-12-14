use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    ops::Sub,
    rc::{Rc, Weak},
    slice::SliceIndex,
};

use aoc::{
    coordinate::{Coordinate, RowCol},
    grid::Grid,
    position::{Direction, Position, EACH_DIRECTION},
};
use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
struct Height(char);

impl Height {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

impl Sub for Height {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as i32 - rhs.0 as i32
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn parse_input(input: &str) -> (RowCol, RowCol, Grid<Height, RowCol>) {
    let mut heights = Grid::new();

    let mut row = 0;

    let mut start = None;
    let mut end = None;
    for line in input.lines() {
        let mut col = 0;
        for c in line.chars() {
            let coordinate = RowCol::new(row, col);
            if c == 'S' {
                start = Some(coordinate.clone());
            }
            if c == 'E' {
                end = Some(coordinate.clone());
            }

            let h = if c == 'S' {
                Height::new('a')
            } else if c == 'E' {
                Height::new('z')
            } else {
                Height::new(c)
            };
            *heights.get_mut_or_default(&coordinate) = h;

            col += 1;
        }
        row += 1;
    }

    (start.unwrap(), end.unwrap(), heights)
}

fn fill_distances(
    heights: &Grid<Height, RowCol>,
    distances: &mut Grid<usize, RowCol>,
    position: RowCol,
    distance: usize,
) {
    if let Some(existing_distance) = distances.get(&position) {
        if *existing_distance <= distance {
            return;
        }
    }

    *distances.get_mut_or_default(&position) = distance;

    let current_height = heights.get(&position).unwrap();

    for d in EACH_DIRECTION {
        let next = position.project(d, 1);
        if heights.range().contains(&next) {
            let next_height = heights.get(&next).unwrap();

            let step_up = *current_height - *next_height;
            if step_up <= 1 {
                fill_distances(heights, distances, next, distance + 1);
            }
        }
    }
}
fn part1(input: &str) -> String {
    let (start, end, heights) = parse_input(input);

    let mut distances = Grid::new();

    fill_distances(&heights, &mut distances, end, 0);

    distances.get(&start).unwrap().to_string()
}

fn part2(input: &str) -> String {
    let (start, end, heights) = parse_input(input);

    let mut distances = Grid::new();

    fill_distances(&heights, &mut distances, end, 0);

    let mut best = None;
    for (c, t) in heights.enumerate_tiles() {
        if *t.unwrap() == Height::new('a') {
            if let Some(d) = distances.get(&c) {
                if let Some(best_distance) = best {
                    if d < best_distance {
                        best = Some(d)
                    }
                } else {
                    best = Some(d);
                }
            }
        }
    }

    best.unwrap().to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "31";
    let expected_example_part2 = "29";

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
