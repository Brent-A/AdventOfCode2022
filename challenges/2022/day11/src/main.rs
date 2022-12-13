use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    rc::{Rc, Weak}, borrow::Borrow,
};

use aoc::{
    coordinate::{Coordinate, RowCol},
    grid::Grid,
    position::{Direction, Position, EACH_DIRECTION},
};
use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

#[derive(Debug)]
enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Constant(i32),
    Variable(String),
}

impl Expression {
    pub fn parse(str: &str) -> Expression {
        let trimmed = str.trim();
        if let Ok((a, b)) = scan_fmt!(trimmed, "{} + {}", String, String) {
            Expression::Add(
                Box::new(Expression::parse(&a)),
                Box::new(Expression::parse(&b)),
            )
        } else if let Ok((a, b)) = scan_fmt!(trimmed, "{} * {}", String, String) {
            Expression::Multiply(
                Box::new(Expression::parse(&a)),
                Box::new(Expression::parse(&b)),
            )
        } else if let Ok(a) = scan_fmt!(trimmed, "{d}", i32) {
            Expression::Constant(a)
        } else {
            Expression::Variable(trimmed.to_string())
        }
    }
    pub fn evaluate(&self, variables: &HashMap<String, i32>) -> i32 {
        match self {
            Expression::Add(a, b) => a.evaluate(variables) + b.evaluate(variables),
            Expression::Multiply(a, b) => a.evaluate(variables) * b.evaluate(variables),
            Expression::Constant(c) => c.clone(),
            Expression::Variable(v) => variables.get(v).unwrap().clone(),
        }
    }
}

#[derive(Debug)]
enum Test {
    DivisibleBy(i32),
}

impl Test {
    pub fn parse(str: &str) -> Test {
        if let Ok(v) = scan_fmt!(str, "divisible by {}", i32) {
            Test::DivisibleBy(v)
        } else {
            panic!("Unexpected test value");
        }
    }

    pub fn evaluate(&self, value: i32) -> bool {
        match self {
            Test::DivisibleBy(v) => value % *v == 0,
        }
    }
}

#[derive(Debug)]
enum Action {
    ThrowTo(i32),
}

impl Action {
    pub fn parse(str: &str) -> Action {
        if let Ok(v) = scan_fmt!(str, "throw to monkey {}", i32) {
            Action::ThrowTo(v)
        } else {
            panic!("Unexpected action value: {}", str);
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i32>,
    operation: Expression,
    test: Test,
    true_action: Action,
    false_action: Action,
    inspection_count: i32,
}


fn parse_input(input: &str) -> Vec<RefCell<Monkey>> {
    let mut monkeys = Vec::new();

    let mut line_iterator = input.lines();

    while let Some(monkey_line) = line_iterator.next() {
        let monkey_index = scan_fmt!(monkey_line, "Monkey {}:", i32).unwrap();
        assert!(monkey_index as usize == monkeys.len());
        let items_str = scan_fmt!(
            line_iterator.next().unwrap(),
            "  Starting items: {[0-9, ]}",
            String
        )
        .unwrap();
        
        let items = items_str
            .split(",")
            .map(|i| i.trim().parse::<i32>().unwrap())
            .collect();
        let operation_str = scan_fmt!(
            line_iterator.next().unwrap(),
            "Operation: new = {[a-zA-Z0-9+* ]}{e}",
            String
        )
        .unwrap();
        let operation = Expression::parse(&operation_str);
        let test_str = scan_fmt!(line_iterator.next().unwrap(), "Test: {[a-z0-9 ]}{e}", String).unwrap();
        let test = Test::parse(&test_str);
        let true_action_str =
            scan_fmt!(line_iterator.next().unwrap(), "If true: {[a-zA-Z0-9 ]}{e}", String).unwrap();
        let true_action = Action::parse(&true_action_str);
        let false_action_str =
            scan_fmt!(line_iterator.next().unwrap(), "If false: {[a-zA-Z0-9 ]}{e}", String).unwrap();
        let false_action = Action::parse(&false_action_str);

        monkeys.push(RefCell::new(Monkey {
            items,
            operation,
            test,
            true_action,
            false_action,
            inspection_count: 0,
        }));

        line_iterator.next();
    }
    monkeys
}

fn part1(input: &str) -> String {
    let mut input = parse_input(input);

    println!("Monkeys: {input:?}");

    for round in 0..20 {
        for (index, monkey) in input.iter().enumerate() {
            let mut current_monkey = monkey.borrow_mut();

            while let Some(item) = current_monkey.items.pop_front() {
                let mut variables = HashMap::new();
                variables.insert("old".to_string(), item);

                // Inspect
                let new = current_monkey.operation.evaluate(&variables);
                let post_inspection = new / 3;
                current_monkey.inspection_count += 1;

                // Test
                let action = if current_monkey.test.evaluate(post_inspection) {
                    &current_monkey.true_action
                } else {
                    &current_monkey.false_action
                };

                // Act
                match action {
                    Action::ThrowTo(target) => {
                        let mut target_monkey = input[*target as usize].borrow_mut();
                        target_monkey.items.push_back(post_inspection);

                        if round == 0 {
                            println!("Monkey {index}: {item} became {new} and then {post_inspection}, thrown to {target}")
                        }
                    }
                }
            }
        }

        println!("Round {}", round + 1);
        for (i, monkey) in input.iter().enumerate() {
            println!("Monkey {}: {:?}", i, monkey.borrow().items);
        }
    }
    let mut top_count :Vec<_> = input.iter().map(|m| m.borrow().inspection_count).collect();
    top_count.sort();
    println!("top_count {top_count:?}");
    let monkey_business : i32 = top_count.iter().rev().take(2).product();
    monkey_business.to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);

    "".to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "10605";
    let expected_example_part2 = "?";

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
