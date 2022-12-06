use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    character: char,
}

impl Item {
    pub fn new(character: char) -> Self {
        Self { character }
    }

    pub fn priority(&self) -> u32 {
        if self.character >= 'a' && self.character <= 'z' {
            1 + (self.character as u32 - 'a' as u32)
        } else {
            27 + (self.character as u32 - 'A' as u32)
        }
    }
}

pub struct Rucksack {
    compartments: [Vec<Item>; 2],
}

impl Rucksack {
    pub fn compartment_from_str(string: &str) -> Vec<Item> {
        string.chars().map(|c| Item::new(c)).collect()
    }
    pub fn from_str(string: &str) -> Self {
        assert!(string.len() % 2 == 0);
        Self {
            compartments: [
                Self::compartment_from_str(&string[0..string.len() / 2]),
                Self::compartment_from_str(&string[string.len() / 2..]),
            ],
        }
    }
    pub fn items_in_both(&self) -> HashSet<Item> {
        self.compartments[0]
            .iter()
            .filter_map(|e| {
                if self.compartments[1].contains(e) {
                    Some(e.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn all_item_types(&self) -> HashSet<Item> {
        self.compartments[0]
            .iter()
            .chain(self.compartments[1].iter())
            .cloned()
            .collect()
    }
}

fn part1(input: &str) {
    println!("Part1:");
    let mut priorities = 0;
    for line in input.lines() {
        let r = Rucksack::from_str(line);
        let common_items = r.items_in_both();
        assert_eq!(common_items.len(), 1);
        let common_item = common_items.iter().next().unwrap();

        priorities += common_item.priority();
    }
    println!(" Total: {}", priorities);
}

fn part2(input: &str) {
    println!("Part2:");
    let mut iter = input.lines();
    let mut priorities = 0;
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        let rucksacks = (
            Rucksack::from_str(first),
            Rucksack::from_str(second),
            Rucksack::from_str(third),
        );

        let s0: HashSet<Item> = rucksacks
            .0
            .all_item_types()
            .intersection(&rucksacks.1.all_item_types())
            .cloned()
            .collect();
        let s1: HashSet<Item> = s0
            .intersection(&rucksacks.2.all_item_types())
            .cloned()
            .collect();

        assert_eq!(1, s1.len());

        let badge = s1.iter().next().unwrap();

        priorities += badge.priority();
    }
    println!(" Total: {}", priorities);
}

fn main() {
    let input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");
    println!("Sample:");
    part1(&input);
    part2(&input);
    let input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "input.txt");
    println!("Challenge:");
    part1(&input);
    part2(&input);
}
