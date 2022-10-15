// Day 11: https://adventofcode.com/2021/day/11

use ndarray::{s, Array, Array2, Ix};
use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::fs;

const RADIX: u32 = 10;
const STEPS: usize = 1000;

/// Get index neighbors without overflowing the grid sides
fn get_neighbors(index: &(Ix, Ix), side_len: usize) -> HashSet<(usize, usize)> {
    // Check up/down/left/right to see that it's a local minimum
    let min_x_bounds = max(0, index.0 as i32 - 1) as usize;
    let max_x_bounds = min(index.0 + 1, side_len - 1);
    let min_y_bounds = max(0, index.1 as i32 - 1) as usize;
    let max_y_bounds = min(index.1 + 1, side_len - 1);

    let mut neighbors: HashSet<(usize, usize)> = HashSet::new();
    for i in min_x_bounds..=max_x_bounds {
        for j in min_y_bounds..=max_y_bounds {
            if (i, j) != *index {
                neighbors.insert((i, j));
            }
        }
    }
    neighbors
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file.");

    // Read into 2d array
    let num_lines = input.lines().count();
    let line_len = input.lines().next().unwrap().chars().count();
    let mut grid: Array2<usize> = Array::default((num_lines, line_len));

    // Fill the grid with values - Todo: improve this?
    for (index, values) in input.lines().enumerate() {
        let mut row_at_index = grid.slice_mut(s![index, ..]);
        let values: Vec<usize> = values
            .chars()
            .map(|x| x.to_digit(RADIX).expect("Error parsing value") as usize)
            .collect();

        row_at_index.assign(&Array::from(values));
    }

    let mut all_zeros = -1;
    let mut flashes_count = 0;
    for step in 1..=STEPS {
        // 1. Add 1 to every value in the grid
        grid.mapv_inplace(|x| x + 1);

        // Check if any value are now > 9 and add to queue
        let mut deque: VecDeque<(Ix, Ix)> = VecDeque::new();
        for (index, value) in grid.indexed_iter() {
            if value > &9 {
                deque.push_back(index);
            }
        }

        let mut visited: HashSet<(Ix, Ix)> = HashSet::new();

        // If any values are above 9 propagate to neighbors and add new values > 9
        while let Some(deque_index) = deque.pop_front() {
            // Get neighbors that have not already flashed
            let neighbors = get_neighbors(&deque_index, line_len);
            for not_visited_idx in neighbors.difference(&visited) {
                // Increase neighbor value and add to deque if > 9 and not already in deque
                let val = grid.get_mut(*not_visited_idx).unwrap();
                *val += 1;
                if *val > 9 && !deque.contains(not_visited_idx) {
                    deque.push_back(*not_visited_idx);
                }
            }

            // Add index to visited
            visited.insert(deque_index);
            // And set visited value to 0
            let val = grid.get_mut(deque_index).unwrap();
            *val = 0;
        }

        // increase flash count
        flashes_count += visited.len();

        // Part 2: Check if all values are 0 and exit
        if grid.iter().all(|x| *x == 0) {
            all_zeros = step as i32;
            break;
        }
    }
    println!("The total flash count is {}", flashes_count);

    // Part 2 - calculate the first time all flashes happen simultaneously
    println!("Simultaneous flashes happen at step {}", all_zeros);
}
