fn part1(input: &str) -> i32
{
    let mut last_depth = None;
    let mut increases = 0;
    for line in input.lines() {
        let depth : u32 = line.parse().unwrap();
        if let Some(last) = &last_depth {
            if depth > *last {
                increases += 1;
            }
        }
        last_depth = Some(depth);
    }
    increases
}

fn part2(input: &str) -> i32
{
    let mut last_window = None;
    let mut last_depths = Vec::new();
    let mut increases = 0;
    for line in input.lines() {
        let depth : u32 = line.parse().unwrap();
        last_depths.push(depth);
        if last_depths.len() > 3 {
            last_depths.remove(0);
        }

        if last_depths.len() == 3 {

            let window : u32 = last_depths.iter().sum();

            if let Some(last) = &last_window {
                if window > *last {
                    increases += 1;
                }
            }

            last_window = Some(window);
        }
    }
    increases
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");
    
    let expected_example_part1 = 7;
    let expected_example_part2 = 5;

    println!("AOC 2021 Day 1");
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
