use std::fs::read_to_string;

fn read_input() -> Vec<Vec<i32>> {
    let content = read_to_string("input.txt").expect("Failed to read file");

    let input: Vec<Vec<_>> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().expect("Failed to parse number"))
                .collect()
        })
        .collect();

    input
}

fn is_all_increasing_or_decreasing(levels: &Vec<i32>) -> bool {
    let first_diff = levels[1] - levels[0];
    let is_increasing = first_diff > 0;
    for j in 1..levels.len() - 1 {
        let current_diff = levels[j + 1] - levels[j];
        if (is_increasing && current_diff <= 0) || (!is_increasing && current_diff >= 0) {
            return false;
        }
    }
    true
}

fn is_level_safe_part(levels: &Vec<i32>) -> bool {
    for j in 0..levels.len() - 1 {
        let diff = (levels[j] - levels[j + 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    if levels.len() > 1 {
        return is_all_increasing_or_decreasing(levels);
    }
    true
}

fn is_level_safe_part_two(levels: &Vec<i32>) -> bool {
    if is_level_safe_part(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified_levels: Vec<i32> = levels.clone();
        modified_levels.remove(i);

        if modified_levels.len() > 0 && is_level_safe_part(&modified_levels) {
            return true;
        }
    }
    false
}

fn part_one(input: Vec<Vec<i32>>) {
    let mut safe_count = 0;
    for i in 0..input.len() {
        let levels = &input[i];
        let is_safe = is_level_safe_part(levels);
        if is_safe {
            safe_count += 1;
        }
    }
    println!("Part One: Safe count: {}", safe_count);
}

fn part_two(input: Vec<Vec<i32>>) {
    let mut safe_count = 0;
    for i in 0..input.len() {
        let levels = &input[i];
        let is_safe = is_level_safe_part_two(levels);
        if is_safe {
            safe_count += 1;
        }
    }
    println!("Part Two: Safe count: {}", safe_count);
}

fn main() {
    let input = read_input();
    part_one(input.clone());
    part_two(input.clone());
}
