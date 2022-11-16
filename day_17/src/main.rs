use itertools::Itertools;
use std::fs;

use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Box {
    top_left: Point,
    bottom_right: Point,
}

impl Box {
    fn contains(&self, point: &Point) -> bool {
        if point.x <= self.bottom_right.x
            && point.x >= self.top_left.x
            && point.y <= self.top_left.y
            && point.y >= self.bottom_right.y
        {
            return true;
        }
        false
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

// Find the top y from the initial_position
fn top_y(initial_position: Point, initial_velocity: Point, target: &Box) -> Option<i32> {
    let mut top_y = i32::MIN;
    let mut velocity = initial_velocity;
    let mut probe_position = initial_position;
    loop {
        // Update max_y
        if probe_position.y > top_y {
            top_y = probe_position.y;
        }

        // Update probe position
        probe_position += velocity;

        // Check if in target area
        if target.contains(&probe_position) {
            return Some(top_y);
        }

        // Otherwise check if passed target area
        if probe_position.x > target.bottom_right.x || probe_position.y < target.bottom_right.y {
            return None;
        }

        // Update velocity
        // Add drag
        if velocity.x > 0 {
            velocity.x -= 1;
        }

        // Add gravity
        velocity.y -= 1;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let (x_coords, y_coords) = input
        .strip_prefix("target area: ")
        .unwrap()
        .split(", ")
        .collect_tuple()
        .unwrap();

    let (min_x, max_x) = x_coords
        .strip_prefix("x=")
        .unwrap()
        .splitn(2, "..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    let (min_y, max_y) = y_coords
        .strip_prefix("y=")
        .unwrap()
        .splitn(2, "..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    let target = Box {
        top_left: Point { x: min_x, y: max_y },
        bottom_right: Point { x: max_x, y: min_y },
    };

    let probe_position = Point { x: 0, y: 0 };

    let mut top: i32 = i32::MIN;
    'outer: for x in 1..max_x {
        for y in min_y..min_y.abs() {
            let initial_velocity = Point { x, y };
            // break when top doesn't
            match top_y(probe_position, initial_velocity, &target) {
                None => {
                    continue;
                }
                Some(top_y) => {
                    // Assume that top y is always increasing
                    if top_y > top {
                        top = top_y;
                    } else {
                        break 'outer;
                    }
                }
            };
        }
    }

    // Part 1: Find the initial velocity that causes the probe to reach the
    // highest y position and still eventually be within the target area after any step.
    // What is the highest y position it reaches on this trajectory?
    println!("{}", top);

    // Part 2: Find every initial velocity that will hit the target
    let mut count: usize = 0;
    for x in 1..=max_x {
        for y in min_y..min_y.abs() {
            let initial_velocity = Point { x, y };
            match top_y(probe_position, initial_velocity, &target) {
                None => {
                    continue;
                }
                Some(_) => {
                    count += 1;
                }
            };
        }
    }
    println!("{}", count);
}
