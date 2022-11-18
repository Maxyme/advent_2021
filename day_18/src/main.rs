use std::collections::VecDeque;
use itertools::Itertools;
use std::fs;


fn explode(snail_number: &str) -> String {
    let mut left = String::new();
    let mut last_number_index: usize = 0;
    let mut next_number_index: usize = 0;

    let mut right_pair: usize = 0;
    let mut left_pair: usize = 0;


    let mut chars = snail_number.chars();
    let mut index = 0;
    for char in chars {
        left.push(char);
        index += 1;
    }

    let middle = "a".to_string();
    let mut right = &snail_number[..index];
    // increment number on right
    let next_number = &right[next_number_index..next_number_index+1].parse::<usize>().unwrap();
    let new_next_number = next_number + right_pair;
    //right[next_number_index] = right[next_number_index]

    format!("{},{},{}", left, middle, right)
}

/// Reduce snailfish number by exploding nested pairs and/or splitting regular numbers
fn reduce(snail_number: &str) -> String {
    let mut left = String::new();

    let mut chars = snail_number.chars();
    let mut index = 0;
    for char in chars {
        left.push(char);
        index += 1;
    }
    let middle = "a".to_string();
    //new_string
    let right = &snail_number[..index];
    format!("{},{},{}", left, middle, right)
}

fn main() {
    let input = fs::read_to_string("example.txt").expect("Error reading file");

    // Create a vec of instructions
    let mut lines = input.lines();
    let mut sum : String = lines.next().unwrap().to_string();
    for line in lines {
        // Add the inputs together to form a new snailfish number (Pairs of pairs)
        let addition = format!("[{},{}]", sum, line);
        println!("s {}", addition);

        // Reduce the input and set as the new sum
        sum = reduce_number(addition.as_str());
        println!("Sum {}", sum);
    }
    println!("Sum {}", sum);
    // To reduce follow these rules, until none of these actions applies
    // If any pair is nested inside four pairs, the leftmost such pair explodes.
    // If any regular number is 10 or greater, the leftmost such regular number splits.

    // The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right element
    // Magnitude calculations are recursive
    let sum_magnitude = 0;
    println!("Part 1: {}", sum_magnitude);

}

fn get_nested_pair(s: &str) -> Option<(usize, usize)> {
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

// Get index of the last number in str
// Note, can be inlined
fn get_last_number_index(s: &str) -> Option<usize> {
    let mut last_number_index = None;
    for (index, char) in s.chars().enumerate() {
        if char.is_alphanumeric() {
            last_number_index = Some(index);
        }
    };
    last_number_index
}

fn get_first_number_index(s: &str) -> Option<usize> {
    for (index, char) in s.chars().enumerate() {
        if char.is_alphanumeric() {
            return Some(index);
        }
    };
    None
}

fn explode_number(s: &str, start: usize, end: usize) -> String {
    // Get the pair as usize
    let (left, right) = s[start + 1..end].split(',').map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();

    // Find the number left and right of the pair and update them
    // Todo: can this number be > 10? - 2 chars? then regex..?
    let mut left_side = s[..start].to_string();
    if let Some(left_number_index) = get_last_number_index(&left_side) {
        // Update the number
        let old_number = left_side.get(left_number_index..left_number_index + 1).unwrap().parse::<usize>().unwrap();
        let new_number = left + old_number;
        left_side.replace_range(left_number_index..left_number_index + 1, &new_number.to_string());
    }

    let mut right_side = s[end + 1..].to_string();
    if let Some(right_number_index) = get_first_number_index(&right_side) {
        let old_number = right_side.get(right_number_index..right_number_index + 1).unwrap().parse::<usize>().unwrap();
        let new_number = right + old_number;
        right_side.replace_range(right_number_index..right_number_index + 1, &new_number.to_string());
    }
    format!("{}{}{}", left_side, "0", right_side)
}

fn get_number_to_split(s: &str) -> Option<usize> {
    // Check if there a number with 2 digits
    let inter = s.chars().collect::<Vec<char>>();
    let mut windows = inter.windows(2);

    for (index, window) in windows.enumerate() {
        if window.iter().all(|x| x.is_numeric()) {
            return Some(index)
        }
    }
    None
}

fn split_number(s: &str, index: usize) -> String {
    // Get the number at the index
    let number = s.get(index..index + 2).unwrap().parse::<usize>().unwrap();
    let right = (number as f64 / 2.0).ceil();
    let left = (number as f64 / 2.0).floor();
    format!("{}[{},{}]{}", &s[..index], left, right, &s[index + 2..])

}

fn reduce_number(s: &str) -> String {
    // Check if a number needs to be exploded
    if let Some(pair) = get_nested_pair(s) {
        // snail number contains a 4x nested pair
        let exploded = explode_number(s, pair.0, pair.1);
        return reduce_number(&exploded);
    }

    // Check if a number needs to be split
    if let Some(split_index) = get_number_to_split(s) {
        let split = split_number(s, split_index);
        return reduce_number(&split);
    }

    s.to_string()
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

    // [[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]
    #[test]
    fn test_overflow() {
        let input = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let split = split_number(input, 13);
        assert_eq!(split, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    }
}
