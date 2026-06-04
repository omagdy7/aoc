use std::collections::{BTreeMap, HashMap};

type Pos = (usize, usize);

fn build_diagram(data: &str) -> (Vec<char>, usize) {
    let lines: Vec<&str> = data.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut grid = Vec::with_capacity(height * width);
    for line in lines {
        grid.extend(line.chars());
    }
    (grid, width)
}

fn idx(r: usize, c: usize, width: usize) -> usize {
    r * width + c
}

fn find_next_caret(grid: &[char], width: usize, start_r: usize, c: usize) -> Option<usize> {
    let height = grid.len() / width;
    for r in start_r..height {
        if grid[idx(r, c, width)] == '^' {
            return Some(r);
        }
    }
    None
}

fn propagate_pipes(grid: &mut [char], width: usize, r: usize, c: usize) {
    let height = grid.len() / width;
    if c + 1 < width {
        grid[idx(r, c + 1, width)] = '|';
        for nr in (r + 1)..height {
            if grid[idx(nr, c + 1, width)] == '^' {
                break;
            }
            grid[idx(nr, c + 1, width)] = '|';
        }
    }
    if c > 0 {
        grid[idx(r, c - 1, width)] = '|';
        for nr in (r + 1)..height {
            if grid[idx(nr, c - 1, width)] == '^' {
                break;
            }
            grid[idx(nr, c - 1, width)] = '|';
        }
    }
}

fn prepare_diagram(data: &str) -> (Vec<char>, usize) {
    let (mut grid, width) = build_diagram(data);
    let height = grid.len() / width;

    for r in 0..height {
        for c in 0..width {
            match grid[idx(r, c, width)] {
                'S' if r + 1 < height => grid[idx(r + 1, c, width)] = '|',
                '^' => propagate_pipes(&mut grid, width, r, c),
                _ => {}
            }
        }
    }
    (grid, width)
}

#[derive(Debug)]
struct Graph {
    edges: BTreeMap<Pos, Vec<Pos>>,
}

impl Graph {
    fn new(grid: &[char], width: usize) -> Self {
        let height = grid.len() / width;
        let mut edges: BTreeMap<Pos, Vec<Pos>> = BTreeMap::new();

        for r in 0..height {
            for c in 0..width {
                if grid[idx(r, c, width)] != '^' {
                    continue;
                }

                let parent = (r, c);
                edges.entry(parent).or_default();

                if let Some(next_r) = find_next_caret(grid, width, r + 1, c + 1) {
                    edges.entry(parent).or_default().push((next_r, c + 1));
                } else if r + 1 < height {
                    edges.entry((height, c + 1)).or_default();
                    edges.entry(parent).or_default().push((height, c + 1));
                }

                if c > 0 {
                    if let Some(next_r) = find_next_caret(grid, width, r + 1, c - 1) {
                        edges.entry(parent).or_default().push((next_r, c - 1));
                    } else if r + 1 < height {
                        edges.entry((height, c - 1)).or_default();
                        edges.entry(parent).or_default().push((height, c - 1));
                    }
                }
            }
        }
        Self { edges }
    }

    fn count_paths(&self, node: Pos, memo: &mut HashMap<Pos, usize>) -> usize {
        if let Some(&count) = memo.get(&node) {
            return count;
        }

        let neighbours = self.edges.get(&node).map(|v| v.as_slice()).unwrap_or(&[]);

        let count = if neighbours.is_empty() {
            1
        } else {
            neighbours.iter().map(|&n| self.count_paths(n, memo)).sum()
        };

        memo.insert(node, count);
        count
    }
}

fn solve_part_one(data: &str) -> usize {
    let (grid, width) = prepare_diagram(data);
    let height = grid.len() / width;
    let mut ans = 0;

    for r in 0..height {
        for c in 0..width {
            if grid[idx(r, c, width)] == '^' && r > 0 && grid[idx(r - 1, c, width)] == '|' {
                ans += 1;
            }
        }
    }
    ans
}

fn solve_part_two(data: &str) -> usize {
    let (grid, width) = prepare_diagram(data);
    let graph = Graph::new(&grid, width);
    let mut memo = HashMap::new();

    let start = graph.edges.keys().next().copied().unwrap_or((0, 0));
    graph.count_paths(start, &mut memo)
}

fn main() {
    let test = include_str!("../input/day_7.test");
    let prod = include_str!("../input/day_7.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod: {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test));
    println!("part_2 prod: {:?}", solve_part_two(prod));
}