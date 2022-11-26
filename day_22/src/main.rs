use itertools::Itertools;
use ndarray::{s, Array, Array3};
use regex::Regex;
use std::cmp::{max, min};
use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Slice {
    start: i32,
    end: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Step {
    on: bool,
    x: Slice,
    y: Slice,
    z: Slice,
}

fn get_range(re: &Regex, line: &str) -> Slice {
    let (start, end) = re
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split("..")
        .map(|x| x.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Slice { start, end }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // Parse each line into 3d slices
    let mut steps: Vec<Step> = Vec::new();

    let x_re: Regex = Regex::new(r"x=(.*),y=").unwrap();
    let y_re: Regex = Regex::new(r"y=(.*),z=").unwrap();
    let z_re: Regex = Regex::new(r"z=(.*)").unwrap();

    for line in input.lines() {
        let on = &line[0..2] == "on";
        let x = get_range(&x_re, line);
        let y = get_range(&y_re, line);
        let z = get_range(&z_re, line);
        let slice = Step { on, x, y, z };
        steps.push(slice);
    }

    // Part 1: considering only cubes in the region x=-50..50,y=-50..50,z=-50..50, how many cubes are on
    // Create a 3d (100x100x100) array of zeroes - add 1 as values are inclusive
    let grid = solve(&steps, (101, 101, 101));
    let count = grid.iter().filter(|&x| x == &true).count();
    println!("Part 1: {}", count);

    // Part 2: run for all cuboids
    let count = solve_sum_volumes(&steps);
    println!("Part 2: {}", count);
}

/// Todo: refactor this!
fn solve(steps: &Vec<Step>, shape: (usize, usize, usize)) -> Array3<bool> {
    let mut grid: Array3<bool> = Array::default(shape);

    for step in steps {
        // Update the slices to fit inside the cube
        let slice_x = match get_slice(&mut grid, &step.x, 0) {
            None => continue,
            Some(x) => x,
        };
        let slice_y = match get_slice(&mut grid, &step.y, 1) {
            None => continue,
            Some(x) => x,
        };
        let slice_z = match get_slice(&mut grid, &step.z, 2) {
            None => continue,
            Some(x) => x,
        };

        // Update the cubes following the slices
        let mut slice = grid.slice_mut(s![slice_x, slice_y, slice_z]);
        slice.mapv_inplace(|_| step.on);
    }
    grid
}

/// Todo: refactor this!
fn get_slice(grid: &mut Array3<bool>, slice: &Slice, axis: usize) -> Option<RangeInclusive<i32>> {
    let mut start = slice.start;
    let mut end = slice.end;

    if slice.start < -((grid.shape()[axis] / 2) as i32) {
        start = -((grid.shape()[axis] / 2) as i32);
    }
    if end <= start || start > (grid.shape()[axis] / 2) as i32 {
        return None;
    }
    if slice.end > (grid.shape()[axis] / 2) as i32 {
        end = (grid.shape()[axis] / 2) as i32;
    }
    let boost = (grid.shape()[axis] / 2) as i32;
    Some(start + boost..=end + boost)
}

fn get_intersection(step: &Step, core: &Step) -> Option<Step> {
    let x_start = max(step.x.start, core.x.start);
    let x_end = min(step.x.end, core.x.end);
    if x_start > x_end {
        return None;
    }

    let y_start = max(step.y.start, core.y.start);
    let y_end = min(step.y.end, core.y.end);
    if y_start > y_end {
        return None;
    }

    let z_start = max(step.z.start, core.z.start);
    let z_end = min(step.z.end, core.z.end);
    if z_start > z_end {
        return None;
    }

    let new_step = Step {
        on: !core.on,
        x: Slice {
            start: x_start,
            end: x_end,
        },
        y: Slice {
            start: y_start,
            end: y_end,
        },
        z: Slice {
            start: z_start,
            end: z_end,
        },
    };

    Some(new_step)
}

fn solve_sum_volumes(steps: &Vec<Step>) -> usize {
    let mut cores = Vec::new();
    for step in steps {
        let mut to_add = Vec::new();
        if step.on {
            to_add.push(step.clone());
        }
        for core in &cores {
            let intersection = get_intersection(step, core);
            match intersection {
                Some(x) => to_add.push(x),
                None => continue,
            };
        }
        cores.append(&mut to_add);
    }

    let mut volume_sum: usize = 0;
    for core in cores {
        let volume = (core.x.end - core.x.start + 1) as i64
            * (core.y.end - core.y.start + 1) as i64
            * (core.z.end - core.z.start + 1) as i64;
        if core.on {
            volume_sum += volume as usize;
        } else {
            volume_sum -= volume as usize;
        }
    }
    volume_sum
}
