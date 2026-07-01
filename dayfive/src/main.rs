use std::fs;

struct Solution {}

struct IngredientRange {
    start: i128,
    end: i128,
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
            println!("parsing: {}", entry);
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
}

fn main() {
    let file_name = String::from("input.txt");
    println!("part_one: {}", Solution::part_one(&file_name));
}
