use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, Hash, Default, Eq, PartialEq, Ord, PartialOrd)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

type JunctionBox = Point3D;

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn euclidean_distance(self, other: Point3D) -> u64 {
        let x_square_diff = (self.x - other.x).abs().pow(2);
        let y_square_diff = (self.y - other.y).abs().pow(2);
        let z_square_diff = (self.z - other.z).abs().pow(2);

        ((x_square_diff + y_square_diff + z_square_diff) as f64).sqrt() as u64
    }
}

type Distance = u64;

fn smallest_distance(j_boxes: &Vec<JunctionBox>) -> Vec<((JunctionBox, JunctionBox), Distance)> {
    let mut distances: Vec<((JunctionBox, JunctionBox), Distance)> = vec![];
    let n = j_boxes.len();
    for i in 0..n {
        let p1 = j_boxes[i];
        for j in i + 1..n {
            let p2 = j_boxes[j];
            let dis = p1.euclidean_distance(p2);
            distances.push(((p1, p2), dis));
        }
    }
    distances
}

fn print_adjacency_matrix(matrix: &[Vec<bool>]) {
    print!("    ");

    for i in 0..matrix.len() {
        print!("{:3}", i);
    }
    println!();

    for (i, row) in matrix.iter().enumerate() {
        print!("{:3} ", i);

        for &cell in row {
            print!("{:3}", if cell { 1 } else { 0 });
        }

        println!();
    }
}

fn connected_components(points: &[Point3D], adjacency: &[Vec<bool>]) -> Vec<Vec<Point3D>> {
    let n = points.len();
    let mut visited = vec![false; n];
    let mut components = Vec::new();

    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut component = Vec::new();
        let mut stack = vec![start];

        visited[start] = true;

        while let Some(node) = stack.pop() {
            component.push(points[node]);

            for neighbor in 0..n {
                if adjacency[node][neighbor] && !visited[neighbor] {
                    visited[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }

        components.push(component);
    }

    components
}

fn is_connected(adjacency: &[Vec<bool>]) -> bool {
    let n = adjacency.len();

    if n == 0 {
        return true;
    }

    let mut visited = vec![false; n];
    let mut stack = vec![0];

    visited[0] = true;

    while let Some(node) = stack.pop() {
        for neighbor in 0..n {
            if adjacency[node][neighbor] && !visited[neighbor] {
                visited[neighbor] = true;
                stack.push(neighbor);
            }
        }
    }

    visited.into_iter().all(|v| v)
}

fn solve_part_one(data: &str) -> usize {
    let j_boxes: Vec<JunctionBox> = data
        .lines()
        .map(|line| {
            let point: Vec<i64> = line
                .split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            let (x, y, z) = (point[0], point[1], point[2]);
            JunctionBox::new(x, y, z)
        })
        .collect();
    let n = j_boxes.len();
    let mut adjacency_matrix: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut distances = smallest_distance(&j_boxes);
    distances.sort_by(|a, b| a.1.cmp(&b.1));
    let mut indices_map: HashMap<JunctionBox, usize> = HashMap::new();
    for i in 0..n {
        indices_map.insert(j_boxes[i], i);
    }
    for d in distances.iter().take(1000) {
        adjacency_matrix[*indices_map.get(&d.0.0).unwrap()][*indices_map.get(&d.0.1).unwrap()] =
            true;
        adjacency_matrix[*indices_map.get(&d.0.1).unwrap()][*indices_map.get(&d.0.0).unwrap()] =
            true;
    }
    let mut connected_comps = connected_components(&j_boxes, &adjacency_matrix);
    connected_comps.sort_by(|b, a| a.len().cmp(&b.len()));
    connected_comps.iter().map(|g| g.len()).take(3).product()
}

fn solve_part_two(data: &str) -> usize {
    let j_boxes: Vec<JunctionBox> = data
        .lines()
        .map(|line| {
            let point: Vec<i64> = line
                .split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            let (x, y, z) = (point[0], point[1], point[2]);
            JunctionBox::new(x, y, z)
        })
        .collect();
    let n = j_boxes.len();
    let mut adjacency_matrix: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut distances = smallest_distance(&j_boxes);
    distances.sort_by(|a, b| a.1.cmp(&b.1));

    let mut indices_map: HashMap<JunctionBox, usize> = HashMap::new();
    for i in 0..n {
        indices_map.insert(j_boxes[i], i);
    }
    let mut i = 0;
    while !is_connected(&adjacency_matrix) {
        adjacency_matrix[*indices_map.get(&distances[i].0.0).unwrap()]
            [*indices_map.get(&distances[i].0.1).unwrap()] = true;
        adjacency_matrix[*indices_map.get(&distances[i].0.1).unwrap()]
            [*indices_map.get(&distances[i].0.0).unwrap()] = true;
        i += 1;
    }
    ((distances[i - 1]).0.0.x * (distances[i - 1]).0.1.x) as usize
}

fn main() {
    let test = include_str!("../input/day_8.test");
    let prod = include_str!("../input/day_8.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod: {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test));
    println!("part_2 prod: {:?}", solve_part_two(prod));
}
