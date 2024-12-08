use std::collections::{HashMap, HashSet};

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

fn in_bounds(i: i32, j: i32, n: i32, m: i32) -> bool {
    i < n && i >= 0 && j < m && j >= 0
}

type Row = usize;
type Rng = (usize, usize);

fn get_num_range(i: usize, j: usize, grid: &Vec<&[u8]>) -> (Row, Rng) {
    let row = grid[i];
    let mut k = j;
    while k > 0 && row[k - 1].is_ascii_digit() {
        k -= 1;
    }
    let l = k;
    while k < row.len() && row[k].is_ascii_digit() {
        k += 1;
    }
    let r = k;
    (i, (l, r))
}

fn get_num(grid: &Vec<&[u8]>, range: (Row, Rng)) -> u64 {
    let (l, r) = (((range.1).0), ((range.1).1));
    grid[range.0][l..r]
        .iter()
        .map(|x| *x as char)
        .collect::<String>()
        .parse::<u64>()
        .expect("Should be parasable to u64")
}

fn solve_part_one(data: &str) -> u64 {
    let grid: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();
    let (n, m) = (grid.len(), grid[0].len());
    let mut set = HashSet::new();
    let mut sum = 0;
    for i in 0..n {
        for j in 0..m {
            if !".0123456789".contains(grid[i][j] as char) {
                for (x, y) in DIRS.iter() {
                    let nx = i as i32 + x;
                    let ny = j as i32 + y;
                    if in_bounds(nx, ny, n as i32, m as i32)
                        && grid[nx as usize][ny as usize].is_ascii_digit()
                    {
                        let rng = get_num_range(nx as usize, ny as usize, &grid);
                        if !set.contains(&rng) {
                            set.insert(rng);
                            sum += get_num(&grid, rng);
                        }
                    }
                }
            }
        }
    }
    sum
}

fn solve_part_two(data: &str) -> u64 {
    let grid: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();
    let (n, m) = (grid.len(), grid[0].len());
    let mut map: HashMap<Rng, HashSet<(Row, Rng)>> = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] as char == '*' {
                for (x, y) in DIRS.iter() {
                    let nx = i as i32 + x;
                    let ny = j as i32 + y;
                    if in_bounds(nx, ny, n as i32, m as i32)
                        && grid[nx as usize][ny as usize].is_ascii_digit()
                    {
                        let rng = get_num_range(nx as usize, ny as usize, &grid);
                        map.entry((i, j))
                            .and_modify(|st| {
                                st.insert(rng);
                            })
                            .or_insert_with(|| HashSet::from_iter(vec![rng].iter().cloned()));
                    }
                }
            }
        }
    }
    map.iter()
        .filter(|(_, set)| set.len() == 2)
        .map(|(_, rng)| {
            Vec::from_iter(rng.iter().map(|rng| get_num(&grid, *rng)))
                .iter()
                .product::<u64>()
        })
        .into_iter()
        .sum::<u64>()
}

fn main() {
    let test_1 = include_str!("../input/day3_1.test");
    let prod = include_str!("../input/day3.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_1));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
