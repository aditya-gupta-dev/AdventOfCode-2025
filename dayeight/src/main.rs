use std::fs;

struct Dsu {
    parent: Vec<usize>,
    num_components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            num_components: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
            self.num_components -= 1;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

struct Edge {
    dist_sq: i128,
    u: usize,
    v: usize,
}

fn part_two(file_name: &String) -> i128 {
    let content = fs::read_to_string(file_name).expect("Failed to read input file");

    let points: Vec<Point> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let coords: Vec<i64> = line
                .trim()
                .split(',')
                .map(|val| val.parse::<i64>().expect("Invalid coordinate"))
                .collect();
            Point {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    let n = points.len();
    if n <= 1 {
        return 0;
    }

    let num_edges = n * (n - 1) / 2;
    let mut edges = Vec::with_capacity(num_edges);

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = (points[i].x - points[j].x) as i128;
            let dy = (points[i].y - points[j].y) as i128;
            let dz = (points[i].z - points[j].z) as i128;
            let dist_sq = dx * dx + dy * dy + dz * dz;

            edges.push(Edge {
                dist_sq,
                u: i,
                v: j,
            });
        }
    }

    edges.sort_unstable_by_key(|e| e.dist_sq);

    let mut dsu = Dsu::new(n);

    for edge in edges {
        if dsu.union(edge.u, edge.v) && dsu.num_components == 1 {
            let x1 = points[edge.u].x as i128;
            let x2 = points[edge.v].x as i128;
            return x1 * x2;
        }
    }

    0
}
fn main() {
    let file_name: String = "input-eight.txt".to_string();
    println!("part_two {}", part_two(&file_name));
}
