use regex::Regex;
use std::fs::read_to_string;

fn part_one(content: &String) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = Vec::new();
    for (_line_num, line) in content.lines().enumerate() {
        for cap in re.captures_iter(line) {
            let n1 = cap[1].parse::<u32>().unwrap();
            let n2 = cap[2].parse::<u32>().unwrap();
            results.push(n1 * n2);
        }
    }

    results.iter().sum()
}

fn part_two(content: &String) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut results = Vec::new();
    let mut mul_enabled = true;

    for line in content.lines() {
        for cap in re.captures_iter(line) {
            if let Some(_mul_cap) = cap.get(1) {
                if mul_enabled {
                    let n1 = cap[1].parse::<u32>().unwrap();
                    let n2 = cap[2].parse::<u32>().unwrap();
                    results.push(n1 * n2);
                }
            } else if cap.get(0).map_or(false, |m| m.as_str() == "do()") {
                mul_enabled = true;
            } else if cap.get(0).map_or(false, |m| m.as_str() == "don't()") {
                mul_enabled = false;
            }
        }
    }

    results.iter().sum()
}

fn main() {
    let content = read_to_string("input.txt").expect("Failed to read file");
    println!("Part one: {:?}", part_one(&content));
    println!("Part two: {:?}", part_two(&content));
}
