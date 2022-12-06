use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

trait GetOrDefault<T>
where
    T: Default,
{
    fn get_or_default(&mut self, index: usize) -> &T;
    fn get_mut_or_default(&mut self, index: usize) -> &mut T;
}

impl<T> GetOrDefault<T> for Vec<T>
where
    T: Default,
{
    fn get_or_default(&mut self, index: usize) -> &T {
        while self.len() < index + 1 {
            self.push(T::default());
        }
        self.get(index).unwrap()
    }
    fn get_mut_or_default(&mut self, index: usize) -> &mut T {
        while self.len() < index + 1 {
            self.push(T::default());
        }
        self.get_mut(index).unwrap()
    }
}

fn part1(input: &str) -> String {
    let mut m = HashMap::new();

    let mut bitcount: Vec<usize> = Vec::new();
    let mut count = 0;

    for line in input.lines() {
        for (i, bit) in line.chars().enumerate() {
            let c = bitcount.get_mut_or_default(i);

            if bit == '1' {
                *c += 1;
            }
        }
        count += 1;
    }

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    for bit in bitcount {
        if bit > count / 2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_val = u32::from_str_radix(&gamma, 2).unwrap();
    let epsion_val = u32::from_str_radix(&epsilon, 2).unwrap();

    (gamma_val * epsion_val).to_string()
}

fn part2(input: &str) -> String {
    "".to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "198";
    let expected_example_part2 = "?";

    println!("AOC 2021 Day 3");
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
