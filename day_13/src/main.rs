use itertools::{enumerate, Itertools};
use ndarray::{s, Array2};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // Get (x,y) coordinates and fold instructions
    let mut all_coordinates: Vec<(usize, usize)> = Vec::new();
    let mut fold_instructions_section = false;
    let mut fold_instructions: Vec<(&str, usize)> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            // Switch from coords to fold instructions when encountering the empty line
            fold_instructions_section = true;
        } else if fold_instructions_section {
            // Add fold instructions, skipping "fold along " part
            let (axis, position) = line[11..line.len()].splitn(2, '=').collect_tuple().unwrap();
            fold_instructions.push((axis, position.parse::<usize>().unwrap()))
        } else {
            // Add coordinates
            let (x, y) = line
                .splitn(2, ',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            all_coordinates.push((x, y))
        }
    }

    // Set coordinates in a 2d matrix
    let max_x = all_coordinates
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .0;
    let max_y = all_coordinates
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1;

    let mut board: Array2<u8> = Array2::zeros((max_x + 1, max_y + 1));

    // Set the visible dots (1s) from the coordinates
    for coord in all_coordinates {
        let value = board.get_mut(coord).unwrap();
        *value = 1;
    }

    // fold the board multiple times
    for (index, (fold_axis, fold_index)) in enumerate(&fold_instructions) {
        // 1. get 2 matrices by slicing todo: use slice_mult
        let axis = {
            if *fold_axis == "x" {
                ndarray::Axis(0)
            } else {
                ndarray::Axis(1)
            }
        };

        let indices_1 = ndarray::Slice {
            start: 0,
            end: Some(*fold_index as isize),
            step: 1,
        };
        let mut slice_1 = board.slice_axis(axis, indices_1).into_owned();

        // reverse slice the second board (Note: step = -1)
        let indices_2 = ndarray::Slice {
            start: (fold_index + 1) as isize,
            end: None,
            step: -1,
        };
        let slice_2 = board.slice_axis(axis, indices_2);

        // Merge the boards by taking the max value of each (OR)
        for (coord, value_1) in slice_1.indexed_iter_mut() {
            let value_2 = slice_2.get(coord).unwrap();
            *value_1 |= *value_2
        }

        // Part 1: count the dots after 1 fold
        if index == 0 {
            let dots_visible = slice_1.iter().filter(|&&x| x == 1).count();
            println!("Part 1: There are {} dots visible", dots_visible);
        }

        board = slice_1;
    }
    // Part 2: show the final instructions letter by letter (5 x 5 windows)
    println!("The instructions are:");
    let end = board.len_of(ndarray::Axis(0));
    for slice_ind in (0..end).step_by(5) {
        let slice = board.slice(s![slice_ind..(slice_ind + 5), ..]);
        // Transpose before printing as the x and y are interchanged from the example
        println!("{}", slice.reversed_axes());
    }
}
