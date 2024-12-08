use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(content: &str) -> HashMap<char, Vec<Point>> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    for (y, line) in content.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    antennas
}

fn calculate_antinode(p1: Point, p2: Point) -> Option<Point> {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    Some(Point {
        x: p1.x - dx,
        y: p1.y - dy,
    })
}

fn is_within_bounds(point: &Point, max_x: i32, max_y: i32) -> bool {
    point.x >= 0 && point.x < max_x && point.y >= 0 && point.y < max_y
}

fn are_points_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    let area = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    area == 0
}

fn find_antinodes(content: &str) -> usize {
    let antennas = parse_input(content);
    let height = content.lines().count() as i32;
    let width = content.lines().next().unwrap().len() as i32;
    let mut antinodes = HashSet::new();

    for (_, positions) in antennas.iter() {
        if positions.len() < 2 {
            continue;
        }

        for y in 0..height {
            for x in 0..width {
                let current_point = Point { x, y };

                for i in 0..positions.len() {
                    for j in (i + 1)..positions.len() {
                        if are_points_collinear(&positions[i], &positions[j], &current_point) {
                            antinodes.insert(current_point);
                            break;
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let content = read_to_string("input.txt").expect("Failed to read file");
    if content.trim().is_empty() {
        eprintln!("Error: Input file is empty");
        return;
    }
    let result = find_antinodes(&content);
    println!("Number of unique antinode locations: {}", result);
}
