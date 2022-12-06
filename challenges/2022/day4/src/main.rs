use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {}

#[derive(Debug, Deserialize)]
struct Pair {
    first: Range,
    second: Range,
}

impl Pair {
    pub fn complete_overlapped(&self) -> bool {
        if (self.first.start >= self.second.start && self.first.end <= self.second.end)
            || (self.second.start >= self.first.start && self.second.end <= self.first.end)
        {
            true
        } else {
            false
        }
    }

    pub fn any_overlapped(&self) -> bool {
        if (self.first.start >= self.second.start && self.first.start <= self.second.end)
            || (self.first.end >= self.second.start && self.first.end <= self.second.end)
            || (self.first.start < self.second.start && self.first.end > self.second.end)
        {
            true
        } else {
            false
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut completely_overlapped = 0;
    for line in input.lines() {
        let (a, b, c, d) = scan_fmt!(line, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap();
        let pair = Pair {
            first: Range { start: a, end: b },
            second: Range { start: c, end: d },
        };
        if pair.complete_overlapped() {
            completely_overlapped += 1;
        }
    }
    completely_overlapped
}

fn part2(input: &str) -> i32 {
    let mut any_overlapped = 0;
    for line in input.lines() {
        let (a, b, c, d) = scan_fmt!(line, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap();
        let pair = Pair {
            first: Range { start: a, end: b },
            second: Range { start: c, end: d },
        };
        if pair.any_overlapped() {
            any_overlapped += 1;
        }
    }
    any_overlapped
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = 2;
    let expected_example_part2 = 4;

    println!("AOC 2022 Day 4");
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
