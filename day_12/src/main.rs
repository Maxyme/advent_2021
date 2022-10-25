// Day 12: https://adventofcode.com/2021/day/12

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn get_paths_from_node(curr_path: &[String], all_paths: &mut Vec<Vec<String>>, edges_map: &HashMap<&str, Vec<&str>>, can_visit_twice: bool, visit_counts: &mut HashMap<String, u32>) -> Vec<Vec<String>> {

    let source: &str = curr_path.last().unwrap().as_ref();
    let targets: &Vec<&str> = edges_map.get(source).unwrap();
    for target in targets {
        let cave_name_is_lowercase = target.chars().all(|c| c.is_lowercase());
        if *target == "end" {
            let mut new_path = curr_path.to_owned();
            new_path.push("end".to_string());
            all_paths.push(new_path)
        }
        else if cave_name_is_lowercase && curr_path.contains(&target.to_string()) {
            // small cave has already been seen
            if !can_visit_twice {
                // Skip path
                continue;
            }
            // Add cave to registry of already visited once if no small cave is already visited
            visit_count = visit_counts.get(&target).unwrap();
            if visit_count == 0 {

            }

        } else {
            // Keep going further
            let mut new_path = curr_path.to_owned();
            new_path.push(target.to_string());
            get_paths_from_node(&new_path, all_paths, edges_map, can_visit_twice, visit_counts);
        }
    }
    all_paths.to_vec()
}

fn main() {
    let input = fs::read_to_string("example.txt").expect("Error reading file.");

    // Parse edges and add to dict of source <-> targets
    let edges: Vec<(&str, &str)> = input
        .lines()
        .map(|x| x.splitn(2, '-').collect_tuple().unwrap())
        .collect();

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
    let mut all_paths: Vec<Vec<String>> = Vec::new();
    let start_path = ["start".to_string()].to_vec();
    let mut visit_count = HashMap::new();
    get_paths_from_node(&start_path, &mut all_paths, &edges_map, false, &mut visit_count);
    println!("Part 1: All paths count: {:?}", all_paths.len());

    // Part 2: a single small cave may be visited twice
    let mut all_paths_part_2: Vec<Vec<String>> = Vec::new();
    let mut visit_count = HashMap::new();
    get_paths_from_node(&start_path, &mut all_paths_part_2, &edges_map, true, &mut visit_count);
    println!("Part 2: paths count: {:?}", all_paths_part_2.len());
}
