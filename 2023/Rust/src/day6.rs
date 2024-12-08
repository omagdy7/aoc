use std::marker::PhantomData;

#[derive(Debug)]
struct Race<Part = One> {
    times: Vec<u64>,
    distances: Vec<u64>,
    state: std::marker::PhantomData<Part>,
}

struct One;
struct Two;

impl From<&str> for Race<One> {
    fn from(value: &str) -> Self {
        let (times, distances) = value.split_once("\n").unwrap();
        let (_, times) = times.split_once(':').unwrap();
        let (_, distances) = distances.split_once(':').unwrap();
        let times = times
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let distances = distances
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        Self {
            times,
            distances,
            state: PhantomData::<One>,
        }
    }
}

impl From<&str> for Race<Two> {
    fn from(value: &str) -> Self {
        let (times, distances) = value.split_once("\n").unwrap();
        let (_, times) = times.split_once(':').unwrap();
        let (_, distances) = distances.split_once(':').unwrap();
        let times = times
            .chars()
            .filter(|ch| ch.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let distances = distances
            .chars()
            .filter(|ch| ch.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        Self {
            times: vec![times],
            distances: vec![distances],
            state: PhantomData::<Two>,
        }
    }
}

fn quadratic_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b * b - 4.0 * a * c;
    let root1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b + discriminant.sqrt()) / (2.0 * a);
    // hacky solution to round the first root up and the scond root down
    ((root1 + 0.52).round(), (root2 - 0.52).round())
}

fn solve_math_two(data: &str) -> u64 {
    let race: Race<Two> = Race::from(data);
    race.times
        .iter()
        .zip(race.distances.iter())
        .map(|(&time, &distance)| {
            let (start, end) = quadratic_equation(1.0, -((time - 0) as f64), distance as f64);
            (end - start + 1.0) as usize
        })
        .product::<usize>() as u64
}

fn solve_math_one(data: &str) -> u64 {
    let race: Race<One> = Race::from(data);
    race.times
        .iter()
        .zip(race.distances.iter())
        .map(|(&time, &distance)| {
            let (start, end) = quadratic_equation(1.0, -((time - 0) as f64), distance as f64);
            (end - start + 1.0) as usize
        })
        .product::<usize>() as u64
}

fn solve_part_one(data: &str) -> u64 {
    let race: Race<One> = Race::from(data);
    race.times
        .iter()
        .zip(race.distances.iter())
        .map(|(&time, &distance)| {
            (0..time)
                .into_iter()
                .filter(|t| (time - t) * t > distance)
                .map(|_| distance)
                .count()
        })
        .product::<usize>() as u64
}
fn solve_part_two(data: &str) -> u64 {
    let race: Race<Two> = Race::from(data);
    race.times
        .iter()
        .zip(race.distances.iter())
        .map(|(&time, &distance)| {
            (0..time)
                .into_iter()
                .filter(|t| (time - t) * t > distance)
                .map(|_| distance)
                .count()
        })
        .product::<usize>() as u64
}

fn main() {
    let test = include_str!("../input/day6.test");
    let prod = include_str!("../input/day6.prod");
    println!("part_1 test: {:?}", solve_math_one(test));
    println!("part_1 prod {:?}", solve_math_one(prod));
    println!("part_2 test: {:?}", solve_math_two(test));
    println!("part_2 prod {:?}", solve_math_two(prod));
}
