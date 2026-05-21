fn parse_input(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
    i >= 0 && i < n && j >= 0 && j < m
}

fn is_less_than_4_neighbours(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    let dx: [i32; 8] = [0, 1, 0, -1, 1, -1, 1, -1];
    let dy: [i32; 8] = [1, 0, -1, 0, 1, -1, -1, 1];
    let mut count_rolls_paper: usize = 0;
    let n = grid.len();
    let m = grid[0].len();
    for k in 0..8 {
        let nx = i as i32 + dx[k];
        let ny = j as i32 + dy[k];
        if is_valid(nx as i32, ny as i32, n as i32, m as i32)
            && grid[nx as usize][ny as usize] == '@'
        {
            count_rolls_paper += 1;
        }
    }

    count_rolls_paper < 4
}

fn solve_part_one(data: &str) -> usize {
    let grid = parse_input(data);
    let mut ans = 0;
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '@' && is_less_than_4_neighbours(grid.as_slice(), i, j) {
                ans += 1;
            }
        }
    }
    ans
}

fn solve_part_two(data: &str) -> usize {
    let mut grid = parse_input(data);
    let mut ans = 0;
    let n = grid.len();
    let m = grid[0].len();
    let mut positions_to_modify = vec![];
    loop {
        let ans_before = ans;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '@' && is_less_than_4_neighbours(grid.as_slice(), i, j) {
                    ans += 1;
                    positions_to_modify.push((i, j));
                }
            }
        }
        for (i, j) in &positions_to_modify {
            grid[*i][*j] = 'x';
        }
        positions_to_modify.clear();
        if ans == ans_before {
            break;
        }
    }
    ans
}

fn main() {
    let test = include_str!("../input/day_4.test");
    let prod = include_str!("../input/day_4.prod");
    println!("part_1 test: {}", solve_part_one(test));
    println!("part_1 prod: {}", solve_part_one(prod));
    println!("part_2 test: {}", solve_part_two(test));
    println!("part_2 prod: {}", solve_part_two(prod));
}
