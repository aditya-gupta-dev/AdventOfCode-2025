use std::fs;

struct Solution { }

impl Solution {
    fn part_one(file_name: &String) -> i32 { 
        let data: String = String::from_utf8(fs::read(file_name).unwrap()).unwrap(); 

        23 
    }
} 

fn main() { 
    let file_name: String = "input-seven.txt".to_string(); 
    println!("part_one = {}", Solution::part_one(&file_name)); 

}
