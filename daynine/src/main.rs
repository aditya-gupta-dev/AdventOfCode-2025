fn part_one(file_name: &'static str) -> i64 {
    let content = std::fs::read_to_string(file_name).expect("failed to load input");

    let points: Vec<(i64, i64)> = content
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect();

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area =
                ((points[i].0 - points[j].0).abs() + 1) * ((points[i].1 - points[j].1).abs() + 1);
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn main() {
    println!("part-one = {}", part_one("input-nine.txt"));
}
