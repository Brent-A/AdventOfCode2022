#![feature(drain_filter)]
use std::collections::HashMap;

use aoc::GetOrDefault;
use scan_fmt::scan_fmt;
use serde_derive::Deserialize;


fn part1(input: &str) -> String {
    
    let mut bitcount: Vec<usize> = Vec::new();
    let mut count = 0;

    for line in input.lines() {
        for (i, bit) in line.chars().enumerate() {
            let c = bitcount.get_mut_or_default(i);

            if bit == '1' {
                *c += 1;
            }
        }
        count +=1;
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
    
    let mut bitcount: Vec<usize> = Vec::new();
    let mut count = 0;

    let mut values = Vec::new();

    for line in input.lines() {
        for (i, bit) in line.chars().enumerate() {
            let c = bitcount.get_mut_or_default(i);

            if bit == '1' {
                *c += 1;
            }
        }
        values.push(line.to_string());
        count +=1;
        
    }

    let mut o2values = values.clone();
    let mut co2values = values.clone();

    let mut index = 0;
    while o2values.len() > 1 {
        bitcount.clear();
        count = 0;

        for value in &o2values {
            for (i, bit) in value.chars().enumerate() {
                let c = bitcount.get_mut_or_default(i);

                if bit == '1' {
                    *c += 1;
                }
            }
            count = count + 1;
        }

        let bit = bitcount.iter().nth(index).unwrap();

        if *bit > count / 2 {
            o2values.drain_filter(|e| e.chars().nth(index).unwrap() == '0');
        } else {
            o2values.drain_filter(|e| e.chars().nth(index).unwrap() == '1');
        }

        println!("kept {}", o2values.len());
        index += 1;
    }

    
    println!("o2value: {}", o2values[0]);
    index = 0;
    while co2values.len() > 1 {
        bitcount.clear();
        count = 0;

        for value in &co2values {
            for (i, bit) in value.chars().enumerate() {
                let c = bitcount.get_mut_or_default(i);

                if bit == '1' {
                    *c += 1;
                }
            }
            count = count + 1;
        }

        
        let bit = bitcount.iter().nth(index).unwrap();

        if *bit <= count / 2 {
            co2values.drain_filter(|e| e.chars().nth(index).unwrap() == '0');
        } else {
            co2values.drain_filter(|e| e.chars().nth(index).unwrap() == '1');
        }

        println!("kept {:?}", co2values);
        index += 1;
    }

    println!("co2value: {}", co2values[0]);

    let gamma_val = u32::from_str_radix(&o2values[0], 2).unwrap();
    let epsion_val = u32::from_str_radix(&co2values[0], 2).unwrap();

    (gamma_val * epsion_val).to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "198";
    let expected_example_part2 = "230";

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
