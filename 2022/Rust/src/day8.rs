enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn new(dir: (i32, i32)) -> Self {
        match dir {
            (0, 1) => Direction::Up,
            (1, 0) => Direction::Right,
            (0, -1) => Direction::Left,
            (-1, 0) => Direction::Down,
            _ => unreachable!(),
        }
    }
}

fn is_visible(grd: &Vec<&str>, direction: &Direction, visible: &mut Vec<Vec<bool>>) {
    let mut max_height = 0;
    match direction {
        Direction::Up => {
            for j in 0..grd[0].len() {
                let mut cur_max_h: i32 = -1;
                for i in (0..grd.len()).rev() {
                    if (grd[i].chars().nth(j).unwrap() as u8 - '0' as u8) as i32 > cur_max_h {
                        cur_max_h = (grd[i].chars().nth(j).unwrap() as u8 - '0' as u8) as i32;
                        visible[i][j] = true;
                    }
                }
            }
        }
        Direction::Down => {
            for j in 0..grd[0].len() {
                let mut cur_max_h: i32 = -1;
                for i in 0..grd.len() {
                    if (grd[i].chars().nth(j).unwrap() as u8 - '0' as u8) as i32 > cur_max_h {
                        cur_max_h = (grd[i].chars().nth(j).unwrap() as u8 - '0' as u8) as i32;
                        visible[i][j] = true;
                    }
                }
            }
        }
        Direction::Right => {
            for i in 0..grd.len() {
                let mut cur_max_h: i32 = -1;
                for j in 0..grd[0].len() {
                    if (grd[i].chars().nth(j).unwrap() as u8 - '0' as u8) as i32 > cur_max_h {
                        cur_max_h = (grd[i].chars().nth(j).unwrap() as u8 - '0' as u8) as i32;
                        visible[i][j] = true;
                    }
                }
            }
        }
        Direction::Left => {
            for i in 0..grd.len() {
                let mut cur_max_h: i32 = -1;
                for j in (0..grd[0].len()).rev() {
                    if (grd[i].chars().nth(j).unwrap() as u8 - '0' as u8) as i32 > cur_max_h {
                        cur_max_h = (grd[i].chars().nth(j).unwrap() as u8 - '0' as u8) as i32;
                        visible[i][j] = true;
                    }
                }
            }
        }
    }
}

fn is_valid_idx(pos: (usize, usize), size: (usize, usize), direction: &Direction) -> bool {
    match direction {
        Direction::Up => pos.0 as i32 - 1 >= 0,
        Direction::Down => pos.0 + 1 < size.0,
        Direction::Right => pos.1 + 1 < size.1,
        Direction::Left => pos.1 as i32 - 1 >= 0,
    }
}

fn count_vis_right(pos: (usize, usize), grd: &Vec<&str>) -> usize {
    let mut cnt = 0;
    if is_valid_idx(pos, (grd.len(), grd[0].len()), &Direction::Right) {
        let ch = grd[pos.0].chars().nth(pos.1).unwrap() as u8;
        for i in (pos.1 + 1)..grd[0].len() {
            if grd[pos.0].chars().nth(i).unwrap() as u8 >= ch {
                cnt += 1;
                break;
            }
            cnt += 1
        }
    }
    cnt
}
fn count_vis_left(pos: (usize, usize), grd: &Vec<&str>) -> usize {
    let mut cnt = 0;
    if is_valid_idx(pos, (grd.len(), grd[0].len()), &Direction::Left) {
        let ch = grd[pos.0].chars().nth(pos.1).unwrap() as u8;
        for i in (0..=(pos.1 - 1)).rev() {
            if grd[pos.0].chars().nth(i).unwrap() as u8 >= ch {
                cnt += 1;
                break;
            }
            cnt += 1
        }
    }
    cnt
}

fn count_vis_up(pos: (usize, usize), grd: &Vec<&str>) -> usize {
    let mut cnt = 0;
    if is_valid_idx(pos, (grd.len(), grd[0].len()), &Direction::Up) {
        let ch = grd[pos.0].chars().nth(pos.1).unwrap() as u8;
        // print!("{:?}  ", pos);
        for i in (0..=(pos.0 - 1)).rev() {
            // print!("{:?}, ", (i, pos.1));
            if grd[i].chars().nth(pos.1).unwrap() as u8 >= ch {
                cnt += 1;
                break;
            }
            cnt += 1
        }
        // println!()
    }
    cnt
}
fn count_vis_down(pos: (usize, usize), grd: &Vec<&str>) -> usize {
    let mut cnt = 0;
    let ch = grd[pos.0].chars().nth(pos.1).unwrap() as u8;
    if is_valid_idx(pos, (grd.len(), grd[0].len()), &Direction::Down) {
        for i in (pos.0 + 1)..grd.len() {
            if grd[i].chars().nth(pos.1).unwrap() as u8 >= ch {
                cnt += 1;
                break;
            }
            cnt += 1
        }
    }
    cnt
}

fn count_visible(pos: (usize, usize), grd: &Vec<&str>) -> usize {
    count_vis_up(pos, grd)
        * count_vis_down(pos, grd)
        * count_vis_left(pos, grd)
        * count_vis_right(pos, grd)
}

fn solve_part_one(data: &str) -> usize {
    let grid: Vec<_> = data.lines().collect();
    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [1, 0, -1, 0];
    let mut visible: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    for i in 0..4 {
        let direction = Direction::new((dx[i], dy[i]));
        is_visible(&grid, &direction, &mut visible)
    }
    visible.iter().flatten().filter(|&x| *x == true).count()
}

fn solve_part_two(data: &str) -> usize {
    let grid: Vec<_> = data.lines().collect();

    let mut max_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            max_score = std::cmp::max(max_score, count_visible((i, j), &grid));
        }
    }
    max_score
}

fn main() {
    let data_test = include_str!("../input/day8.test");
    let data_prod = include_str!("../input/day8.prod");
    println!("{}", solve_part_one(data_test));
    println!("{}", solve_part_one(data_prod));
    println!("{}", solve_part_two(data_test));
    println!("{}", solve_part_two(data_prod));
}
