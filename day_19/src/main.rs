use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Sub;
extern crate nalgebra as na;
use na::Vector3;

// Todo: replace point by Vector3 from nalgebra
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // Parse the list of beacons per scanner in a hashmap
    let scanners_beacons = parse_scanner_positions(input);

    // Get all beacons position relative from scanner 0
    let (from_beacon_0, distances) = get_beacons(&scanners_beacons);

    // Part 1: How many beacons are there?
    println!("Part 1: {}", from_beacon_0.len());

    // Part 2: What is the largest Manhattan distance between any two scanners?
    let mut highest_manhattan = 0;
    for (i_1, p_1) in distances.iter().enumerate() {
        for (i2, p_2) in distances.iter().enumerate() {
            if i_1 == i2 {
                continue;
            }
            let man = (p_1.x - p_2.x).abs() + (p_1.y - p_2.y).abs() + (p_1.z - p_2.z).abs();
            if man > highest_manhattan {
                highest_manhattan = man;
            }
        }
    }
    println!("Part 2: {}", highest_manhattan);
}

fn get_all_orientations(points: &Vec<Point>) -> Vec<Vec<Point>> {
    // Todo: return an iterable instead
    let mut all_rotated_points: Vec<Vec<Point>> = Vec::new();
    for dir in (0..3).permutations(2) {
        // Todo: use rotation3 from nalgebra
        for x_sign in [-1, 1] {
            for y_sign in [-1, 1] {
                let mut x = Vector3::zeros();
                x[dir[0]] = x_sign;
                let mut y = Vector3::zeros();
                y[dir[1]] = y_sign;
                let z = x.cross(&y);
                let mut rotated_points: Vec<Point> = Vec::new();
                for point in points {
                    let vec = Vector3::new(point.x, point.y, point.z);
                    let p = Point {
                        x: x.dot(&vec),
                        y: y.dot(&vec),
                        z: z.dot(&vec),
                    };
                    rotated_points.push(p);
                }
                all_rotated_points.push(rotated_points)
            }
        }
    }
    all_rotated_points
}

/// Get beacons and scanner distances from the first scanner
fn get_beacons(scanners_beacons: &HashMap<usize, Vec<Point>>) -> (HashSet<Point>, Vec<Point>) {
    // Store beacon positions as seen by the first scanner
    let mut beacon_positions: HashSet<Point> = HashSet::new();
    let (scanner, beacons) = scanners_beacons.iter().next().unwrap();
    beacon_positions.extend(beacons);

    let mut seen_scanners: HashSet<usize> = HashSet::new();
    seen_scanners.insert(*scanner);

    let mut scanner_distances: Vec<Point> = Vec::new();
    scanner_distances.push(Point{x:0, y:0, z:0});

    while seen_scanners.len() < scanners_beacons.len() {
        for (scanner,  beacons) in scanners_beacons.iter() {
            if seen_scanners.contains(&scanner) {
                continue;
            }
            for rotated_points in get_all_orientations(beacons) {
                let mut dist_frequency: HashMap<Point, u32> = HashMap::new();
                for r_p in &rotated_points {
                    for b_p in &beacon_positions {
                        let dist = *b_p - *r_p;
                        *dist_frequency.entry(dist).or_insert(0) += 1;
                    }
                }
                let (translation, freq) =
                    dist_frequency.iter().max_by_key(|entry| entry.1).unwrap();
                if freq >= &12 {
                    let translated_points: _ =
                        rotated_points.iter().map(|&x| x + *translation);
                    seen_scanners.insert(*scanner);
                    beacon_positions.extend(translated_points);
                    scanner_distances.push(*translation);
                    break;
                }
            }
        }
    }

    (beacon_positions, scanner_distances)
}
fn parse_scanner_positions(input: String) -> HashMap<usize, Vec<Point>> {
    let mut scanners_positions: HashMap<usize, Vec<Point>> = HashMap::new();
    let mut scanner_header = true; // First line is a scanner header
    let mut current_scanner = 0;
    for line in input.lines() {
        if scanner_header {
            scanner_header = false;
            current_scanner = line
                .strip_prefix("--- scanner ")
                .unwrap()
                .strip_suffix(" ---")
                .unwrap()
                .parse::<usize>()
                .unwrap();
        } else if line.is_empty() {
            // Empty lines split scanners
            scanner_header = true;
        } else {
            let line_coords: Vec<i32> =
                line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            let coord = Point {
                x: line_coords[0],
                y: line_coords[1],
                z: line_coords[2],
            };
            let coords = scanners_positions.entry(current_scanner).or_default();
            coords.push(coord);
        }
    }
    scanners_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test exploding snailfish number
    #[test]
    fn test_get_all_orientations() {
        // 649,640,665
        let beacons = vec![Point {
            x: 649,
            y: 640,
            z: 665,
        }];
        let orientations = get_all_orientations(&beacons);
        println!("{:?}", orientations);
        assert_eq!(orientations.len(), 24);
    }
}
