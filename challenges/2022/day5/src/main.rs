use scan_fmt::scan_fmt;
use serde_derive::Deserialize;



fn part1(stacks_raw: &[&str], input: &str) -> String
{

    let mut stacks : Vec<Vec<char>> = stacks_raw.iter().map(|r| r.chars().rev().collect::<Vec<char>>()).collect();
    for line in input.lines() {

        if let Ok((count, source, dest) )= scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize) {

            for _ in 0..count {
                
                let moved = stacks[source - 1].pop().unwrap();
                stacks[dest - 1].push(moved);
            }
        }

    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn part2(stacks_raw: &[&str], input: &str) -> String
{
    
    let mut stacks : Vec<Vec<char>> = stacks_raw.iter().map(|r| r.chars().rev().collect::<Vec<char>>()).collect();
    for line in input.lines() {

        if let Ok((count, source, dest) )= scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize) {

            let mut moved = Vec::new();
            for _ in 0..count {
                
                
                moved.insert(0, stacks[source - 1].pop().unwrap());
                
            }

            stacks[dest - 1].append(&mut moved)
        }

    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");
    let stacks = [
        "NZ",
        "DCM",
        "P",
    ];

    let expected_example_part1 = "CMZ";
    let expected_example_part2 = "MCD";

    println!("AOC 2022 Day 5");
    println!("Sample Part 1:");
    let sample_result_part1 = part1(&stacks, &example_input);
    if sample_result_part1 != expected_example_part1
    {
        println!("  Answer: {} (expected {})", sample_result_part1, expected_example_part1);
    }
    else {
        println!("  Answer: {} CORRECT!!!", sample_result_part1);
    }
    println!("Sample Part 2:");
    let sample_result_part2 = part2(&stacks, &example_input);
    if sample_result_part2 != expected_example_part2
    {
        println!("  Answer: {} (expected {})", sample_result_part2, expected_example_part2);
    }
    else {
        println!("  Answer: {} CORRECT!!!", sample_result_part2);
    }println!("");

    let input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "input.txt");
    let stacks = [
        "FGVRJLD",
        "SJHVBMPT",
        "CPGDFMHV",
        "QGNPDM",
        "FNHLJ",
        "ZTGDQVFN",
        "LBDF",
        "NDVSBJM",
        "DLG",
    ];

    println!("Puzzle Part 1:");
    let result = part1(&stacks, &input);
    println!("  Answer: {}", result);
    println!("Puzzle Part 2:");
    let result = part2(&stacks, &input);
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
