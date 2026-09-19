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

    fn part_two(file_name: &String) -> i128 {
        let content = fs::read_to_string(file_name).expect("Failed to read input file");

        let grid: Vec<Vec<char>> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.chars().collect())
            .collect();

        if grid.is_empty() {
            return 0;
        }

        let num_rows = grid.len();

        let mut timeline_counts: std::collections::HashMap<usize, i128> =
            std::collections::HashMap::new();
        let mut start_row = 0;

        'find_start: for (r, row) in grid.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    timeline_counts.insert(c, 1);
                    start_row = r;
                    break 'find_start;
                }
            }
        }

        for r in (start_row + 1)..num_rows {
            if timeline_counts.is_empty() {
                break;
            }

            let mut next_timeline_counts: std::collections::HashMap<usize, i128> =
                std::collections::HashMap::new();

            for (&col, &count) in &timeline_counts {
                let cell = grid[r].get(col).copied().unwrap_or('.');

                if cell == '^' {
                    if col > 0 {
                        *next_timeline_counts.entry(col - 1).or_insert(0) += count;
                    }
                    *next_timeline_counts.entry(col + 1).or_insert(0) += count;
                } else {
                    *next_timeline_counts.entry(col).or_insert(0) += count;
                }
            }

            timeline_counts = next_timeline_counts;
        }

        timeline_counts.values().sum()
    }
}

fn main() {
    let file_name: String = "input-seven.txt".to_string();

    println!("part_one = {}", Solution::part_one(&file_name));
    println!("part_two = {}", Solution::part_two(&file_name));
}
