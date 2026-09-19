use std::fs;

struct Solution {}

impl Solution {
    fn part_one(file_name: &String) -> i128 {
        let content = fs::read_to_string(file_name).expect("Failed to read input file");

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return 0;
        }

        let grid: Vec<Vec<char>> = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.chars().collect())
            .collect();

        let num_rows = grid.len();
        let num_cols = grid[0].len();

        let mut active_cols: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let mut start_row = 0;

        'find_start: for (r, row) in grid.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    active_cols.insert(c);
                    start_row = r;
                    break 'find_start;
                }
            }
        }

        let mut total_splits: i128 = 0;

        for r in (start_row + 1)..num_rows {
            if active_cols.is_empty() {
                break;
            }

            let mut next_cols: std::collections::HashSet<usize> = std::collections::HashSet::new();

            for &col in &active_cols {
                if grid[r][col] == '^' {
                    total_splits += 1;

                    if col > 0 {
                        next_cols.insert(col - 1);
                    }
                    if col + 1 < num_cols {
                        next_cols.insert(col + 1);
                    }
                } else {
                    next_cols.insert(col);
                }
            }

            active_cols = next_cols;
        }

        total_splits
    }
}

fn main() {
    let file_name: String = "input-seven.txt".to_string();
    println!("part_one = {}", Solution::part_one(&file_name));
}
