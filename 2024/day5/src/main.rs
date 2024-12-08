use std::fs::read_to_string;

fn is_valid_order(numbers: &[u32], pairs: &[(u32, u32)]) -> bool {
    for &(x, y) in pairs {
        if numbers.contains(&x) && numbers.contains(&y) {
            let pos_x = numbers.iter().position(|&n| n == x).unwrap();
            let pos_y = numbers.iter().position(|&n| n == y).unwrap();

            if pos_x > pos_y {
                return false;
            }
        }
    }
    true
}

fn part_one(pairs: Vec<(u32, u32)>, number_lists: Vec<Vec<u32>>) -> u32 {
    number_lists
        .iter()
        .filter(|list| is_valid_order(list, &pairs))
        .map(|list| list[list.len() / 2])
        .sum()
}

fn find_valid_order(numbers: &[u32], pairs: &[(u32, u32)]) -> Vec<u32> {
    let mut result = numbers.to_vec();
    let mut changed = true;

    while changed {
        changed = false;
        for i in 0..result.len() {
            for j in i + 1..result.len() {
                // Check if we need to swap these numbers
                let x = result[i];
                let y = result[j];

                // If there's a rule y|x (meaning y should come before x), swap them
                if pairs.contains(&(y, x)) {
                    result.swap(i, j);
                    changed = true;
                }
            }
        }
    }
    result
}

fn part_two(pairs: Vec<(u32, u32)>, number_lists: Vec<Vec<u32>>) -> u32 {
    number_lists
        .iter()
        .filter(|list| !is_valid_order(list, &pairs)) // Only process invalid lists
        .map(|list| {
            let ordered = find_valid_order(list, &pairs);
            ordered[ordered.len() / 2] // Get middle number
        })
        .sum()
}

fn main() {
    let content = read_to_string("input.txt").expect("Failed to read file");
    let sections: Vec<&str> = content.split("\n\n").collect();

    let pairs: Vec<(u32, u32)> = sections[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<u32> = line.split('|').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let number_lists: Vec<Vec<u32>> = sections[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    println!(
        "Part one: {}",
        part_one(pairs.clone(), number_lists.clone())
    );
    println!(
        "Part two: {}",
        part_two(pairs.clone(), number_lists.clone())
    );
}
