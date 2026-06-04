#[derive(Debug)]
enum Symbol {
    Mul,
    Add,
}

#[derive(Debug)]
enum NumAlignment {
    Right,
    Left,
}

#[derive(Debug)]
struct MathSheet {
    nums: Vec<Vec<u64>>,
    alignment: Vec<NumAlignment>,
    symbols: Vec<Symbol>,
}

impl From<&str> for MathSheet {
    fn from(value: &str) -> Self {
        let mut nums: Vec<Vec<u64>> = vec![];
        let mut symbols = vec![];
        let mut alignment: Vec<NumAlignment> = vec![];
        let grid: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();
        let rows_count = grid.len();
        for line in value.lines() {
            if line.starts_with('*') || line.starts_with('+') {
                for (col, ch) in line.chars().enumerate() {
                    if ch == '*' || ch == '+' {
                        let has_space = (0..rows_count - 1).any(|i| grid[i][col] == ' ');
                        alignment.push(if has_space {
                            NumAlignment::Right
                        } else {
                            NumAlignment::Left
                        });
                    }
                }
                symbols = line
                    .split_whitespace()
                    .map(|s| match s {
                        "*" => Symbol::Mul,
                        "+" => Symbol::Add,
                        _ => unreachable!(),
                    })
                    .collect()
            } else {
                nums.push(
                    line.trim_start()
                        .split_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect(),
                );
            }
        }
        Self {
            nums,
            alignment,
            symbols,
        }
    }
}

fn solve_part_one(data: &str) -> usize {
    let math_sheet = MathSheet::from(data);
    let rows = math_sheet.nums.len();
    let mut ans = 0;
    for (col, symbol) in math_sheet.symbols.iter().enumerate() {
        match symbol {
            Symbol::Mul => ans += (0..rows).map(|j| math_sheet.nums[j][col]).product::<u64>(),
            Symbol::Add => ans += (0..rows).map(|j| math_sheet.nums[j][col]).sum::<u64>(),
        }
    }
    ans as usize
}

fn extract_vertical_numbers(nums: &[Vec<u64>], col: usize, alignment: &NumAlignment) -> Vec<u64> {
    let num_strings: Vec<String> = nums.iter().map(|row| row[col].to_string()).collect();
    let char_grid: Vec<Vec<char>> = num_strings.iter().map(|s| s.chars().collect()).collect();
    let row_count = char_grid.len();
    let max_len = char_grid.iter().map(|row| row.len()).max().unwrap();
    let char_grid: Vec<Vec<char>> = char_grid
        .iter()
        .map(|row| match alignment {
            NumAlignment::Right => std::iter::repeat('X')
                .take(max_len - row.len())
                .chain(row.iter().copied())
                .collect(),
            NumAlignment::Left => row
                .iter()
                .copied()
                .chain(std::iter::repeat('X').take(max_len - row.len()))
                .collect(),
        })
        .collect();

    (0..row_count)
        .filter_map(|c| {
            let digits: String = char_grid
                .iter()
                .filter_map(|row| {
                    if c < row.len() && row[c] != 'X' {
                        Some(row[c])
                    } else {
                        None
                    }
                })
                .collect();
            if digits.is_empty() {
                None
            } else {
                Some(digits.parse::<u64>().unwrap())
            }
        })
        .collect()
}

fn solve_part_two(data: &str) -> usize {
    let math_sheet = MathSheet::from(data);
    let mut ans = 0;
    for (col, symbol) in math_sheet.symbols.iter().enumerate() {
        let numbers = extract_vertical_numbers(&math_sheet.nums, col, &math_sheet.alignment[col]);
        match symbol {
            Symbol::Mul => ans += numbers.iter().product::<u64>(),
            Symbol::Add => ans += numbers.iter().sum::<u64>(),
        }
    }
    ans as usize
}

fn main() {
    let test = include_str!("../input/day_6.test");
    let prod = include_str!("../input/day_6.prod");
    println!("part_1 test: {}", solve_part_one(test));
    println!("part_1 prod: {}", solve_part_one(prod));
    println!("part_2 test: {}", solve_part_two(test));
    println!("part_2 prod: {}", solve_part_two(prod));
}
