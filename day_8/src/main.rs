// Day 8: https://adventofcode.com/2021/day/8

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn get_patterns(input_commands: Vec<&str>) -> HashMap<Vec<char>, usize> {
    // Return a dict of what pattern matches to what value
    // Get unique len values first
    let mut commands_values: HashMap<Vec<char>, usize> = HashMap::new();
    let one = input_commands.iter().find(|x| x.len() == 2).expect("");
    commands_values.insert(one.chars().sorted().collect(), 1);

    let seven = input_commands.iter().find(|x| x.len() == 3).expect("");
    commands_values.insert(seven.chars().sorted().collect(), 7);

    let four = input_commands.iter().find(|x| x.len() == 4).expect("");
    commands_values.insert(four.chars().sorted().collect(), 4);

    let eight = input_commands.iter().find(|x| x.len() == 7).expect("");
    commands_values.insert(eight.chars().sorted().collect(), 8);

    // 3 is a value of len 5 with all values in 1
    let three = input_commands
        .iter()
        .find(|x| x.len() == 5 && one.chars().all(|item| x.chars().contains(&item)))
        .expect("");

    let three_vec = three.chars().sorted().collect();

    // 6 has len 6 and contais all values of 1
    let six = input_commands
        .iter()
        .find(|x| x.len() == 6 && !one.chars().all(|item| x.chars().contains(&item)))
        .expect("");

    let six_vec = six.chars().sorted().collect();

    // Find the top right char by comparing eight and six
    let chars: Vec<char> = eight.chars().collect();
    let set_1: HashSet<char> = HashSet::from_iter(chars);
    let chars_six: Vec<char> = six.chars().collect();
    let set_six: HashSet<char> = HashSet::from_iter(chars_six);
    let top_right_char: &char = set_1.difference(&set_six).next().expect("");

    // two is the set containing the top right char
    let two = input_commands
        .iter()
        .find(|x| {
            x.len() == 5
                && x.contains(top_right_char.to_string().as_str())
                && x.chars().sorted().collect::<Vec<char>>() != three_vec
        })
        .expect("");
    let two_vec = two.chars().sorted().collect();

    let five = input_commands
        .iter()
        .find(|x| x.len() == 5 && !x.contains(top_right_char.to_string().as_str()))
        .expect("");
    let five_vec = five.chars().sorted().collect();

    // Find either the middle line or bottom left to diff 0 with 9
    // Get the diff between 6 and 3
    let chars_six: Vec<char> = six.chars().collect();
    let set_six: HashSet<char> = HashSet::from_iter(chars_six);

    // Then find the remaining char not in 5
    let chars_five: Vec<char> = five.chars().collect();
    let set_five: HashSet<char> = HashSet::from_iter(chars_five);
    let bottom_left_char: &char = set_six.difference(&set_five).next().expect("");

    let nine = input_commands
        .iter()
        .find(|x| x.len() == 6 && !x.contains(bottom_left_char.to_string().as_str()))
        .expect("");
    let nine_vec = nine.chars().sorted().collect();

    // Zero contains has len 6 and is not 6 nor 9
    let zero = input_commands
        .iter()
        .find(|x| {
            x.len() == 6
                && x.contains(bottom_left_char.to_string().as_str())
                && x.chars().sorted().collect::<Vec<char>>() != six_vec
                && x.chars().sorted().collect::<Vec<char>>() != nine_vec
        })
        .expect("");
    let zero_vec = zero.chars().sorted().collect();

    // Todo, should raise if already exists
    commands_values.insert(zero_vec, 0);

    commands_values.insert(two_vec, 2);
    commands_values.insert(three_vec, 3);

    commands_values.insert(five_vec, 5);
    commands_values.insert(six_vec, 6);


    commands_values.insert(nine_vec, 9);
    commands_values
}

fn get_output_value(output_commands: Vec<&str>, patterns: &HashMap<Vec<char>, usize>) -> usize {
    // Compose a value from the pattern values
    let mut final_value = String::new();
    for command in output_commands {
        let sorted_chars: Vec<char> = command.chars().sorted().collect();
        let value = patterns.get(&sorted_chars).expect("");
        let char_digit = char::from_digit(*value as u32, 10).expect("");
        final_value.push(char_digit);
    }
    final_value.parse().expect("")
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let commands: Vec<(&str, &str)> = input
        .lines()
        .map(|x| x.splitn(2, " | ").collect_tuple().unwrap())
        .collect();

    // Part 1: In the output values, how many times do digits 1, 4, 7, or 8 appear?
    // note 1 is 2 values, 4 is 4 values, 7 is 3 values and 8 is 7 values.
    const SINGLES: [usize; 4] = [2, 3, 4, 7];
    let mut counter = 0;
    for (_, output) in commands.iter() {
        let all_values: Vec<&str> = output.split(char::is_whitespace).collect();
        let count = all_values
            .iter()
            .filter(|x| SINGLES.contains(&x.len()))
            .count();
        counter += count;
    }

    println!("Part 1: {:?}", counter);

    // Part 2: find the only codes from the input and determine what the output values are
    let mut sum = 0;
    for (input, output) in commands {
        let input_comands: Vec<&str> = input.split(char::is_whitespace).collect();
        let patterns = get_patterns(input_comands);
        let outputs_commands: Vec<&str> = output.split(char::is_whitespace).collect();
        let output_value = get_output_value(outputs_commands, &patterns);
        sum += output_value;
    }
    println!("Part 2: {:?}", sum);
}
