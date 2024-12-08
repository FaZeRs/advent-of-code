use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_next_position(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0.wrapping_sub(1), pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1.wrapping_sub(1)),
        }
    }
}

fn find_start_position(grid: &Vec<Vec<char>>) -> ((usize, usize), Direction) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return ((i, j), Direction::Up);
            }
        }
    }
    panic!("No starting position found");
}

fn is_valid_position(pos: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    pos.0 < grid.len() && pos.1 < grid[0].len()
}

fn is_blocked(pos: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    is_valid_position(pos, grid) && grid[pos.0][pos.1] == '#'
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let (mut current_pos, mut current_dir) = find_start_position(&grid);
    let mut visited = HashSet::new();
    visited.insert(current_pos);

    while is_valid_position(current_pos, &grid) {
        let next_pos = current_dir.get_next_position(current_pos);
        if is_blocked(next_pos, &grid) {
            current_dir = current_dir.turn_right();
        } else {
            current_pos = next_pos;
            if is_valid_position(current_pos, &grid) {
                visited.insert(current_pos);
            }
        }
    }

    visited.len()
}

fn get_original_path(grid: &Vec<Vec<char>>) -> Vec<((usize, usize), Direction)> {
    let (mut current_pos, mut current_dir) = find_start_position(&grid);
    let mut path = Vec::new();
    let mut seen_states = HashMap::new();

    while is_valid_position(current_pos, &grid) {
        path.push((current_pos, current_dir));

        seen_states.insert((current_pos, current_dir), path.len() - 1);

        let next_pos = current_dir.get_next_position(current_pos);
        if is_blocked(next_pos, &grid) {
            current_dir = current_dir.turn_right();
        } else {
            current_pos = next_pos;
        }
    }

    path
}

fn would_cause_loop(
    pos: (usize, usize),
    original_path: &[((usize, usize), Direction)],
    grid: &Vec<Vec<char>>,
) -> bool {
    let mut is_adjacent = false;
    for &((x, y), _) in original_path {
        if (pos.0 as i32 - x as i32).abs() <= 1 && (pos.1 as i32 - y as i32).abs() <= 1 {
            is_adjacent = true;
            break;
        }
    }
    if !is_adjacent {
        return false;
    }

    let mut current_pos = original_path[0].0;
    let mut current_dir = original_path[0].1;
    let mut visited_states = HashMap::new();
    let mut steps = 0;
    const MAX_STEPS: usize = 10000;

    while is_valid_position(current_pos, &grid) && steps < MAX_STEPS {
        let state = (current_pos, current_dir);

        if let Some(&prev_step) = visited_states.get(&state) {
            if steps - prev_step > 2 {
                return true;
            }
        }
        visited_states.insert(state, steps);

        let next_pos = current_dir.get_next_position(current_pos);
        if is_blocked(next_pos, &grid) || next_pos == pos {
            current_dir = current_dir.turn_right();
        } else {
            current_pos = next_pos;
        }
        steps += 1;
    }

    steps >= MAX_STEPS
}

fn part_two(grid: &Vec<Vec<char>>) -> usize {
    let original_path = get_original_path(&grid);
    let (start_pos, _) = find_start_position(&grid);
    let mut count = 0;

    let mut positions_to_check = HashSet::new();
    for &((x, y), _) in &original_path {
        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                if is_valid_position((new_x, new_y), &grid)
                    && grid[new_x][new_y] == '.'
                    && (new_x, new_y) != start_pos
                {
                    positions_to_check.insert((new_x, new_y));
                }
            }
        }
    }

    for pos in positions_to_check {
        if would_cause_loop(pos, &original_path, &grid) {
            count += 1;
        }
    }

    count
}

fn main() {
    let content = read_to_string("input.txt").expect("Failed to read file");
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}
