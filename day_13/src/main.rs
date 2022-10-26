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
            // todo: figure out why inverse?
            all_coordinates.push((y, x))
        }
    }

    // Set coordinates in a 2d matrix
    let max_x = all_coordinates
        .iter()
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap()
        .0
        + 1; //.filter(|(x,y)| )
    let max_y = all_coordinates
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .1
        + 1; //.filter(|(x,y)| )

    // todo: use bool?
    let mut board: Array2<usize> = Array2::zeros((max_x, max_y));

    // Set the visible dots from the coordinates
    for coord in all_coordinates {
        let v = board.get_mut(coord).unwrap();
        *v = 1;
    }

    // fold the board multiple times
    for (index, (fold_axis, fold_index)) in enumerate(&fold_instructions) {
        // 1. get 2 matrices by slicing
        let axis = {
            if *fold_axis == "x" {
                1
            } else {
                0
            }
        };
        let slice = ndarray::Slice {
            start: 0,
            end: Some(*fold_index as isize),
            step: 1,
        };
        let sliced_board_part_1 = board.slice_axis(ndarray::Axis(axis), slice);
        let slice_2 = ndarray::Slice {
            start: (fold_index + 1) as isize,
            end: None,
            step: 1,
        };
        let mut sliced_board_part_2 = board.slice_axis(ndarray::Axis(axis), slice_2);

        let mut sliced_board_part_1_owned = sliced_board_part_1.into_owned();

        // reverse the second board and merge the boards
        sliced_board_part_2.invert_axis(ndarray::Axis(axis));

        // Merge the boards
        for (coord, _) in sliced_board_part_1.indexed_iter() {
            let value_1 = sliced_board_part_1_owned.get_mut(coord).unwrap();
            let value_2 = sliced_board_part_2.get(coord).unwrap();
            *value_1 = {
                if *value_1 == 1 || *value_2 == 1 {
                    1
                } else {
                    0
                }
            };
        }

        // Part 1: count the dots after 1 fold
        if index == 0 {
            let dots_visible = sliced_board_part_1_owned
                .iter()
                .filter(|&x| *x == 1)
                .count();
            println!("Part 1: There are {} dots visible", dots_visible);
        }

        // Part 2: show the final instructions letter by letter (5 x 5 windows)
        if index == fold_instructions.len() - 1 {
            println!("The instructions are");
            let x = sliced_board_part_1_owned.len_of(ndarray::Axis(1));
            for slice_ind in (0..x).step_by(5) {
                let slice = sliced_board_part_1_owned.slice(s![.., slice_ind..(slice_ind + 5)]);
                println!("{:?}", slice);
            }
        }
        board = sliced_board_part_1_owned;
    }
}
