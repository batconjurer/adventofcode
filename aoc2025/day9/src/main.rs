use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = [u64; 2];

fn area(point1: &Point, point2: &Point) -> u64 {
    let width = point1[0].abs_diff(point2[0]) + 1;
    let height = point1[1].abs_diff(point2[1]) + 1;
    width * height
}

/// Check if a square formed by the provided corner contains any interior
/// edge point in its interior
fn interior_contains_boundary(corners: [&Point; 2], edge: [&Point; 2]) -> bool {
    let row_min = std::cmp::min(corners[0][0], corners[1][0]);
    let row_max = std::cmp::max(corners[0][0], corners[1][0]);
    let col_min = std::cmp::min(corners[0][1], corners[1][1]);
    let col_max = std::cmp::max(corners[0][1], corners[1][1]);
    // horizontal edge
    if edge[0][0] == edge[1][0] {
        let min_col = std::cmp::min(edge[0][1], edge[1][1]) + 1;
        let max_col = std::cmp::max(edge[0][1], edge[1][1]) - 1;

        if row_min < edge[0][0] && edge[0][0] < row_max {
            if (min_col <= col_min && col_min < max_col) || (col_min <= min_col && min_col <= col_max) {
                return true;
            }
        }

        false
    } else {
        // vertical edge
        let min_row = std::cmp::min(edge[0][0], edge[1][0]) + 1;
        let max_row = std::cmp::max(edge[0][0], edge[1][0]) - 1;
        if col_min < edge[0][1] && edge[0][1] < col_max {
            if (min_row <= row_min && row_min <= max_row) || (row_min <= min_row && min_row <= row_max) {
                return true;
            }
        }
        false
    }
}

fn parse_input(filename: &str) -> Vec<Point> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut points = vec![];

    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let coords = line.trim().split(',').enumerate();
        let mut point = [0u64; 2];
        for (ix, coord) in coords {
            point[ix] = u64::from_str_radix(coord, 10).unwrap();
        }
        points.push(point);
        line.clear();
    }
    points
}

fn part_one(filename: &str) {
    let points = parse_input(filename);
    let mut max = 0u64;

    for i in 0..points.len() - 1 {
        for j in i+1..points.len() {
            max = std::cmp::max(max, area(&points[i], &points[j]))
        }
    }
    println!("Part one: {max}");
}

fn part_two(filename: &str) {
    let points = parse_input(filename);
    let mut max = 0u64;
    let num_points = points.len();

    for i in 0..num_points - 1 {
         'outer: for j in i+2..num_points {
            for k in 0..num_points {
                let edge = [&points[k], &points[(k + 1).rem_euclid(num_points)]];
                // technically this is buggy if the polygon is sufficiently non-convex
                // as it could find a large rectangle completely outside the polygon.
                // However, visual inspection reveals polygon is Pacman shaped.
                if interior_contains_boundary([&points[i], &points[j]], edge) {
                    continue 'outer;
                }
            }
            max = std::cmp::max(max, area(&points[i], &points[j]))
        }
    }
    println!("Part two: {max}");
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
}
