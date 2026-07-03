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
}

fn main() {
    let file_name: String = String::from("input-six.txt");
    println!("part_one = {}", Solution::part_one(&file_name));
}
