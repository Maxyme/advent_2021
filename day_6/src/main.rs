// Day 6: https://adventofcode.com/2021/day/6

use std::collections::HashMap;
use std::{fs, iter};

const FISH_TIMER: usize = 6;
const NEW_TIMER: usize = FISH_TIMER + 2;

fn run_cycles(fish_timers: &[usize], num_days: usize) -> Vec<usize> {
    // Return a new vec of fish timers based on the number of days
    let mut new_timers = fish_timers.to_owned();
    // Run the number of cycles for fish
    for _ in 0..num_days {
        // Count the number of 0
        let count_zero = new_timers.iter().filter(|x| **x == 0).count();

        new_timers.iter_mut().for_each(|x| {
            if *x == 0 {
                // Fish timers at 0 reset
                *x = FISH_TIMER
            } else {
                // Otherwise substract 1
                *x -= 1
            }
        });

        if count_zero > 0 {
            // Then add a number of new fish at the end
            let new_fishes: Vec<usize> = iter::repeat(NEW_TIMER).take(count_zero).collect();
            new_timers.extend(new_fishes)
        }
    }

    new_timers
}

fn run_cycles_count(fish_timers: &[usize], num_days: usize) -> usize {
    // Return the count of fishes based on the number of days

    if num_days == 0 {
        // Early return
        return fish_timers.len();
    }

    // Create a counter hashmap of the given fish
    let mut counts = fish_timers.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(*c).or_insert(0) += 1;
        acc
    });

    for _ in 0..num_days {
        let mut new_counts: HashMap<usize, usize> = HashMap::new();
        for k in 0..=NEW_TIMER {
            if k == 0 {
                // Insert the count for 0 at 6 and 8
                let v = counts.get(&k).unwrap_or(&0);
                new_counts.insert(FISH_TIMER, *v);
                new_counts.insert(NEW_TIMER, *v);
            } else {
                // Update the values at k - 1 with the count
                let v = counts.get(&k).unwrap_or(&0);
                *new_counts.entry(k - 1).or_insert(0) += v;
            }
        }

        counts = new_counts;
    }

    // Return the total sum of all counts
    counts.values().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let numbers: Vec<usize> = input.split(',').map(|x| x.parse().expect("")).collect();

    // Part1: Count the number of fish after 80 days. Use queue and return queue size
    let num_days_part_1: usize = 80;
    let final_state = run_cycles(&numbers, num_days_part_1);
    println!(
        "The number of lanternfish after 80 days is {}",
        final_state.len()
    );

    // Part 2: Massive number so we have to be clever
    let num_days_part_2: usize = 256;
    let final_count = run_cycles_count(&numbers, num_days_part_2);
    println!(
        "The number of lanternfish after 256 days is {}",
        final_count
    );
}
