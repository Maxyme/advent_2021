// Day 10: https://adventofcode.com/2021/day/10

use phf::phf_map;
use std::collections::VecDeque;
use std::fs;

static VALUES: phf::Map<char, usize> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
};
static CLOSING_MAP: phf::Map<char, char> = phf_map! {
    ')' => '(',
    ']' => '[',
    '}' => '{',
    '>' => '<',
};

static CLOSING_SCORE: phf::Map<char, usize> = phf_map! {
    '(' => 1,
    '[' => 2,
    '{' => 3,
    '<' => 4,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // Part 1: What is the total syntax error score for those errors
    let mut opening_chars: VecDeque<char> = VecDeque::new();
    let mut total_score = 0;
    for line in input.lines() {
        for char in line.chars() {
            if !CLOSING_MAP.contains_key(&char) {
                opening_chars.push_back(char);
            } else {
                // Check if matching opening char is the last of the queue
                let last_opening = opening_chars.pop_back().unwrap();
                if last_opening != *CLOSING_MAP.get(&char).unwrap() {
                    // Add value to score
                    let value = VALUES.get(&char).unwrap();
                    total_score += value;
                }
            }
        }
    }
    println!("Part 1 total value: {}", total_score);

    // Part 2: Find the completion string for each incomplete line, score the completion strings, and sort the scores.
    // What is the middle score?
    let mut opening_chars: VecDeque<char> = VecDeque::new();
    let mut scores: Vec<usize> = Vec::new();
    for line in input.lines() {
        'outer: for char in line.chars() {
            if !CLOSING_MAP.contains_key(&char) {
                opening_chars.push_back(char);
            } else {
                // Check if matching opening char is the last of the queue
                let last_opening = opening_chars.pop_back().unwrap();
                if last_opening != *CLOSING_MAP.get(&char).unwrap() {
                    // Line is corrupted, skip it
                    opening_chars.clear();
                    break 'outer;
                }
            }
        }
        // Line was incomplete, compute and add score
        if !opening_chars.is_empty() {
            let mut score = 0;
            while let Some(char) = opening_chars.pop_back() {
                score *= 5;
                // Find corresponding closing char score
                let char_value = CLOSING_SCORE.get(&char).unwrap();
                score += char_value;
            }
            scores.push(score)
        }
    }
    scores.sort_unstable();
    println!(
        "Part 2 middle score: {:?}",
        scores[(((scores.len() / 2) as f64).floor() as usize)]
    );
}
