// Day 2: https://adventofcode.com/2021/day/2

use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file.");

    // Parse command values from lines as vec of tuple of str and usize
    let command_values: Vec<(&str, usize)> = input
        .lines()
        .map(|x| x.splitn(2, char::is_whitespace).collect_tuple().unwrap())
        .map(|v: (&str, &str)| (v.0, v.1.parse::<usize>().unwrap()))
        .collect();

    // Part 1: Calculate the horizontal position and depth you would have after following the planned course
    let mut depth = 0;
    let mut horizontal_pos = 0;

    // Loop
    for (command, value) in &command_values {
        match *command {
            "forward" => horizontal_pos += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Unknown command."),
        }
    }
    println!(
        "Part 1 Loop: Multiplying final horizontal position by final depth = {}",
        horizontal_pos * depth
    );

    // Functional with fold
    let (horizontal_pos_fold, depth_fold) = &command_values.iter().fold(
        (0, 0),
        |(hor_pos, depth), &(command, value)| match command {
            "forward" => (hor_pos + value, depth),
            "down" => (hor_pos, depth + value),
            "up" => (hor_pos, depth - value),
            _ => panic!("Unknown command."),
        },
    );

    println!(
        "Part 1 Fold: Multiplying final horizontal position by final depth = {}",
        horizontal_pos_fold * depth_fold
    );

    // Part 2: with aim
    let mut depth = 0;
    let mut horizontal_pos = 0;
    let mut aim = 0;
    for (command, value) in command_values {
        match command {
            "forward" => {
                horizontal_pos += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Unknown command."),
        }
    }

    println!(
        "Part 2: Multiplying final horizontal position by final depth = {}",
        horizontal_pos * depth
    );
}
