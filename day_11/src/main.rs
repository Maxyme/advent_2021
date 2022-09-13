// Day 11: https://adventofcode.com/2021/day/11

use std::cmp::{max, min};
use std::fs;
use ndarray::{Array, Array2, s};
const RADIX: u32 = 10;


// fn get_neighbors(index: &(usize, usize), grid: &Array2<usize>) -> Vec<Option<(usize, usize)>> {
//     // Check up/down/left/right to see that it's a local minimum
//     let left = {
//         if index.0 > 0 {
//             Some((index.0 - 1, index.1))
//         } else {
//             None
//         }
//     };
//
//     let right = {
//         if (index.0) < grid.shape()[0] - 1 {
//             Some((index.0 + 1, index.1))
//         } else {
//             None
//         }
//     };
//
//     let top = {
//         if index.1 > 0 {
//             Some((index.0, index.1 - 1))
//         } else {
//             None
//         }
//     };
//
//     let bottom = {
//         if (index.1) < grid.shape()[1] - 1 {
//             Some((index.0, index.1 + 1))
//         } else {
//             None
//         }
//     };
//
//     vec![left, right, top, bottom]
// }

fn main() {
    let input = fs::read_to_string("example.txt").expect("Error reading file.");

    // Read into 2d array
    let num_lines = input.lines().count();
    let line_len = input.lines().next().unwrap().chars().count();
    let mut grid: Array2<usize> = Array::default((num_lines, line_len));

    // Fill the grid with the values
    for (index, values) in input.lines().enumerate() {
        let mut row_at_index = grid.slice_mut(s![index, ..]);
        let values: Vec<usize> = values
            .chars()
            .map(|x| x.to_digit(RADIX).expect("Error parsing value") as usize)
            .collect();
        row_at_index.assign(&Array::from(values));
    }
    println!("{}", grid);
    //grid.gen_range()
    let index_0 = (1_usize, 0_usize);
    //let val_0_0 = max((index_0.0 - 1), 0);
    let val_0_0 = index_0.0.checked_rem(1).unwrap_or(0);
    let val_0_1 = min(index_0.0 + 1, line_len);
    let val_1_0 = index_0.1.checked_rem(1).unwrap_or(0);
    let val_1_1 = min((index_0.1 + 1), line_len);
    let around_1 = grid.slice_mut(s![val_0_0..val_0_1, val_1_0..val_1_1]);
    println!("{:?}", around_1);
    // Part 1: How many total flashes are there after 100 steps

}
