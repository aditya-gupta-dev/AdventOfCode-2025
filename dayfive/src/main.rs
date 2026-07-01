use std::{fs, i64};

struct Solution {}

struct IngredientRange {
    start: i128,
    end: i128,
}

struct IngredientRange64 {
    start: i64,
    end: i64,
}

impl Solution {
    fn part_one(file_name: &String) -> i32 {
        let mut fresh_ingredients_counter: i32 = 0;
        let mut ingredients_list: Vec<IngredientRange> = Vec::new();
        let data: String = String::from_utf8(fs::read(file_name).unwrap()).unwrap();
        let res: Vec<&str> = data.split("\n\n").collect::<Vec<&str>>();

        for enteries in res[0].split("\n").collect::<Vec<&str>>() {
            let entry: Vec<&str> = enteries.split("-").collect::<Vec<&str>>();

            ingredients_list.push(IngredientRange {
                start: entry[0].parse::<i128>().unwrap(),
                end: entry[1].parse::<i128>().unwrap(),
            });
        }

        for entry in res[1].split("\n").collect::<Vec<&str>>() {
            let entry = entry.parse::<i128>().unwrap();

            for ingredient in &ingredients_list {
                if entry >= ingredient.start && entry <= ingredient.end {
                    fresh_ingredients_counter += 1;
                    break;
                }
            }
        }

        fresh_ingredients_counter
    }

    fn part_two(file_name: &String) -> i64 {
        let data: String = String::from_utf8(fs::read(file_name).unwrap()).unwrap();
        let block = data.split("\n\n").next().unwrap();

        let mut ranges: Vec<IngredientRange64> = Vec::new();
        for line in block.lines() {
            let (s, e) = line.split_once('-').unwrap();
            ranges.push(IngredientRange64 {
                start: s.parse::<i64>().unwrap(),
                end: e.parse::<i64>().unwrap(),
            });
        }

        ranges.sort_by_key(|r| r.start);

        let mut total: i64 = 0;
        let mut cur_start = ranges[0].start;
        let mut cur_end = ranges[0].end;
        for r in &ranges[1..] {
            if r.start <= cur_end + 1 {
                cur_end = cur_end.max(r.end);
            } else {
                total += cur_end - cur_start + 1;
                cur_start = r.start;
                cur_end = r.end;
            }
        }
        total += cur_end - cur_start + 1;

        total
    }
}

fn main() {
    let file_name = String::from("input.txt");
    println!("part_one: {}", Solution::part_one(&file_name));
    println!("part_two: {}", Solution::part_two(&file_name));
}
