// Day 8: https://adventofcode.com/2021/day/8

use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};
use std::fs;

fn get_patterns(input_commands: Vec<&str>) -> HashMap<BTreeSet<char>, usize> {
    // Return a dict of what pattern matches to what value

    // Convert to Vec of Hashsets
    let input_sets: Vec<BTreeSet<char>> = input_commands
        .iter()
        .map(|x| BTreeSet::from_iter(x.chars()))
        .collect();

    let mut commands_values = HashMap::new();

    // Get unique len values first
    let one = input_sets.iter().find(|x| x.len() == 2).unwrap();
    commands_values.insert(one.clone(), 1);

    let seven = input_sets.iter().find(|x| x.len() == 3).unwrap();
    commands_values.insert(seven.clone(), 7);

    let four = input_sets.iter().find(|x| x.len() == 4).unwrap();
    commands_values.insert(four.clone(), 4);

    let eight = input_sets.iter().find(|x| x.len() == 7).unwrap();
    commands_values.insert(eight.clone(), 8);

    // 3 is a value of len 5 with all values in 1
    let three = input_sets
        .iter()
        .find(|x| x.len() == 5 && x.is_superset(one))
        .unwrap();
    commands_values.insert(three.clone(), 3);

    // 6 has len 6 and does not contains all values of 1
    let six = input_sets
        .iter()
        .find(|x| x.len() == 6 && !x.is_superset(one))
        .unwrap();

    commands_values.insert(six.clone(), 6);

    // Find the top right char by comparing eight and six
    let top_right_char = eight.difference(six).next().unwrap();

    // two is the set containing the top right char
    let two = input_sets
        .iter()
        .find(|x| x.len() == 5 && x.contains(top_right_char) && x != &three)
        .unwrap();
    commands_values.insert(two.clone(), 2);

    let five = input_sets
        .iter()
        .find(|x| x.len() == 5 && x != &three && x != &two)
        .unwrap();
    commands_values.insert(five.clone(), 5);

    // Find the bottom left char
    let bottom_left_char = six.difference(five).next().unwrap();

    let nine = input_sets
        .iter()
        .find(|x| x.len() == 6 && !x.contains(bottom_left_char))
        .unwrap();

    commands_values.insert(nine.clone(), 9);
    // Zero contains has len 6 and is not 6 nor 9
    let zero = input_sets
        .iter()
        .find(|x| x.len() == 6 && x != &six && x != &nine)
        .unwrap();

    commands_values.insert(zero.clone(), 0);
    commands_values
}

fn get_output_value(
    output_commands: Vec<&str>,
    patterns: &HashMap<BTreeSet<char>, usize>,
) -> usize {
    // Compose a value from the pattern values
    let mut final_value = String::new();
    for command in output_commands {
        let sorted_chars = BTreeSet::from_iter(command.chars());
        let value = patterns.get(&sorted_chars).unwrap();
        let char_digit = char::from_digit(*value as u32, 10).unwrap();
        final_value.push(char_digit);
    }
    final_value.parse().unwrap()
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
        let input_comands = input.split(char::is_whitespace).collect();
        let patterns = get_patterns(input_comands);
        let outputs_commands = output.split(char::is_whitespace).collect();
        let output_value = get_output_value(outputs_commands, &patterns);
        sum += output_value;
    }
    println!("Part 2: {:?}", sum);
}
