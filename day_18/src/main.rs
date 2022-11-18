use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fs;

lazy_static! {
    static ref FIRST_NUM_RE: Regex = Regex::new(r"^\D*(\d+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // Create a vec of instructions
    let mut lines = input.lines();
    let mut sum: String = lines.next().unwrap().to_string();
    for line in lines {
        // Add the inputs together to form a new snailfish number (Pairs of pairs)
        let addition = add_numbers(&sum, line);
        // Reduce the input and set as the new sum
        sum = reduce_number(&addition);
    }

    let sum_magnitude = get_magnitude(&sum);
    println!("Part 1: {}", sum_magnitude);

    // Part 2: what is the largest magnitude from the sum of any 2 numbers
    let mut biggest_magnitude = 0;
    for pair in input.lines().permutations(2) {
        let addition = add_numbers(pair[0], pair[1]);
        let reduction = reduce_number(&addition);
        let sum_magnitude = get_magnitude(&reduction);
        if sum_magnitude > biggest_magnitude {
            biggest_magnitude = sum_magnitude;
        }
    }

    println!("Part 2: {}", biggest_magnitude);
}

fn get_nested_pair(s: &str) -> Option<(usize, usize)> {
    // Note, could be replaced by a regex
    let mut open_brackets_queue = VecDeque::new();
    let chars = s.chars();
    for (index, char) in chars.enumerate() {
        if char == '[' {
            open_brackets_queue.push_back(index);
        }
        if char == ']' {
            if open_brackets_queue.len() > 4 {
                let start_index = open_brackets_queue.pop_back().unwrap();
                let end_index = index;
                return Some((start_index, end_index));
            }
            open_brackets_queue.pop_back().unwrap();
        }
    }
    None
}

fn add_numbers(s_1: &str, s_2: &str) -> String {
    format!("[{},{}]", s_1, s_2)
}

fn explode_number(s: &str, start: usize, end: usize) -> String {
    // Get the pair as usize
    let (left, right) = s[start + 1..end]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    // Find the number left and right of the pair and update them
    // Todo replace by a last number regex
    let mut left_side = s[..start].to_string();
    let reversed_left = left_side.chars().rev().collect::<String>();
    if let Some(cap) = FIRST_NUM_RE.captures(&reversed_left) {
        let item = cap.get(1).unwrap();
        let end = left_side.len() - item.start();
        let start = left_side.len() - item.end();
        let old_number = &left_side[start..end];
        let old_number = old_number.parse::<usize>().unwrap();

        let new_number = left + old_number;
        left_side.replace_range(start..end, &new_number.to_string());
    }

    let mut right_side = s[end + 1..].to_string();
    if let Some(cap) = FIRST_NUM_RE.captures(&right_side) {
        let item = cap.get(1).unwrap();
        let old_number = right_side[item.start()..item.end()]
            .parse::<usize>()
            .unwrap();

        let new_number = right + old_number;
        right_side.replace_range(item.start()..item.end(), &new_number.to_string());
    }

    format!("{}{}{}", left_side, "0", right_side)
}

fn get_number_to_split(s: &str) -> Option<usize> {
    // Check if there a number with 2 digits
    let inter = s.chars().collect::<Vec<char>>();
    let windows = inter.windows(2);

    for (index, window) in windows.enumerate() {
        if window.iter().all(|x| x.is_numeric()) {
            return Some(index);
        }
    }
    None
}

fn split_number(s: &str, index: usize) -> String {
    // Get the number at the index
    let number = s[index..index + 2].parse::<f64>().unwrap();
    let right = (number / 2.0).ceil();
    let left = (number / 2.0).floor();
    format!("{}[{},{}]{}", &s[..index], left, right, &s[index + 2..])
}

/// To reduce follow these rules, until none of these actions applies
/// If any pair is nested inside four pairs, the leftmost such pair explodes.
/// If any regular number is 10 or greater, the leftmost such regular number splits.
fn reduce_number(s: &str) -> String {
    // First: check if a number needs to be exploded
    if let Some(pair) = get_nested_pair(s) {
        // snail number contains a 4x nested pair
        let exploded = explode_number(s, pair.0, pair.1);
        return reduce_number(&exploded);
    }

    // Second: check if a number needs to be split
    if let Some(split_index) = get_number_to_split(s) {
        let split = split_number(s, split_index);
        return reduce_number(&split);
    }
    s.to_string()
}

/// The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right element
/// Magnitude calculations are recursive
fn get_magnitude(s: &str) -> usize {
    if s.len() == 1 {
        return s[..1].parse::<usize>().unwrap();
    }

    // Split into left and right and if they are both numbers, return early
    // else return recursively
    let without_external_brackets = &s[1..s.len() - 1];

    // Todo: replace with regex?
    let mut open_brackets_queue = VecDeque::new();
    let mut split_index = 0;
    for (index, c) in without_external_brackets.chars().enumerate() {
        if c == ',' && open_brackets_queue.is_empty() {
            split_index = index;
            break;
        }
        if c == '[' {
            open_brackets_queue.push_back(c);
        } else if c == ']' {
            open_brackets_queue.pop_back();
        }
    }

    let left = &without_external_brackets[..split_index];
    let right = &without_external_brackets[split_index + 1..];

    get_magnitude(left) * 3 + get_magnitude(right) * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Test exploding snailfish number
    #[test]
    fn test_get_pair_indices() {
        let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let (start, end) = get_nested_pair(input).unwrap();
        assert_eq!(start, 10);
        assert_eq!(end, 14);
    }

    #[test]
    fn test_get_explode() {
        let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let exploded = explode_number(input, 10, 14);
        assert_eq!(exploded, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let exploded = reduce_number(input);
        assert_eq!(exploded, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_number_to_split() {
        let input = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let split_index = get_number_to_split(input).unwrap();
        assert_eq!(split_index, 13);
    }

    #[test]
    fn test_split_snail_number() {
        let input = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let split = split_number(input, 13);
        assert_eq!(split, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    }

    #[test]
    fn test_get_magnitude() {
        let input = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]";
        let magnitude = get_magnitude(input);
        assert_eq!(magnitude, 3488)
    }
}
