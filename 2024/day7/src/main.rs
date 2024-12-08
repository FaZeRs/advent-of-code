use std::fs::read_to_string;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn parse_line(line: &str) -> Equation {
    let parts: Vec<&str> = line.split(":").collect();
    let test_value = parts[0].trim().parse().unwrap();
    let numbers: Vec<i64> = parts[1]
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    Equation {
        test_value,
        numbers,
    }
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn evaluate(numbers: &[i64], operators: &[Operator]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            Operator::Add => result += numbers[i + 1],
            Operator::Multiply => result *= numbers[i + 1],
            Operator::Concatenate => {
                result = format!("{}{}", result, numbers[i + 1])
                    .parse()
                    .unwrap_or(i64::MAX);
            }
        }
    }
    result
}

fn generate_operator_combinations(len: usize) -> Vec<Vec<Operator>> {
    let mut result = Vec::new();
    let total_combinations = 3_i32.pow(len as u32);

    for i in 0..total_combinations {
        let mut combination = Vec::new();
        for j in 0..len {
            match (i / 3_i32.pow(j as u32)) % 3 {
                0 => combination.push(Operator::Add),
                1 => combination.push(Operator::Multiply),
                2 => combination.push(Operator::Concatenate),
                _ => unreachable!(),
            }
        }
        result.push(combination);
    }
    result
}

fn can_make_test_value(equation: &Equation) -> bool {
    let operator_count = equation.numbers.len() - 1;
    let combinations = generate_operator_combinations(operator_count);

    combinations
        .iter()
        .any(|ops| evaluate(&equation.numbers, ops) == equation.test_value)
}

fn main() {
    let content = read_to_string("input.txt").expect("Failed to read file");

    let result: i64 = content
        .lines()
        .map(parse_line)
        .filter(|eq| can_make_test_value(eq))
        .map(|eq| eq.test_value)
        .sum();

    println!("Total calibration result: {}", result);
}
