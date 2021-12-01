// Day 1: https://adventofcode.com/2021/day/1

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let input_values: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();

    // Part 2:
    // Find the number of times a depth measurement increases

    // Loop option
    let mut count_increase = 0;
    let mut last_value_option = None;
    for value in input_values.iter() {
        if let Some(last_value) = last_value_option {
            if value > last_value {
                count_increase += 1;
            }
        }
        // Update last value with new value
        last_value_option = Some(value);
    }
    println!("Loop: The number of depth increases is {}", count_increase);

    // Window option
    let count_window = input_values.windows(2).filter(|w| w[1] > w[0]).count();
    println!("Window: The number of depth increases is {}", count_window);

    // Part 2:
    // count the number of times the sum of measurements in this sliding window increases
    let count_window_3 = input_values
        .windows(4)
        //.filter(|w| w[1] + w[2] + w[3] > w[0] + w[1] + w[2])
        .filter(|w| w[1..].iter().sum::<usize>() > w[..=2].iter().sum::<usize>())
        .count();
    println!(
        "Window 3: The number of depth increases is {}",
        count_window_3
    );
}
