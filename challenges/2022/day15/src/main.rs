#![allow(dead_code, unused_variables, unused_imports)]

use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
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
    range::Range,
};

use scan_fmt::scan_fmt;
use serde_derive::Deserialize;

type Unit = i32;
type C = XY<Unit, true, false>;

struct Sensor {
    sensor_position: C,
    neareset_beacon: C,
}

impl Sensor {
    fn beacon_distance(&self) -> Unit {
        self.sensor_position
            .manhatten_distance(&self.neareset_beacon)
    }
    fn in_range(&self, c: &C) -> bool {
        self.sensor_position.manhatten_distance(c)
            <= self
                .sensor_position
                .manhatten_distance(&self.neareset_beacon)
    }

    // Iterates all of the values just outside the perimiter
    fn perimeter(&self) -> impl Iterator<Item = C> + '_ {
        let d = self.beacon_distance();
        let upper_left = (0..d).map(move |i| self.sensor_position.up(i + 1).left(d - i));
        let upper_right = (0..d).map(move |i| self.sensor_position.up(i + 1).right(d - i));
        let lower_left = (0..d).map(move |i| self.sensor_position.down(i + 1).left(d - i));
        let lower_right = (0..d).map(move |i| self.sensor_position.down(i + 1).right(d - i));

        upper_left
            .chain(upper_right)
            .chain(lower_left)
            .chain(lower_right)
            .chain(std::iter::once(self.sensor_position.left(d + 1)))
            .chain(std::iter::once(self.sensor_position.right(d + 1)))
        // (d + 1) * 2 + 1

        //   c
        //  b#d
        // a#S#e
        //  h#f
        //   g
    }
}

//Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn parse_input(input: &str) -> Vec<Sensor> {
    let mut v = Vec::new();
    for line in input.lines() {
        let (sx, sy, bx, by) = scan_fmt!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            Unit,
            Unit,
            Unit,
            Unit
        )
        .unwrap();

        v.push(Sensor {
            sensor_position: C::new(sx, sy),
            neareset_beacon: C::new(bx, by),
        });
    }
    v
}

fn part1(input: &str, target_row: Unit) -> String {
    let sensors = parse_input(input);

    let mut ranges = Vec::new();
    for s in &sensors {
        let distance_to_target_row = s.sensor_position.y().abs_diff(target_row) as Unit;

        let max_blackout = s.beacon_distance() * 2 + 1;
        let row_blackout = max_blackout - 2 * distance_to_target_row;
        let max_blackout_start = s.sensor_position.x().sub(s.beacon_distance());
        let row_blackout_start = max_blackout_start + distance_to_target_row;

        if row_blackout > 0 {
            let blackout_range = row_blackout_start..=(row_blackout_start + row_blackout - 1);
            let range = Range::new(blackout_range);

            ranges.push(range);
        }
    }

    let mut points: HashSet<_> = ranges.iter().flat_map(|r| r.iter()).collect();

    println!("points: {}", points.len());
    for s in sensors
        .iter()
        .filter(|s| *s.neareset_beacon.y() == target_row)
    {
        points.remove(s.neareset_beacon.x());
    }
    println!("points: {}", points.len());

    // let min = points.iter().min().unwrap().clone();
    // let max = points.iter().max().unwrap().clone();
    // for i in min..=max {
    //     if points.contains(&i) {
    //         print!("#");
    //     } else {
    //         print!(".");
    //     }
    // }
    // println!();

    points.len().to_string()
}

fn part2(input: &str, max_coordinate: Unit) -> String {
    let sensors = parse_input(input);

    let max_range = Range::new(0..=max_coordinate);

    let area = RectangularRange::<C>::new(
        HorizontalRange::new(max_range.clone()),
        VerticalRange::new(max_range.clone()),
    );

    let mut c = None;

    'outer: for sensor in &sensors {
        'points: for point in sensor.perimeter() {
            if area.contains(&point) {
                for sensor in &sensors {
                    if sensor.in_range(&point) {
                        continue 'points;
                    }
                }
                c = Some(point);
                break 'outer;
            }
        }
    }
    let f = *c.unwrap().x() as i64 * 4000000 + *c.unwrap().y() as i64;
    f.to_string()
}

fn main() {
    let run_input = true;

    let example_input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "sample.txt");

    let expected_example_part1 = "26";
    let expected_example_part2 = "56000011";

    println!("AOC 2022 {}", env!("CARGO_PKG_NAME"));
    println!("Sample Part 1:");
    let sample_result_part1 = part1(&example_input, 10);
    if sample_result_part1 != expected_example_part1 {
        println!(
            "  Answer: {} (expected {})",
            sample_result_part1, expected_example_part1
        );
    } else {
        println!("  Answer: {} CORRECT!!!", sample_result_part1);
    }
    println!("Sample Part 2:");
    let sample_result_part2 = part2(&example_input, 20);
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
        let result = part1(&input, 2000000);
        println!("  Answer: {}", result);
        println!("Puzzle Part 2:");
        let result = part2(&input, 4000000);
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
