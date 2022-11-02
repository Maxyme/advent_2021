use ndarray::{s, Array, Array1, Array2};
use rustc_hash::FxHashSet;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

const RADIX: u32 = 10;

/// Get index neighbors without overflowing the grid sides
fn get_neighbors(index: &(usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    // Get neighbors without diagonals
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if index.0 > 1 {
        neighbors.push((index.0 - 1, index.1));
    }
    if index.0 < width - 1 {
        neighbors.push((index.0 + 1, index.1));
    }
    if index.1 > 1 {
        neighbors.push((index.0, index.1 - 1));
    }
    if index.1 < height - 1 {
        neighbors.push((index.0, index.1 + 1));
    }
    neighbors
}

fn main() {
    // Part 1: find the path with the lowest total risk
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // Read into 2d array
    let num_lines = input.lines().count();
    let line_len = input.lines().next().unwrap().chars().count();
    let mut grid: Array2<usize> = Array::default((num_lines, line_len));
    for (index, values) in input.lines().enumerate() {
        let values: Array1<usize> = values
            .chars()
            .map(|x| x.to_digit(RADIX).expect("Error parsing value") as usize)
            .collect();
        let mut row_at_index = grid.slice_mut(s![index, ..]);
        row_at_index.assign(&values);
    }

    // Create a map of minimum destination costs for each node (coord) in the array from 0,0
    let mut destinations_costs: Array2<usize> = Array2::zeros((num_lines, line_len));
    destinations_costs.mapv_inplace(|_| usize::MAX);

    // First node is a known cost of 0
    let src: (usize, usize) = (0, 0);
    let src_cost = destinations_costs.get_mut(src).unwrap();
    *src_cost = 0;

    //let mut visited : FnvHashSet<(usize, usize)> = FnvHashSet::default();
    let mut visited: FxHashSet<(u8, u8)> = FxHashSet::default();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    // Add Src to queue and set cost
    queue.push_back(src);
    let v = destinations_costs.get_mut(src).unwrap();
    *v = 0;

    while let Some(node) = queue.pop_back() {
        // Get the min cost to get to that node from the start
        let node_dist_cost = *destinations_costs.get(node).unwrap();
        let neighs = get_neighbors(&node, num_lines, line_len);
        for neigh_node in neighs {
            let new_neigh_dist_cost = node_dist_cost + grid[neigh_node]; //).unwrap();
            let previously_computed_cost = destinations_costs.get_mut(neigh_node).unwrap();
            if new_neigh_dist_cost < *previously_computed_cost {
                // new cost is smaller, update the previous known cost
                *previously_computed_cost = new_neigh_dist_cost;
                queue.push_back(neigh_node);
            } else if !visited.contains(&(neigh_node.0 as u8, neigh_node.1 as u8)) {
                // If not visited then add to queue
                queue.push_back(neigh_node);
            };
            // Add to visited
            visited.insert((neigh_node.0 as u8, neigh_node.1 as u8));
        }
    }

    let destination = (num_lines - 1, line_len - 1);
    let destination_min_cost = destinations_costs.get(destination).unwrap();
    println!("{:?}", destination_min_cost);
}
