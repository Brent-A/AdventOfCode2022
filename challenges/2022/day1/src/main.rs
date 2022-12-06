use std::collections::HashMap;

fn main() {
    let input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "input.txt");
    
    let mut elf_index = 0;
    let mut calories = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            elf_index = elf_index + 1;
        } else {
            let value : u32 = line.parse().unwrap();
            if !calories.contains_key(&elf_index) {
                calories.insert(elf_index, 0);
            }

            let total = calories.get_mut(&elf_index).unwrap();
            *total = *total + value;
        }
    }

    println!("Part 1:");

    let max_calories = calories.values().max().unwrap();
    println!("max calories: {}", max_calories);

    println!("Part 2:");

    let mut sorted : Vec<_> = calories.values().collect();
    sorted.sort();

    let top_three : Vec<_> = sorted.iter().rev().take(3).collect();
    println!(" top_three: {:?}", top_three);
    let top_three_total : u32 = top_three.iter().map(|v| ***v).sum();
    println!(" sum: {}", top_three_total);
}
