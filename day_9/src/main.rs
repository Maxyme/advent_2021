// Day 9: https://adventofcode.com/2021/day/9

use ndarray::{s, Array, Array2};
use std::collections::{HashSet, VecDeque};
use std::fs;

const RADIX: u32 = 10;


fn get_neighbors(index: &(usize, usize), heightmap: &Array2<usize>) -> Vec<Option<(usize, usize)>> {
    // Check up/down/left/right to see that it's a local minimum
    let left = {
        if index.0 > 0 {
            Some((index.0 - 1, index.1))
        } else {
            None
        }
    };

    let right = {
        if (index.0) < heightmap.shape()[0] - 1 {
            Some((index.0 + 1, index.1))
        } else {
            None
        }
    };

    let top = {
        if index.1 > 0 {
            Some((index.0, index.1 - 1))
        } else {
            None
        }
    };

    let bottom = {
        if (index.1) < heightmap.shape()[1] - 1 {
            Some((index.0, index.1 + 1))
        } else {
            None
        }
    };
    vec![left, right, top, bottom]
}

fn value_less_than_neigh(value: usize, index: (usize, usize), heightmap: &Array2<usize>) -> bool {
    // Return true if the value at the index is less than the values of it's neighbors

    let neighbors = get_neighbors(&index, heightmap);
    value
        < *neighbors
            .iter()
            .filter_map(|x| x.as_ref().map(|x| heightmap.get(*x).unwrap()))
            .min()
            .unwrap()
}

fn get_blob_indices(index: &(usize, usize), heightmap: &Array2<usize>) -> HashSet<(usize, usize)> {
    // Find all contiguous blobs (values 0 - 8)
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // Add first value to queue
    queue.push_back(*index);

    while !queue.is_empty() {
        let current_index = queue.pop_front().unwrap();
        // Add to visited
        visited.insert(current_index);
        let neighbors = get_neighbors(&current_index, heightmap);
        for neighbor in neighbors {
            match neighbor {
                Some(neigh_index) => {
                    // Add to queue if value !=9 and not visited already
                    let height = heightmap.get(neigh_index).unwrap();
                    if !visited.contains(&neigh_index) && *height != 9 {
                        queue.push_back(neigh_index);
                    }
                }
                None => continue,
            }
        }
    }

    visited
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // Part 1: Get the sum of the risk levels of all low points on your heightmap

    // Find array shape and create an empty heightmap
    let num_lines = input.lines().count();
    let line_len = input.lines().next().unwrap().chars().count();
    let mut heightmap: Array2<usize> = Array::default((num_lines, line_len));

    // Fill the heightmap with the values
    for (index, values) in input.lines().enumerate() {
        let mut row_at_index = heightmap.slice_mut(s![index, ..]);
        let values: Vec<usize> = values
            .chars()
            .map(|x| x.to_digit(RADIX).expect("Error parsing value") as usize)
            .collect();
        row_at_index.assign(&Array::from(values));
    }

    // Find the indices of all values that are lower than their neighbors
    let sum: usize = heightmap
        .indexed_iter()
        .filter(|(index, x)| value_less_than_neigh(**x, *index, &heightmap))
        .map(|(_, v)| *v + 1)
        .sum();

    // Get the sum of all of these risk levels (value + 1)
    println!("Part 1: {:?}", sum);

    // Part 2: multiply together the sizes of the three largest basins
    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (index, value) in heightmap.indexed_iter() {
        if *value == 9 || visited.contains(&index) {
            // Skip visited values and values of 9 as they are a boundary
            continue;
        }
        let visited_values = get_blob_indices(&index, &heightmap);
        // Add to visited to skip next time
        visited.extend(visited_values.clone());
        // Add the basin size
        basin_sizes.push(visited_values.len());
    }

    // Get the multiplication of the 3 largest values
    basin_sizes.sort_unstable();
    let top_three = basin_sizes.iter().rev().take(3);
    let mult: usize = top_three.product();

    println!("Part 2: {:?}", mult);
}
