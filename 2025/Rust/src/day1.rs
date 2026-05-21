#[derive(Debug)]
enum LockRotation {
    Left(usize),
    Right(usize),
}

impl From<&str> for LockRotation {
    fn from(value: &str) -> Self {
        let direction = value.chars().nth(0).unwrap();
        let rot_value = value[1..].parse::<usize>().unwrap();
        match direction {
            'L' => LockRotation::Left(rot_value),
            'R' => LockRotation::Right(rot_value),
            _ => unreachable!(),
        }
    }
}

fn solve_part_one(data: &str) -> usize {
    let rotations = data
        .lines()
        .map(|line| LockRotation::from(line))
        .collect::<Vec<LockRotation>>();
    let mut lock_pos: isize = 50;
    let mut ans = 0;
    for rot in rotations {
        match rot {
            LockRotation::Left(val) => lock_pos = (lock_pos - val as isize).rem_euclid(100),
            LockRotation::Right(val) => lock_pos = (lock_pos + val as isize).rem_euclid(100),
        }
        ans += if lock_pos == 0 { 1 } else { 0 };
    }
    ans
}

fn solve_part_two(data: &str) -> usize {
    let rotations = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(LockRotation::from);
    let mut lock_pos: isize = 50;
    let mut ans = 0;
    for rot in rotations {
        let (delta, steps) = match rot {
            LockRotation::Left(s) => (-1, s),
            LockRotation::Right(s) => (1, s),
        };

        for _ in 0..steps {
            lock_pos = (lock_pos + delta).rem_euclid(100);

            if lock_pos == 0 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let test_1 = include_str!("../input/day_1.test");
    let prod = include_str!("../input/day_1.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    println!("part_1 prod: {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_1));
    println!("part_2 prod: {:?}", solve_part_two(prod));
}
