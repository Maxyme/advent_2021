use itertools::Itertools;
use std::cmp::max;
use std::fs;

use std::ops::AddAssign;

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

fn main() {
    let input = fs::read_to_string("example.txt").expect("Error reading file");
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

    // Loop steps
    // Note velocity must be an integer
    let mut initial_velocity = Point { x: 6, y: 3 };
    let mut probe_position = Point { x: 0, y: 0 };
    let mut top_y = i32::MIN;

    // loop all possible initial velocities
    'outer: loop {
        let mut velocity = initial_velocity;

        'inner: loop {
            println!("{:?}", probe_position);

            // Update max_y
            if probe_position.y > top_y {
                top_y = probe_position.y;
            }

            // Update probe position
            probe_position += velocity;

            // Check if in target area
            if probe_position.x <= max_x
                && probe_position.x >= min_x
                && probe_position.y <= max_y
                && probe_position.y >= min_y
            {
                println!("Hit");
                break;
            }

            // Otherwise check if passed target area
            if probe_position.x > max_x || probe_position.y < min_y {
                println!("Impossible");
                break;
            }

            // Update velocity
            // Add drag
            if velocity.x > 0 {
                velocity.x = velocity.x - 1;
            }

            // Add gravity
            velocity.y -= 1;
        }
    }

    // Part 1: Find the initial velocity that causes the probe to reach the
    // highest y position and still eventually be within the target area after any step.
    // What is the highest y position it reaches on this trajectory?
    println!("{}", top_y)

    // Todo, make an ellipse from each point in the target area
    // to the source and check what the max amplitude could be
}
