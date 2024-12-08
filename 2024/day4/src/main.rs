use std::fs::read_to_string;

fn part_one(grid: &[&str]) -> Vec<(usize, usize, String)> {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
    ];

    let mut results = Vec::new();
    let word = "XMAS";
    let word_len = word.len();

    for (i, &line) in grid.iter().enumerate() {
        for (j, _) in line.chars().enumerate() {
            for &(dx, dy) in &directions {
                if let Some(found_word) = check_direction(grid, i, j, dx, dy, word) {
                    results.push((i, j, found_word));
                }
            }
        }
    }

    results
}

fn check_direction(
    grid: &[&str],
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
    word: &str,
) -> Option<String> {
    let mut x = start_x as isize;
    let mut y = start_y as isize;
    let mut found_word = String::new();

    for ch in word.chars() {
        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
            return None;
        }
        if grid[x as usize].chars().nth(y as usize) != Some(ch) {
            return None;
        }
        found_word.push(ch);
        x += dx;
        y += dy;
    }

    Some(found_word)
}

fn part_two(grid: &[&str]) -> Vec<(usize, usize, String)> {
    let mut results = Vec::new();
    let word = "MAS";

    // Check each position as the center of potential X
    for (i, &line) in grid.iter().enumerate() {
        // Skip first and last row since they can't be centers of an X
        if i == 0 || i >= grid.len() - 1 {
            continue;
        }

        for (j, _) in line.chars().enumerate() {
            // Skip first and last column since they can't be centers of an X
            if j == 0 || j >= line.len() - 1 {
                continue;
            }

            // Check upper-left to lower-right diagonal
            let ul_lr = check_direction(grid, i - 1, j - 1, 1, 1, word)
                .or_else(|| check_direction(grid, i - 1, j - 1, 1, 1, "SAM"));

            // Check upper-right to lower-left diagonal
            let ur_ll = check_direction(grid, i - 1, j + 1, 1, -1, word)
                .or_else(|| check_direction(grid, i - 1, j + 1, 1, -1, "SAM"));

            // If we found both diagonals forming an X
            if let (Some(d1), Some(d2)) = (ul_lr, ur_ll) {
                results.push((i, j, format!("{}{}", d1, d2)));
            }
        }
    }

    results
}

fn main() {
    let content = read_to_string("input.txt").expect("Failed to read file");
    let grid: Vec<&str> = content.lines().collect();
    println!("Part one: {:?}", part_one(&grid).len());
    println!("Part two: {:?}", part_two(&grid).len());
}
