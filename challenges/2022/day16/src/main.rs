#![allow(dead_code, unused_variables, unused_imports)]

use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque, BTreeSet},
    fmt::Display,
    hash::Hash,
    ops::{Index, Sub},
    rc::{Rc, Weak},
    slice::SliceIndex,
};

use aoc::{
    coordinate::{Coordinate, HorizontalRange, RectangularRange, RowCol, VerticalRange, XY},
    grid::Grid,
    position::{Direction, Position, EACH_DIRECTION},
    range::Range, GetOrDefault,
};

use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

type Unit = i32;
type C = XY<Unit, true, false>;

struct Valve {
    name: String,
    rate: Unit,
    tunnels_to: Vec<String>,

    distance_map: RefCell<HashMap<String, usize>>,
}

impl Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Valve {
    fn distance_to(&self, other: &str) -> usize {
        self.distance_map.borrow()[other]
    }
}

struct Maze {
    valves: HashMap<String, Valve>,
    min_valve_distance: i32,
}


struct State {
    position: String,
    time: Unit,
    opened_valves: BTreeSet<String>,
    score: Unit,

    next: RefCell<HashMap<Choice, Option<Rc<State>>>>,

    pruned_in_favor_of: RefCell<Option<Rc<State>>>,

    maze: Rc<Maze>,
    previous: Option<Weak<State>>,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        
        let mut sorted: Vec<_> = self.opened_valves.iter().collect();
        sorted.sort();
        sorted.hash(state);

        self.time.hash(state);
        self.score.hash(state);
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.opened_valves == other.opened_valves
            && self.time == other.time
            && self.score == other.score
    }
}

impl Eq for State{

}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Get all the ancestors of the state
        let mut ancestors = Vec::new();
        let mut current = self.previous.clone();
        while let Some(weak) = current {
            if let Some(strong) = weak.upgrade() {
                current = strong.previous.clone();
                ancestors.push(strong);
            } else {
                break;
            }
        }

        ancestors.reverse();
        let ancestor_strings: Vec<_> = ancestors.iter().map(|s| s.position.clone()).collect();

        write!(f, "{}", ancestor_strings.join("->"))?;
        write!(
            f,
            "->{},t={} (s={}) s~[{},{}] explored {}/{}",
            self.position,
            self.time,
            self.score,
            self.worst_possible_score(),
            self.theoretical_best_possible_score(),
            self.next.borrow().values().filter(|s| s.is_some()).count(),
            self.next.borrow().len(),
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Choice {
    MoveTo(String),
    //OpenValve,
}

impl State {
    fn new(maze: Rc<Maze>) -> Self {
        Self {
            position: "AA".to_string(),
            time: 0,
            opened_valves: BTreeSet::new(),
            score: 0,
            maze: maze.clone(),
            previous: None,
            next: RefCell::new(
                Self::choices("AA", 0, &BTreeSet::new(), &maze)
                    .iter()
                    .map(|c| (c.clone(), None))
                    .collect(),
            ),
            pruned_in_favor_of: RefCell::new(None),
        }
    }

    fn is_strictly_better_than(&self, other: &Rc<Self>) -> bool {
        if other.as_ref() == self {
            return false;
        }

        if self.worst_possible_score() >= other.theoretical_best_possible_score() {
            return true;
        }

        if self.position == other.position
            && self.opened_valves == other.opened_valves
            && self.time <= other.time
            && self.score == other.score
        {
            return true;
        }

        if self.position == other.position
            && self.opened_valves == other.opened_valves
            && self.time == other.time
            && self.score >= other.score
        {
            return true;
        }

        return false;
    }

    fn worst_possible_score(&self) -> Unit {
        // The worst possible score this path could have is the best score of of any of the possible
        // paths we have explored so far

        // At worst case we will accumulate points until the end of the simulation
        // having taken no other choice
        let mut worst_score = self.sitting_score();

        // For each of the future actions we have predicted, if it improves our worst score, use that instead
        for state in self.next.borrow().values().filter_map(|s| s.as_ref()) {
            worst_score = worst_score.max(state.worst_possible_score());
        }
        worst_score
    }

    fn theoretical_best_possible_score(&self) -> Unit {
        // The theoretical best score this path can have is bounded by
        //  * Its score so far
        //  * How many remaining valves coudld be opened
        //  * How many points they would provide if they were opened optimally
        //
        // If this path is fully explored, this score should converge to be equal
        // to the worst possible score (i.e. the best possible and the worst possible, will become
        // the actual)
        //
        // Each unexplored path has a theoretical best
        // Each explored path has a theoretical best
        //   We take the maximum of each of these

        let mut unopened_valves = self
            .maze
            .valves
            .values()
            .filter(|v| !self.opened_valves.contains(&v.name))
            .filter(|v| {
                self.maze.distance_between(&self.position, &v.name)
                    <= self.maze.time_limit() - self.time
            })
            .collect::<Vec<_>>();

        // Sort by rate, ascending
        unopened_valves.sort_by(|a, b| a.rate.cmp(&b.rate));

        let mut mininum_travel_time = unopened_valves.iter().map(|v| self.maze.distance_between(&self.position, &v.name)).min().unwrap_or(0);

        // Simulate the rest of the game if we could magically open the best valves in order
        // until the end of the game
        let mut theoretical_score = self.score;
        let mut remaining_time = self.maze.time_limit() - self.time;
        let mut theoretical_rate = self.flow_rate();

        while remaining_time > 0 {
            if mininum_travel_time == 0 {
                if let Some(v) = unopened_valves.pop() {
                    theoretical_rate += v.rate;
                    mininum_travel_time = self.maze.min_valve_distance;
                }
            }
            mininum_travel_time -= 1;
            theoretical_score += theoretical_rate;
            remaining_time -= 1;
        }

        if self.is_fully_explored() {
            self.worst_possible_score()
        } else {
            theoretical_score
        }
/* 
        let mut theoretical_best = self.sitting_score();
        assert!(theoretical_score >= theoretical_best);
        for (choice, next) in self.next.borrow().iter() {
            if let Some(explored) = next {
                theoretical_best = theoretical_best.max(explored.theoretical_best_possible_score());
            } else {
                // If the path is unexplored, use the theoretical cap we have calculated
                // (that value should be higher than any existing explored path)
                assert!(theoretical_score >= theoretical_best);
                theoretical_best = theoretical_best.max(theoretical_score);
            }
        }

        theoretical_best

        */
    }

    fn flow_rate(&self) -> Unit {
        self.opened_valves
            .iter()
            .map(|v| self.maze.valves[v].rate)
            .sum()
    }

    /// Computes the list of possible reasonable next actions given the current
    /// state
    fn choices(
        position: &str,
        time: Unit,
        opened_valves: &BTreeSet<String>,
        maze: &Rc<Maze>,
    ) -> Vec<Choice> {
        let mut choices = Vec::new();

        /*
        // Open the current valve if there is one
        if opened_valves.contains(position) {
            choices.push(Choice::OpenValve);
        }
        */

        // Travel to an unopened valve with a positive flow rate
        // that is near enough it is possible to reach
        for valve in maze
            .significant_valves()
            .filter(|v| !opened_valves.contains(&v.name))
            .filter(|v| v.distance_to(position) as Unit + 1 <= maze.time_limit() - time)
        {
            {
                choices.push(Choice::MoveTo(valve.name.clone()));
            }
        }

        choices
    }

    fn move_to(self: &Rc<Self>, to_valve: &str) -> Rc<State> {
        let elapsed_time = self.maze.distance_between(&self.position, to_valve) + 1;
        let mut score = self.score;

        score += self.flow_rate() * elapsed_time;

        let mut new_opened_valves = self.opened_valves.clone();
        new_opened_valves.insert(to_valve.to_string());

        let choices = Self::choices(
            to_valve,
            self.time + elapsed_time,
            &new_opened_valves,
            &self.maze,
        );

        Rc::new(State {
            position: to_valve.to_string(),
            time: self.time + elapsed_time,
            opened_valves: new_opened_valves,
            score,
            maze: self.maze.clone(),
            previous: Some(Rc::downgrade(self)),
            next: RefCell::new(HashMap::from_iter(
                choices.iter().map(|c| (c.clone(), None)),
            )),
            pruned_in_favor_of: RefCell::new(None),
        })
    }

    // If there are no more choices, this state is complete
    fn is_leaf(&self) -> bool {
        self.next.borrow().len() == 0
    }

    fn is_fully_explored(&self) -> bool {
        self.next.borrow().values().all(|s| s.is_some())
    }

    fn is_pruned(&self) -> bool {
        self.pruned_in_favor_of.borrow().is_some()
    }

    fn prune_in_favor_of(&self, other: &Rc<Self>) {
        self.pruned_in_favor_of.borrow_mut().replace(other.clone());
    }

    /*
    fn open_valve(self: &Rc<Self>) -> Rc<State> {
        let mut opened_valves = self.opened_valves.clone();
        opened_valves.insert(self.position.clone());
        Rc::new(State {
            position: self.position.clone(),
            time: self.time + 1,
            opened_valves,
            score: self.score + self.flow_rate(),
            maze: self.maze.clone(),
            previous: Some(Rc::downgrade(self)),
            next: RefCell::new(HashMap::from_iter(
                Self::choices(
                    &self.position,
                    self.time + 1,
                    &self.opened_valves,
                    &self.maze,
                )
                .iter()
                .map(|c| (c.clone(), None)),
            )),
        })
    }
    */

    fn score(&self) -> Unit {
        self.score
    }

    // Score if we take no more choices
    fn sitting_score(&self) -> Unit {
        self.score + self.flow_rate() * (self.maze.time_limit() - self.time)
    }
}

impl Maze {
    pub fn new(valves: HashMap<String, Valve>) -> Self {
        let maze = Self { valves, min_valve_distance: 0 };

        // Pre-compute the distance maps
        for valve in maze.valves.keys() {
            let distance_map = maze.build_distance_map(valve.as_str());
            maze.valves
                .get(valve.as_str())
                .unwrap()
                .distance_map
                .replace(distance_map);
        }

        let mut minimum_distance_between_significant_valves = Unit::MAX;
        for a in maze.significant_valves() {
            for b in maze.significant_valves() {
                let d = maze.distance_between(&a.name, &b.name);
                if d != 0 {
                    minimum_distance_between_significant_valves = minimum_distance_between_significant_valves.min(d);
                }
            }
        }

        assert!(minimum_distance_between_significant_valves > 0);

        Self { valves: maze.valves, min_valve_distance: minimum_distance_between_significant_valves }
    }

    pub fn neighbors(&self, from: &str) -> Vec<String> {
        self.valves.get(from).unwrap().tunnels_to.clone()
    }

    pub fn significant_valves(&self) -> impl Iterator<Item = &Valve> {
        self.valves.values().filter(|v| v.rate != 0)
    }

    pub fn time_limit(&self) -> Unit {
        30
    }

    /// Return a HashMap of the distances from a starting node to all other nodes in the graph.
    ///
    /// This function uses a breadth-first search (BFS) to find the distances from a starting node
    /// to all other nodes in the graph. The keys of the returned HashMap are the nodes in the graph
    /// and the values are the distances from the starting node to each node in the graph.
    ///
    /// The starting node is specified by the parameter `from`. The distance from a node to itself
    /// is always 0.
    pub fn build_distance_map(&self, from: &str) -> HashMap<String, usize> {
        let mut distance_map = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(from.to_string());
        distance_map.insert(from.to_string(), 0);
        while let Some(valve) = queue.pop_front() {
            for neighbor in self.neighbors(&valve) {
                if !distance_map.contains_key(&neighbor) {
                    distance_map.insert(neighbor.clone(), distance_map[&valve] + 1);
                    queue.push_back(neighbor);
                }
            }
        }
        distance_map
    }

    pub fn distance_between(&self, from: &str, to: &str) -> Unit {
        self.valves.get(from).unwrap().distance_map.borrow()[to] as Unit
    }
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn parse_input(input: &str) -> Maze {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let (valve, rate, tunnels) = scan_fmt!(
            line,
            "Valve {} has flow rate={}; tunnel leads to valve {[A-Z, ]}",
            String,
            Unit,
            String
        )
        .or_else(|_| {
            scan_fmt!(
                line,
                "Valve {} has flow rate={}; tunnels lead to valves {[A-Z, ]}",
                String,
                Unit,
                String
            )
        })
        .unwrap();
        valves.insert(
            valve.clone(),
            Valve {
                name: valve.clone(),
                rate: rate,
                tunnels_to: tunnels.split(", ").map(|s| s.to_string()).collect(),
                distance_map: RefCell::new(HashMap::new()),
            },
        );
    }
    Maze::new(valves)
}

fn part1(input: &str) -> String {
    let maze = Rc::new(parse_input(input));

    let root_state = Rc::new(State::new(maze.clone()));

    let mut unexplored = Vec::new();

    let mut leaves: Vec<Rc<State>> = Vec::new();
    let mut states_by_position_and_opened: HashMap<(String, BTreeSet<String>), HashSet<Rc<State>>> = HashMap::new();

    unexplored.push(root_state);

    'unexplored_queue: while let Some(state) = unexplored.pop() {
        println!("Queue: {} Exploring {}", unexplored.len(), state);

        if state.is_pruned() {
            continue;
        }

        // Try to prune the branch
        // First, check to see if any leaf (completed exploration) is better
        if let Some(leaf) = leaves.first() {
            if leaf.is_strictly_better_than(&state) {
                println!("  -Pruning in favor of leaf {}", leaf);
                state.prune_in_favor_of(leaf);
                continue 'unexplored_queue;
            }
        }

        // Next, check to see if any of the other explorations is strictly better
        for other in &unexplored {
            if other.is_strictly_better_than(&state) {
                println!("  -Pruning in favor of state {}", other);
                state.prune_in_favor_of(other);
                continue 'unexplored_queue;
            }
        }

        // Next, check to see if any other states have the same position and
        // set of opened valves
        let key = (state.position.clone(), state.opened_valves.clone());
        if states_by_position_and_opened.contains_key(&key) {
            for other in states_by_position_and_opened.get(&key).unwrap() {
                if other.is_strictly_better_than(&state) {
                    println!("  -Pruning in favor of state {}", other);
                    state.prune_in_favor_of(other);
                    continue 'unexplored_queue;
                }
            }
        }

        states_by_position_and_opened.get_mut_or_default(&key).insert(state.clone());

        // If we're still here, we're not pruned

        // If we're not pruned, expand the state
        {
            let mut next_choices = state.next.borrow_mut();

            // Find the next unexplored choices
            for (choice, next) in next_choices.iter_mut() {
                if next.is_none() {
                    let next_state = match &choice {
                        Choice::MoveTo(next_position) => state.move_to(&next_position),
                    };

                    // If the next choice has more options, add it to the exploration queue
                    if !next_state.is_fully_explored() {
                        
                        println!("  +Found unexplored {}", next_state);
                        unexplored.push(next_state.clone());
                        
                        
                    } else {
                        assert!(next_state.is_leaf());
                        println!("  +Found leaf {}", next_state);
                        leaves.push(next_state.clone());
                        // Sort the leaves, by their final score descending
                        leaves.sort_by(|a, b| b.sitting_score().cmp(&a.sitting_score()));
                    }
                    next.replace(next_state);
                }
            }
        }

        if state.is_leaf() {
            println!("  -Fully explored path!");
            leaves.push(state);
            // Sort the leaves, by their final score descending
            leaves.sort_by(|a, b| b.sitting_score().cmp(&a.sitting_score()));
        } else if !state.is_fully_explored() {
            unexplored.insert(0, state);
        } else {
            println!("   Fully explored");
        }


        // Sort the explorations by the most promising

        /*unexplored.sort_by(|a, b| 
            a.score.cmp(&b.score).then_with(|| {
                b.time.cmp(&a.time) })
            );
*/
            unexplored.sort_by(|a, b| 
                a.sitting_score().cmp(&b.sitting_score())
                );
    }

    // Find the best leaf
    let best_leaf = leaves
        .iter()
        .max_by(|a, b| a.sitting_score().cmp(&b.sitting_score()))
        .unwrap();

    best_leaf.sitting_score().to_string()
}

fn part2(input: &str) -> String {
    let maze = parse_input(input);

    "".to_string()
}

fn main() {
    let run_input = true;

    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "1651";
    let expected_example_part2 = "?";

    println!("AOC 2022 {}", env!("CARGO_PKG_NAME"));
    println!("Sample Part 1:");
    let sample_result_part1 = part1(&example_input);
    if sample_result_part1 != expected_example_part1 {
        println!(
            "  Answer: {} (expected {})",
            sample_result_part1, expected_example_part1
        );
        panic!();
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
