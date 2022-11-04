use ndarray::{s, Array, Array1, Array2};
use std::collections::BinaryHeap;
use std::fs;

const RADIX: u32 = 10;

fn visit_node(
    node: (usize, usize),
    destinations_costs: &mut Array2<usize>,
    queue: &mut BinaryHeap<(usize, usize)>,
    visited: &mut Array2<usize>,
    dist_cost: usize,
) {
    let previously_computed_cost = destinations_costs.get_mut(node).unwrap();
    if dist_cost < *previously_computed_cost {
        // New cost is smaller, update the previous known cost
        *previously_computed_cost = dist_cost;
        queue.push(node);
    } else if visited[node] == 0 {
        // If not visited then add to queue
        queue.push(node);
    }
    visited[node] = 1;
}

fn get_destination_costs(
    num_lines: usize,
    line_len: usize,
    grid: &mut Array2<usize>,
) -> Array2<usize> {
    let src: (usize, usize) = (0, 0);
    let mut heap: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    // Add src to queue
    heap.push(src);

    // Create a map of minimum destination costs for each node (coord) in the array from 0,0
    let mut destinations_costs: Array2<usize> = Array2::zeros(grid.dim());
    destinations_costs.mapv_inplace(|_| usize::MAX);
    // Set src known cost of 0
    destinations_costs[src] = 0;

    //let mut visited: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut visited: Array2<usize> = Array2::zeros(grid.dim());
    while let Some(node) = heap.pop() {
        // Get the min cost to get to that node from the start
        let node_dist_cost = destinations_costs[node];

        // Check each possible pixel neighbor
        if node.0 > 0 {
            let left_neigh = (node.0 - 1, node.1);
            let dist_cost = node_dist_cost + grid[left_neigh];
            visit_node(
                left_neigh,
                &mut destinations_costs,
                &mut heap,
                &mut visited,
                dist_cost,
            );
        }
        if node.0 < line_len - 1 {
            let right_neigh = (node.0 + 1, node.1);
            let dist_cost = node_dist_cost + grid[right_neigh];
            visit_node(
                right_neigh,
                &mut destinations_costs,
                &mut heap,
                &mut visited,
                dist_cost,
            );
        }
        if node.1 > 0 {
            let up_neigh = (node.0, node.1 - 1);
            let dist_cost = node_dist_cost + grid[up_neigh];
            visit_node(
                up_neigh,
                &mut destinations_costs,
                &mut heap,
                &mut visited,
                dist_cost,
            );
        }
        if node.1 < num_lines - 1 {
            let down_neigh = (node.0, node.1 + 1);
            let dist_cost = node_dist_cost + grid[down_neigh];
            visit_node(
                down_neigh,
                &mut destinations_costs,
                &mut heap,
                &mut visited,
                dist_cost,
            );
        }
    }
    destinations_costs
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

    let destinations_costs = get_destination_costs(num_lines, line_len, &mut grid);

    let destination = (grid.dim().0 - 1, grid.dim().1 - 1);
    let destination_min_cost = destinations_costs.get(destination).unwrap();
    println!("Part 1: {:?}", destination_min_cost);

    // Part 2: Extend the grid to make a 5x5 with values += 1
    let mut new_grid: Array2<usize> = Array2::default((num_lines * 5, line_len * 5));
    for j in 0..5 {
        for i in 0..5 {
            let slice = s![
                i * line_len..(i + 1) * line_len,
                j * num_lines..(j + 1) * num_lines
            ];
            let mut matrix = new_grid.slice_mut(slice);
            matrix.assign(&grid.clone());
            matrix.mapv_inplace(|x| {
                let sum = x + i + j;
                // Wrap around to 1 when sum >= 10
                if sum < 10 {
                    sum
                } else {
                    (sum % 10) + 1
                }
            });
        }
    }

    let extended_destination = (new_grid.dim().0 - 1, new_grid.dim().1 - 1);
    let extended_destinations_costs =
        get_destination_costs(num_lines * 5, line_len * 5, &mut new_grid);
    let destination_min_cost = extended_destinations_costs
        .get(extended_destination)
        .unwrap();
    println!("Part 2: {:?}", destination_min_cost);
}
