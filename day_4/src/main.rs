// Day 4: https://adventofcode.com/2021/day/4

use ndarray::{s, Array, Array2};
use std::fs;
use std::str::Lines;

fn is_bingo(board: &Array2<usize>, draws: &[usize]) -> bool {
    // Check if the board has a full line or column
    for row in board.rows().into_iter() {
        if row.iter().all(|item| draws.contains(item)) {
            return true;
        }
    }
    for col in board.columns() {
        if col.iter().all(|item| draws.contains(item)) {
            return true;
        }
    }
    false
}

fn get_boards(input_values: &mut Lines) -> Vec<Array2<usize>> {
    let mut boards: Vec<Array2<usize>> = Vec::new();
    let mut board: Array2<usize> = Array2::zeros((5, 5));
    for (index, line) in input_values.enumerate() {
        // skip a line then take 5 lines
        if index % 6 == 0 {
            // Skip first and every 6th line after that
            continue;
        }
        let row_index = (index - 1) % 6;
        let mut row_at_index = board.slice_mut(s![row_index, ..]);
        let values: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Parse Error"))
            .collect();
        row_at_index.assign(&Array::from(values));

        if row_index == 4 {
            // Save board when on last line
            boards.push(board);
            board = Array2::zeros((5, 5));
        }
    }
    boards
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut input_values = input.lines();

    // Get the draws from the first line
    let draws_str = input_values.next().expect("");
    let draws: Vec<usize> = draws_str
        .split(',')
        .map(|x| x.parse().expect("Parse Error"))
        .collect();

    // Get the boards
    let boards = get_boards(&mut input_values);

    // Mark boards
    let mut winning_board = None;
    let mut draw_seq: Vec<usize> = Vec::new();
    'outer_1: for draw in &draws {
        draw_seq.push(*draw);
        for board in boards.iter() {
            if is_bingo(board, &draw_seq) {
                winning_board = Some(board);
                break 'outer_1;
            }
        }
    }

    let last_pick = draw_seq.last().expect("");
    let unmarked_sum: usize = winning_board
        .expect("")
        .iter()
        .filter(|x| !draw_seq.contains(x))
        .sum();

    // Part 1:
    // Get the sum of all unmarked numbers and multiply that sum by the number that was just called
    println!("The final score is {}", last_pick * unmarked_sum);

    // Part 2:
    // Figure out which board will win last
    let mut last_board = None;
    let mut winning_indices: Vec<usize> = Vec::new();
    let mut draw_seq: Vec<usize> = Vec::new();
    'outer: for draw in &draws {
        draw_seq.push(*draw);
        for (index, board) in boards.iter().enumerate() {
            // When there is a bing on a board that has not already won
            if is_bingo(board, &draw_seq) && !winning_indices.contains(&index) {
                // Add board to winning boards to skip next time
                winning_indices.push(index);
                if winning_indices.len() == boards.len() {
                    // Only one board left, use as the last board
                    last_board = Some(board.clone());
                    break 'outer;
                }
            }
        }
    }

    let last_pick = draw_seq.last().expect("");
    let unmarked_sum: usize = last_board
        .expect("")
        .into_iter()
        .filter(|x| !draw_seq.contains(x))
        .sum();

    // Get the sum of all unmarked numbers and multiply that sum by the number that was just called
    println!("The final score is {}", last_pick * unmarked_sum);
}
