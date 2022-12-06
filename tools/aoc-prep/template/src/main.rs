use std::{collections::HashSet, hash::Hash};


fn part1(input: &str) -> i32
{
    0
}

fn part2(input: &str) -> i32
{
    0
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");
    
    let expected_example_part1 = -1;
    let expected_example_part2 = -1;

    println!("AOC 2022 Day 4");
    println!("Sample Part 1:");
    let sample_result_part1 = part1(&example_input);
    if (sample_result_part1 != expected_example_part1)
    {
        println!("  Answer: {} (expected {})", sample_result_part1, expected_example_part1);
    }
    else {
        println!("  Answer: {} CORRECT!!!", sample_result_part1);
    }
    println!("Sample Part 2:");
    let sample_result_part2 = part2(&example_input);
    if (sample_result_part2 != expected_example_part2)
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
