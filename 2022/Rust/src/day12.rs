use std::collections::{HashMap, VecDeque};

fn in_bounds(pos: (i32, i32), grid: &Vec<&[u8]>) -> bool {
    let row = grid.len() as i32;
    let col = grid[0].len() as i32;
    pos.0 >= 0 && pos.0 < row && pos.1 >= 0 && pos.1 < col
}

fn is_valid(cur_char: u8, nxt_char: u8) -> bool {
    let mut cur = cur_char as i32;
    let mut nxt = nxt_char as i32;
    if cur == ('S' as i32) {
        cur = 'a' as i32;
    }
    if nxt == ('E' as i32) {
        nxt = 'z' as i32;
    }
    nxt - cur <= 1
}

fn bfs(
    grid: &Vec<&[u8]>,
    target: u8,
    start: (i32, i32),
    path: &mut HashMap<(usize, usize), (usize, usize)>,
) -> (usize, usize) {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let row = grid.len();
    let col = grid[0].len();
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, -1, 0, 1];
    queue.push_front(start);
    let mut vis: Vec<Vec<bool>> = vec![vec![false; col]; row];
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        for i in 0..4 {
            let nx = x + dx[i];
            let ny = y + dy[i];
            if in_bounds((nx, ny), &grid)
                && is_valid(grid[x as usize][y as usize], grid[nx as usize][ny as usize])
                && grid[nx as usize][ny as usize] == target
            {
                queue.clear();
                path.insert((nx as usize, ny as usize), (x as usize, y as usize));
                return (nx as usize, ny as usize);
            }
            if in_bounds((nx, ny), &grid)
                && is_valid(grid[x as usize][y as usize], grid[nx as usize][ny as usize])
                && !vis[nx as usize][ny as usize]
            {
                queue.push_back((nx, ny));
                vis[nx as usize][ny as usize] = true;
                path.insert((nx as usize, ny as usize), (x as usize, y as usize));
            }
        }
    }
    (0, 0)
}

fn get_starting_node(grid: &Vec<&[u8]>) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' as u8 {
                return (i as i32, j as i32);
            }
        }
    }
    unreachable!()
}

fn get_nodes_with_elevation_a(grid: &Vec<&[u8]>) -> Vec<(i32, i32)> {
    let mut ret = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' as u8 || grid[i][j] == 'a' as u8 {
                ret.push((i as i32, j as i32));
            }
        }
    }
    ret
}

fn solve_part_one(data: &str) -> usize {
    let grid: Vec<_> = data.lines().map(|s| s.as_bytes()).collect();
    let start = get_starting_node(&grid);
    // path that has the child as a key and the value as its parent
    let mut path: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut cur_pos = bfs(&grid, 'E' as u8, start, &mut path);
    let mut path_length = 0;

    while *path.get(&cur_pos).unwrap() != (start.0 as usize, start.1 as usize) {
        path_length += 1;
        cur_pos = *path.get(&cur_pos).unwrap();
    }
    path_length + 1
}

fn solve_part_two(data: &str) -> usize {
    let grid: Vec<_> = data.lines().map(|s| s.as_bytes()).collect();
    let nodes_with_elevation_a = get_nodes_with_elevation_a(&grid);
    let mut mn = usize::MAX;

    for node in nodes_with_elevation_a {
        // path that has the child as a key and the value as its parent
        let mut path: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut cur_pos = bfs(&grid, 'E' as u8, node, &mut path);
        let mut path_length = 0;
        if path.get(&cur_pos).is_some() {
            while *path.get(&cur_pos).unwrap() != (node.0 as usize, node.1 as usize) {
                path_length += 1;
                cur_pos = *path.get(&cur_pos).unwrap();
            }
            mn = std::cmp::min(mn, path_length + 1);
        }
    }
    mn
}

fn main() {
    let test = include_str!("../input/day12.test");
    let prod = include_str!("../input/day12.prod");
    println!("part1: test {:?}", solve_part_one(test));
    println!("part1: prod {:?}", solve_part_one(prod));
    println!("part2: test {:?}", solve_part_two(test));
    println!("part2: prod {:?}", solve_part_two(prod));
}
