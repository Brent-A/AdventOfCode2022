use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

struct Directory {
    name: String,
    children: RefCell<HashMap<String, Node>>,
    parent: Option<Weak<Directory>>,
}

struct File {
    name: String,
    size: usize,
    parent: Weak<Directory>,
}

#[derive(Clone)]
enum Node {
    File(Rc<File>),
    Directory(Rc<Directory>),
}

impl Directory {
    pub fn new(name: String, parent: Option<&Rc<Directory>>) -> Rc<Self> {
        Rc::new(Self {
            name,
            children: RefCell::new(HashMap::new()),
            parent: parent.map(|p| Rc::downgrade(p)),
        })
    }

    pub fn short_name(self: &Rc<Directory>) -> String {
        self.name.clone()
    }

    pub fn full_name(self: &Rc<Directory>) -> String {
        let mut segments = Vec::new();
        let mut parent = self.parent();

        segments.push(self.short_name());
        while let Some(n) = parent {
            segments.push(n.short_name());
            parent = n.parent();
        }

        segments.reverse();

        segments.join("/")
    }

    pub fn total_size(self: &Rc<Directory>) -> usize {
        let mut size = 0;
        for child in self.children.borrow().values() {
            match child {
                Node::File(f) => size += f.size,
                Node::Directory(d) => size += d.total_size(),
            }
        }
        size
    }

    pub fn add_file(self: &Rc<Directory>, name: String, size: usize) -> Rc<File> {
        if let Some(Node::File(f)) = self.children.borrow().get(&name) {
            return f.clone();
        }

        let file = Rc::new(File {
            name: name.clone(),
            size,
            parent: Rc::downgrade(self),
        });

        self.children
            .borrow_mut()
            .insert(name, Node::File(file.clone()));

        file
    }

    pub fn add_directory(self: &Rc<Directory>, name: String) -> Rc<Directory> {
        if let Some(Node::Directory(f)) = self.children.borrow().get(&name) {
            return f.clone();
        }

        let file = Directory::new(name.clone(), Some(self));

        self.children
            .borrow_mut()
            .insert(name, Node::Directory(file.clone()));

        file
    }

    pub fn parent(self: &Rc<Directory>) -> Option<Rc<Directory>> {
        self.parent.as_ref().map(|p| p.upgrade().unwrap())
    }

    pub fn dir(self: &Rc<Directory>) -> Vec<Node> {
        self.children.borrow().values().cloned().collect()
    }

    pub fn flatten(self: &Rc<Directory>) -> Vec<Node> {
        let mut v = Vec::new();
        v.push(Node::Directory(self.clone()));

        for n in self.children.borrow().values() {
            if let Node::Directory(d) = n {
                v.append(&mut d.flatten());
            } else {
                v.push(n.clone())
            }
        }
        v
    }
}

// scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize)

fn get_tree(input: &str) -> Rc<Directory> {
    let root = Directory::new("".to_string(), None);

    let mut current = root.clone();

    for line in input.lines() {
        if line == "$ cd /" {
            current = root.clone();
        } else if line == "$ cd .." {
            current = current.parent().unwrap();
        } else if let Ok(path) = scan_fmt!(line, "$ cd {s}", String) {
            current = current.add_directory(path);
        } else if line == "$ ls" {
        } else if let Ok((size, name)) = scan_fmt!(line, "{d} {s}", usize, String) {
            current.add_file(name, size);
        } else if let Ok(dir) = scan_fmt!(line, "dir {s}", String) {
            current.add_directory(dir);
        } else {
            panic!("Unexpected input");
        }
    }

    root
}
fn part1(input: &str) -> String {
    let root = get_tree(input);

    let mut sum = 0;
    for n in root.flatten() {
        if let Node::Directory(d) = n {
            if d.total_size() <= 100000 {
                println!("{} = {}", d.full_name(), d.total_size());
                sum += d.total_size();
            }
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let root = get_tree(input);
    let total_disk = 70000000;
    let min_unused = 30000000;

    let must_free = min_unused - (total_disk - root.total_size());

    let mut smallest_dir = None;

    for n in root.flatten() {
        if let Node::Directory(d) = n {
            if d.total_size() >= must_free {
                println!("{} = {}", d.full_name(), d.total_size());
                if let Some(smallest) = smallest_dir {
                    if d.total_size() < smallest {
                        smallest_dir = Some(d.total_size());
                    }
                } else {
                    smallest_dir = Some(d.total_size());
                }
            }
        }
    }

    smallest_dir.unwrap().to_string()
}

fn main() {
    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "95437";
    let expected_example_part2 = "24933642";

    println!("AOC 2022 Day 7");
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
