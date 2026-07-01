use std::fs;

struct Solution {}

struct Ind {
    x: i32,
    y: i32,
}

fn get_adjacent_indicies(x: i32, y: i32) -> Vec<Ind> {
    vec![
        Ind { x: x - 1, y: y - 1 },
        Ind { x: x, y: y - 1 },
        Ind { x: x + 1, y: y - 1 },
        Ind { x: x - 1, y: y },
        Ind { x: x + 1, y: y },
        Ind { x: x - 1, y: y + 1 },
        Ind { x: x, y: y + 1 },
        Ind { x: x + 1, y: y + 1 },
    ]
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

fn count_adjacent_paper(grid: &Vec<Vec<char>>, x: i32, y: i32, limits: &Ind) -> i32 {
    let mut count = 0;
    for adj_ind in get_adjacent_indicies(x, y) {
        if is_valid_index(adj_ind.x, adj_ind.y, limits) {
            if grid[adj_ind.y as usize][adj_ind.x as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

impl Solution {
    fn part_one(file_name: String) -> i32 {
        let data: String = String::from_utf8(fs::read(file_name).unwrap()).unwrap();
        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in data.lines() {
            grid.push(line.chars().collect());
        }

        let limits: Ind = Ind {
            x: grid[0].len() as i32,
            y: grid.len() as i32,
        };

        let mut forkable = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '@' {
                    let adjacent_paper_count = count_adjacent_paper(&grid, x as i32, y as i32, &limits);
                    if adjacent_paper_count < 4 { 
                        forkable += 1; 
                    }
                }
            }
        }

        forkable
    }

    fn part_two(file_name: String) -> i32 {
        let data: String = String::from_utf8(fs::read(file_name).unwrap()).unwrap();
        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in data.lines() {
            grid.push(line.chars().collect());
        }

        let limits: Ind = Ind {
            x: grid[0].len() as i32,
            y: grid.len() as i32,
        };

        let mut total_removed = 0;

        loop {
            // Find all removable rolls in current state
            let mut to_remove: Vec<(usize, usize)> = Vec::new();
            
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] == '@' {
                        let adjacent_paper_count = count_adjacent_paper(&grid, x as i32, y as i32, &limits);
                        if adjacent_paper_count < 4 {
                            to_remove.push((y, x));
                        }
                    }
                }
            }

            // If no more rolls can be removed, we're done
            if to_remove.is_empty() {
                break;
            }

            // Remove all removable rolls
            for (y, x) in to_remove {
                grid[y][x] = '.';
                total_removed += 1;
            }
        }

        total_removed
    }
}


fn main() {
    let file_name: String = String::from("input-four.txt");
    println!("Part 1: {}", Solution::part_one(file_name.clone()));
    println!("Part 2: {}", Solution::part_two(file_name));
}
