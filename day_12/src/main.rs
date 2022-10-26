// Day 12: https://adventofcode.com/2021/day/12

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn get_path_count(
    curr_path: &[&str],
    path_count: &mut usize,
    edges_map: &HashMap<&str, Vec<&str>>,
    can_visit_twice: bool,
) -> usize {
    // Get the last node in the current path to find the next targets in the path
    let source = curr_path.last().unwrap();
    let targets = edges_map.get(source).unwrap();

    for target in targets {
        if *target == "end" {
            // Increase path count when reaching the end
            *path_count += 1;
        } else if target.chars().all(|c| c.is_lowercase()) && curr_path.contains(target) {
            // Check if cave name is lowercase (small cave) and has already been seen
            if can_visit_twice {
                let mut new_path = curr_path.to_owned();
                new_path.push(target);
                // Set visit twice to false for the next small caves on this path
                get_path_count(&new_path, path_count, edges_map, false);
            } else {
                // Skip path as we cannot visit a small cave twice
                continue;
            }
        } else {
            // Keep going further with this path
            let mut new_path = curr_path.to_owned();
            new_path.push(target);
            get_path_count(&new_path, path_count, edges_map, can_visit_twice);
        }
    }
    *path_count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file.");

    // Parse edges
    let edges: Vec<(&str, &str)> = input
        .lines()
        .map(|x| x.splitn(2, '-').collect_tuple().unwrap())
        .collect();

    // Create dict of source <-> targets
    let mut edges_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (node_1, node_2) in edges {
        // Note: "start" can only be a key and "end" cannot be a key
        for (source, target) in [(node_1, node_2), (node_2, node_1)] {
            if source != "end" && target != "start" {
                let target_pairs = edges_map.entry(source).or_insert(Vec::new());
                target_pairs.push(target);
            }
        }
    }

    // Part 1: small caves can only be visited once
    let path_count = get_path_count(&["start"], &mut 0, &edges_map, false);
    println!("Part 1: All paths count: {:?}", path_count);

    // Part 2: a single small cave may be visited twice
    let path_count = get_path_count(&["start"], &mut 0, &edges_map, true);
    println!("Part 2: paths count: {:?}", path_count);
}
