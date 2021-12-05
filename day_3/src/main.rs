// Day 3: https://adventofcode.com/2021/day/3

use itertools::Itertools;
use std::fs;

fn str_to_decimal(binary: &str) -> isize {
    isize::from_str_radix(binary, 2).expect("Error parsing decimal.")
}

fn get_values_matching_bit_criteria(
    values: &Vec<String>,
    position: usize,
    keep_more: bool,
    keep_ones: bool,
) -> Vec<String> {
    // keep_more - If true, will returns the biggest vec
    // return_ones - If true, will returns ones at position when equality
    let mut zeroes: Vec<String> = Vec::new();
    let mut ones: Vec<String> = Vec::new();
    for v in values {
        let char_at_pos = v.chars().nth(position).expect("");
        if char_at_pos == '0' {
            zeroes.push(v.to_string());
        } else {
            ones.push(v.to_string());
        }
    }

    // Return the biggest vec, or if equal use return ones;
    match ones {
        ones if ones.len() > zeroes.len() => {
            if keep_more {
                return ones;
            }
            zeroes
        }
        ones if ones.len() < zeroes.len() => {
            if keep_more {
                return zeroes;
            }
            ones
        }
        ones if ones.len() == zeroes.len() => {
            if keep_ones {
                return ones;
            }
            zeroes
        }
        _ => {
            panic!("This should not happen.")
        }
    }
}

fn get_rating(values: &[&str], keep_more: bool, return_ones: bool) -> String {
    let mut position = 0;
    let mut matching_ratings = values.iter().map(|x| x.to_string()).collect();

    loop {
        matching_ratings = get_values_matching_bit_criteria(&matching_ratings, position, keep_more, return_ones);
        if matching_ratings.len() == 1 {
            // Return when only 1 rating left
            break;
        }
        position += 1;
    }
    matching_ratings.first().expect("").to_string()
}

fn main() {
    let input = fs::read_to_string("example.txt").expect("Error reading file");
    let input_values: Vec<&str> = input.lines().collect();

    // Part 1:
    // Find the most common bit in the corresponding position of all numbers (ie. gamma rate)
    let values_count = input_values.len();
    let binary_len = input_values.get(0).expect("Value not found!").len();
    let mut total_counts = vec![0; binary_len];
    for diagn_num in &input_values {
        for (index, value) in diagn_num.chars().into_iter().enumerate() {
            if value == '1' {
                total_counts[index] += 1
            }
        }
    }

    // Gamma is the majority value for each position
    let gamma_vec: Vec<usize> = total_counts
        .iter()
        .map(|&x| if x > values_count / 2 { 1 } else { 0 })
        .collect();
    let gamma_str: String = gamma_vec.iter().join("");
    let gamma = isize::from_str_radix(gamma_str.as_str(), 2).unwrap();

    // Epsilon is the inverse of the gamma rate
    let epsilon_vec: Vec<usize> = gamma_vec.iter().map(|x| 1 - x).collect();
    let epsilon_str: String = epsilon_vec.iter().join("");
    let epsilon = isize::from_str_radix(epsilon_str.as_str(), 2).unwrap();

    // The power consumption can then be found by multiplying the gamma rate by the epsilon rate.
    println!(
        "Part 1: The power consumption of the submarine is {}",
        epsilon * gamma
    );

    // Part 2: Find the oxygen generator and the CO2 scrubber rating values
    let oxygen_rating_str = get_rating(&input_values, true, true);
    let oxygen_rating = str_to_decimal(oxygen_rating_str.as_str());

    let co2_rating_str = get_rating(&input_values, false, false);
    let co2_rating = str_to_decimal(co2_rating_str.as_str());

    println!(
        "Part 2: The power consumption of the submarine is {}",
        oxygen_rating * co2_rating
    );
}
