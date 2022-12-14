use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    ops::{Sub, Index},
    rc::{Rc, Weak},
    slice::SliceIndex,
};

use aoc::{
    coordinate::{Coordinate, RowCol},
    grid::Grid,
    position::{Direction, Position, EACH_DIRECTION},
};
use nom::{
    branch::alt,
    bytes::complete::take_while,
    character::streaming::one_of,
    combinator::{cut, map, map_res, recognize},
    error::context,
    multi::{many0, many1, separated_list0},
    sequence::{preceded, terminated},
    IResult,
};
use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

type Unit = i32;

#[derive(Clone, PartialEq, Eq)]
enum Element {
    List(Vec<Element>),
    Value(Unit),
}

fn sp<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
}

fn element_value(i: &str) -> IResult<&str, Unit> {
    map_res(preceded(sp, recognize(many1(one_of("0123456789")))), |v| {
        v.parse::<Unit>()
    })(i)
}

fn element_array(i: &str) -> IResult<&str, Vec<Element>> {
    context(
        "array",
        preceded(
            nom::character::complete::char('['),
            cut(terminated(
                separated_list0(preceded(sp, nom::character::complete::char(',')), element),
                preceded(sp, nom::character::complete::char(']')),
            )),
        ),
    )(i)
}

fn element(i: &str) -> IResult<&str, Element> {
    preceded(
        sp,
        alt((
            map(element_array, Element::List),
            map(element_value, Element::Value),
        )),
    )(i)
}

impl Element {
    fn parse(i: &str) -> Element {
        element(i).unwrap().1
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::List(l) => {
                f.write_str("[")?;
                let v: Vec<_> = l.iter().map(|e| e.to_string()).collect();
                f.write_str(&v.join(", "))?;
                f.write_str("]")
            }
            Element::Value(v) => v.fmt(f),
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (Element::List(l), Element::List(r)) => {
                let mut index = 0;
                loop {
                    if index < l.len() && index < r.len() {
                        let c = l[index].cmp(&r[index]);
                        if c != Ordering::Equal {
                            return Some(c);
                        }
                    } else {
                        break;
                    }
                    index = index + 1;
                }

                l.len().cmp(&r.len())
            }
            (Element::List(l), Element::Value(r)) =>  self.cmp(&Element::List(vec![other.clone()])),
            (Element::Value(l), Element::List(r)) => Element::List(vec![self.clone()]).cmp(other),
            (Element::Value(l), Element::Value(r)) => l.cmp(r),
        })
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Pair {
    left: Element,
    right: Element,
}

fn parse_input(input: &str) -> Vec<Pair> {
    let mut lines = input.lines();
    let mut pairs = Vec::new();

    loop {
        pairs.push(Pair {
            left: Element::parse(lines.next().unwrap()),
            right: Element::parse(lines.next().unwrap()),
        });
        if lines.next().is_none() {
            break;
        }
    }

    pairs
}

fn part1(input: &str) -> String {
    let input = parse_input(input);

    let mut correct_indicies = Vec::new();
    for (index, pair) in input.iter().enumerate() {
        println!("left: {}", pair.left);
        println!("right: {}", pair.right);
        println!("sorted: {}", pair.left <= pair.right);
        println!("");
        if pair.left <= pair.right {
            correct_indicies.push(index + 1);
        }
        
    }
    println!("indicies: {correct_indicies:?}");
    correct_indicies.iter().sum::<usize>().to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);

    let mut packets : Vec<_> = input.iter().flat_map(|p| [p.left.clone(), p.right.clone()]).collect();

    let divider0 = Element::parse("[[2]]"); 
    let divider1 = Element::parse("[[6]]"); 
    packets.push(divider0.clone());
    packets.push(divider1.clone());

    packets.sort();

    let divider0_position = packets.iter().enumerate().find_map(|(i, e)| if *e == divider0 { Some(i) } else { None }).unwrap();
    let divider1_position = packets.iter().enumerate().find_map(|(i, e)| if *e == divider1 { Some(i) } else { None }).unwrap();

    ((divider0_position + 1) * (divider1_position + 1)).to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "13";
    let expected_example_part2 = "140";

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
