use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}

struct Input {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

enum ParseStage {
    Stacks,
    StackIndex,
    Instructions,
}

fn parse_input(input: &str) -> Input {
    let mut stacks = Vec::new();
    let mut instructions = Vec::new();
    for line in input.lines() {
        if line.contains("[") {
            for i in 0..(1 + line.len() / 4) {
                let c = line.chars().nth(i * 4 + 1).unwrap();
                if stacks.len() <= i {
                    stacks.push(Vec::new());
                }
                if c != ' ' {
                    let stack = stacks.get_mut(i).unwrap();
                    stack.insert(0, c);
                }
            }
        } else if let Ok((count, source, dest)) =
            scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize)
        {
            instructions.push(Instruction {
                count,
                source,
                dest,
            });
        }
    }
    Input {
        stacks,
        instructions,
    }
}

fn part1(input: &str) -> String {
    let parsed_input = parse_input(input);
    let mut stacks = parsed_input.stacks;
    let instructions = parsed_input.instructions;

    for instruction in instructions {
        for _ in 0..instruction.count {
            let moved = stacks[instruction.source - 1].pop().unwrap();
            stacks[instruction.dest - 1].push(moved);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn part2(input: &str) -> String {
    let parsed_input = parse_input(input);
    let mut stacks = parsed_input.stacks;
    let instructions = parsed_input.instructions;

    for instruction in instructions {
        let mut moved = Vec::new();
        for _ in 0..instruction.count {
            moved.insert(0, stacks[instruction.source - 1].pop().unwrap());
        }

        stacks[instruction.dest - 1].append(&mut moved);
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "CMZ";
    let expected_example_part2 = "MCD";

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
