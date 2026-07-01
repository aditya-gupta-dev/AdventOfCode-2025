use std::{char, fs, usize};

struct Solution {}

struct Ind {
    x: i32,
    y: i32,
}

fn get_adjacent_indicies(x: i32, y: i32) -> Vec<Ind> {
    let mut res = Vec::new();
    res.push(Ind { x: x - 1, y: y - 1 });
    res.push(Ind { x: x, y: y - 1 });
    res.push(Ind { x: x + 1, y: y - 1 });
    res.push(Ind { x: x - 1, y: y });
    res.push(Ind { x: x + 1, y: y });
    res.push(Ind { x: x - 1, y: y + 1 });
    res.push(Ind { x: x, y: y + 1 });
    res.push(Ind { x: x + 1, y: y + 1 });
    return res;
}

fn is_valid_index(x: i32, y: i32, limits: &Ind) -> bool {
    if x < 0 || y < 0 { 
        return false; 
    }
    if x >= limits.x || y >= limits.y { 
        return false; 
    }
    true
}

impl Solution {
    fn part_one(file_name: String) -> i32 {
        let data: String = String::from_utf8(fs::read(file_name).unwrap()).unwrap();
        let mut grid: Vec<Vec<char>> = Vec::new();
        let paper: char = '@';
        let mut forkable = 0;

        for line in data.lines() {
            let mut row: Vec<char> = Vec::new();
            for item in line.chars() {
                row.push(item);
            }
            grid.push(row);
        }

        let limits: Ind = Ind {
            x: grid[0].len() as i32,
            y: grid.len() as i32,
        };

        for (y, _) in grid.iter().enumerate() {
            for (x, _) in grid.iter().enumerate() {
                if grid[y][x] == paper {
                    let adjacent_indicies = get_adjacent_indicies(x as i32, y as i32);
                    let mut adjacent_paper_count = 0;
                    for adj_ind in adjacent_indicies {
                        if is_valid_index(adj_ind.x, adj_ind.y, &limits) {
                            if grid[adj_ind.y as usize][adj_ind.x as usize] == paper {
                                adjacent_paper_count += 1;
                            }
                        }
                    }

                    if adjacent_paper_count < 4 { 
                        forkable += 1; 
                    }
                }
            }
        }

        forkable
    }

}


fn main() {
    let file_name: String = String::from("input-four.txt");
    println!("{}", Solution::part_one(file_name))
}
