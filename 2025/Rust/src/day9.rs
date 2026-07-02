use std::cmp::{max, min};

fn solve_part_one(data: &str) -> usize {
    let red_tile_positions: Vec<(usize, usize)> = data
        .lines()
        .map(|line| {
            let (y, x) = line.split_once(',').unwrap();
            let (y, x) = (y.parse::<usize>().unwrap(), x.parse::<usize>().unwrap());
            (x, y)
        })
        .collect();
    let mut width = 0;
    let mut height = 0;
    let mut manhatten_distance = 0;
    for (x1, y1) in red_tile_positions.iter() {
        for (x2, y2) in red_tile_positions.iter().skip(1) {
            let cur_distance = x1.abs_diff(*x2) + y1.abs_diff(*y2);
            if cur_distance > manhatten_distance {
                manhatten_distance = cur_distance;
                width = x1.abs_diff(*x2) + 1;
                height = y1.abs_diff(*y2) + 1;
            }
        }
    }
    width * height
}

#[derive(Clone, Copy, Debug)]
struct Point2D {
    x: i64,
    y: i64,
}

fn rect_is_valid(points: &[Point2D], xlo: i64, xhi: i64, ylo: i64, yhi: i64) -> bool {
    let n = points.len();
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        // vertically aligned
        if p1.x == p2.x {
            if p1.x > xlo && p1.x < xhi && min(p1.y, p2.y) < yhi && ylo < max(p1.y, p2.y) {
                return false;
            }
        } else if p1.y == p2.y {
            // horizontally aligned
            if p1.y > ylo && p1.y < yhi && min(p1.x, p2.x) < xhi && xlo < max(p1.x, p2.x) {
                return false;
            }
        }
    }
    let cx = (xlo + xhi) as f64 / 2.0;
    let cy = (ylo + yhi) as f64 / 2.0;
    point_in_or_on_polygon(points, cx, cy)
}

fn point_in_or_on_polygon(pts: &[Point2D], px: f64, py: f64) -> bool {
    let n = pts.len();
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        if a.x == b.x
            && (a.x as f64) == px
            && py >= a.y.min(b.y) as f64
            && py <= a.y.max(b.y) as f64
        {
            return true;
        }
        if a.y == b.y
            && (a.y as f64) == py
            && px >= a.x.min(b.x) as f64
            && px <= a.x.max(b.x) as f64
        {
            return true;
        }
    }
    let mut inside = false;
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        let (ay, by) = (a.y as f64, b.y as f64);
        if (ay > py) != (by > py) {
            let x_int = a.x as f64 + (py - ay) / (by - ay) * (b.x as f64 - a.x as f64);
            if px < x_int {
                inside = !inside;
            }
        }
    }
    inside
}

fn solve_part_two(data: &str) -> i64 {
    let pts: Vec<Point2D> = data
        .lines()
        .map(|line| {
            let (y, x) = line.split_once(',').unwrap();
            let (y, x) = (y.parse::<i64>().unwrap(), x.parse::<i64>().unwrap());
            Point2D { x, y }
        })
        .collect();
    let n = pts.len();

    let mut pairs: Vec<(i64, usize, usize)> = Vec::with_capacity(n * n / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = (pts[i].x, pts[i].y);
            let (x2, y2) = (pts[j].x, pts[j].y);
            let area =
                (x1 - x2).abs().checked_add(1).unwrap() * (y1 - y2).abs().checked_add(1).unwrap();
            pairs.push((area, i, j));
        }
    }
    pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // descending by area

    for (area, i, j) in pairs {
        let (x1, y1) = (pts[i].x, pts[i].y);
        let (x2, y2) = (pts[j].x, pts[j].y);
        let (xlo, xhi) = (x1.min(x2), x1.max(x2));
        let (ylo, yhi) = (y1.min(y2), y1.max(y2));
        if rect_is_valid(&pts, xlo, xhi, ylo, yhi) {
            return area;
        }
    }
    unreachable!("no valid rectangle found")
}
fn main() {
    let test = include_str!("../input/day_9.test");
    let prod = include_str!("../input/day_9.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod: {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test));
    println!("part_2 prod: {:?}", solve_part_two(prod));
}
