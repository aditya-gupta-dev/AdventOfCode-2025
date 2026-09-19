use std::{fs, i128};

struct Solution {}

impl Solution {
    fn part_one(file_name: &String) -> i128 {
        let mut result: Vec<i128> = Vec::new();
        let mut numbers: Vec<Vec<i32>> = Vec::new();
        let data = String::from_utf8(fs::read(file_name).unwrap()).unwrap();

        for line in data.lines().take(data.lines().count() - 1) {
            numbers.push(
                line.split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|item| item.trim().parse::<i32>().unwrap())
                    .collect(),
            );
        }

        let last_line = data
            .lines()
            .skip(data.lines().count() - 1)
            .collect::<String>();

        let operations = last_line.split_whitespace().collect::<Vec<&str>>();

        let num_columns = numbers[0].len();
        for i in 0..num_columns {
            let operate = operations[i];
            let mut row_result: i128 = if operate.trim() == "*" { 1 } else { 0 };
            for row in &numbers {
                let num = row[i];
                if operate.trim() == "*" {
                    row_result = row_result * (num as i128);
                } else {
                    row_result = row_result + (num as i128);
                }
            }
            result.push(row_result);
        }

        result.iter().sum()
    }

    fn part_two(file_name: &String) -> i128 {
        let content = fs::read_to_string(file_name).expect("Failed to read input file");

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return 0;
        }

        let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let grid: Vec<Vec<char>> = lines
            .iter()
            .map(|line| {
                let mut chars: Vec<char> = line.chars().collect();
                chars.resize(max_len, ' ');
                chars
            })
            .collect();

        let num_rows = grid.len();
        let num_cols = max_len;

        let mut problems: Vec<Vec<usize>> = Vec::new();
        let mut current_cols: Vec<usize> = Vec::new();

        for col in 0..num_cols {
            let is_empty = (0..num_rows).all(|row| grid[row][col].is_whitespace());

            if is_empty {
                if !current_cols.is_empty() {
                    problems.push(std::mem::take(&mut current_cols));
                }
            } else {
                current_cols.push(col);
            }
        }
        if !current_cols.is_empty() {
            problems.push(current_cols);
        }

        let mut grand_total: i128 = 0;

        for prob_cols in problems {
            let mut op = '+';
            let mut numbers: Vec<i128> = Vec::new();

            for &col in &prob_cols {
                // Check the last row for the operator (+ or *)
                let last_char = grid[num_rows - 1][col];
                if last_char == '+' || last_char == '*' {
                    op = last_char;
                }

                let mut num_str = String::new();
                for row in 0..(num_rows - 1) {
                    let ch = grid[row][col];
                    if ch.is_ascii_digit() {
                        num_str.push(ch);
                    }
                }

                if let Ok(val) = num_str.parse::<i128>() {
                    numbers.push(val);
                }
            }

            let result = match op {
                '+' => numbers.iter().sum::<i128>(),
                '*' => numbers.iter().product::<i128>(),
                _ => 0,
            };

            grand_total += result;
        }

        grand_total
    }
}

fn main() {
    let file_name: String = String::from("input-six.txt");

    println!("part_one = {}", Solution::part_one(&file_name));
    println!("part_two = {}", Solution::part_two(&file_name));
}
