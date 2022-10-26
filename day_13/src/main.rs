use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("example.txt").expect("Error reading file");

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
            all_coordinates.push((x, y))
        }
    }
    println!("{:?}", all_coordinates);
    println!("{:?}", fold_instructions);
}
