// Day 7: https://adventofcode.com/2021/day/7


use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut positions: Vec<usize> = input.split(',').map(|x| x.parse().expect("")).collect();

    positions.sort_unstable();
    let min_pos = positions[0];
    let max_pos = *positions.last().expect("");

    // Part: 1 compute the fuel needed to align all horizontal positions
    let mut fuel_for_values: HashMap<usize, usize> = HashMap::new();
    for pos in min_pos..=max_pos {
        if fuel_for_values.contains_key(&pos) {
            continue;
        }
        let mut sum_diff = 0;
        for v in positions.iter() {
            let dist = {
                if pos > *v {
                    pos - v
                } else {
                    v - pos
                }
            };
            sum_diff += dist;
        }
        fuel_for_values.insert(pos, sum_diff);
    }

    let fuel = fuel_for_values.values().min().expect("");
    println!("Part 1: {:?}", fuel);

    // Part: 2 compute the fuel needed to align all horizontal positions,
    // but fuel costs are sums of all numbers between
    let mut fuel_for_values: HashMap<usize, usize> = HashMap::new();
    for pos in min_pos..=max_pos {
        if fuel_for_values.contains_key(&pos) {
            continue;
        }
        let mut sum_diff = 0;
        for v in positions.iter() {
            let fuel_for_dist: usize = {
                if pos > *v {
                    let n = pos -v;
                    n * (n + 1) / 2
                    // Note this is much slower
                    //(0..=pos -v).sum()
                } else {
                    let n = v - pos;
                    n * (n + 1) / 2

                    //(0..=v-pos).sum()
                }
            };

            sum_diff += fuel_for_dist;
        }
        fuel_for_values.insert(pos, sum_diff);
    }

    let fuel = fuel_for_values.values().min().expect("");
    println!("Part 2: {:?}", fuel);
}
