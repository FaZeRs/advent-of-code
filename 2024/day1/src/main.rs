use std::fs::read_to_string;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let content = read_to_string("input.txt").expect("Failed to read file");

    let (left, right): (Vec<_>, Vec<_>) = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().expect("Failed to parse number"))
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    (left, right)
}

fn part_one(left: Vec<i32>, right: Vec<i32>) {
    left.sort();
    right.sort();
    let mut total_distance = 0;
    for i in 0..left.len() {
        total_distance += (left[i] - right[i]).abs();
    }
    println!("Total distance: {}", total_distance);
}

fn part_two(left: Vec<i32>, right: Vec<i32>) {
    let mut similarity_score = 0;
    for i in 0..left.len() {
        let left_num = left[i];
        let count = right.iter().filter(|&num| *num == left_num).count() as i32;
        similarity_score += left_num * count;
    }
    println!("Similarity score: {}", similarity_score);
}

fn main() {
    let (left, right) = read_input();
    part_one(left, right);
    part_two(left, right);
}
