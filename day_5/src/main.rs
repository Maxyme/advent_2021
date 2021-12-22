use std::cmp::max;
use std::fs;
use std::iter::zip;
use itertools::{Itertools, sorted};
use ndarray::Array2;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize
}
#[derive(Debug)]
struct Line {
    start: Position,
    end: Position
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let input_values = input.lines();

    // Lines are x1,y1 -> x2,y2, convert to list of Line structs
    let mut lines: Vec<Line> = Vec::new();

    // Keep the highest number to make a 2d matrix of that size
    let mut length = 0;

    for row in input_values {
        let (start, end) = row.splitn(2, " -> ").collect_tuple().expect("");
        let start_position: (&str, &str) = start.splitn(2, ',').collect_tuple().expect("");
        let end_position: (&str, &str) = end.splitn(2, ',').collect_tuple().expect("");
        let line = Line {
            start: Position { x: start_position.0.parse().expect(""), y: start_position.1.parse().expect("") },
            end: Position { x: end_position.0.parse().expect(""), y: end_position.1.parse().expect("") },
        };
        let vals = vec![line.start.x, line.start.y, line.end.x, line.end.x];
        let max_val = vals.into_iter().reduce(max).expect("");
        if max_val > length {
            length = max_val;
        }
        lines.push(line);
    }

    // Part 1. Only consider horizontal and vertical lines
    let simple_lines: Vec<&Line> = lines.iter().filter(|line| line.start.x == line.end.x || line.start.y == line.end.y).collect();

    // Build a matrix with the max number of lines
    let mut diagram: Array2<usize> = Array2::zeros((length + 1, length + 1));

    for line in &simple_lines {
        // Update the diagram
        let (start_x, end_x) = sorted(vec![line.start.x, line.end.x]).collect_tuple().expect("");
        let (start_y, end_y) = sorted(vec![line.start.y, line.end.y]).collect_tuple().expect("");

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let value = diagram.get_mut((y,x)).unwrap();
                *value += 1;
            }
        }
    }


    // Determine the number of points where at least two lines overlap
    let count_over_2 = diagram.iter().filter(|x| **x >= 2_usize).count();
    println!("Part 1: {:?}", count_over_2);

    // Part 2: same as above but counting diagonal lines, so lines where diff x == diff y
    let diag_lines = lines.iter().filter(|line| (line.start.y as i32 - line.end.y as i32).abs() == (line.start.x as i32 - line.end.x as i32).abs());

    // Update the diagram with diagonal lines
    for line in diag_lines {
        let xs: Vec<usize>  = if line.start.x < line.end.x {
            (line.start.x..=line.end.x).collect()
        } else {
            (line.end.x..=line.start.x).rev().collect()
        };
        let ys: Vec<usize>  = if line.start.y < line.end.y {
            (line.start.y..=line.end.y).collect()
        } else {
            (line.end.y..=line.start.y).rev().collect()
        };

        for (x, y) in zip(xs, ys) {
            let value = diagram.get_mut((y, x)).unwrap();
            *value += 1;
        }

    }

    // Determine the number of points where at least two lines overlap
    let count_over_2 = diagram.into_iter().filter(|x| *x >= 2_usize).count();
    println!("Part 2 {:?}", count_over_2);

}
