use std::io::BufRead;

#[derive(Debug)]
struct Game {
    id: usize,
    red: u32,
    blue: u32,
    green: u32,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let (game, sets) = s.split_once(':').unwrap();
        let (_, id) = game.split_once(' ').unwrap();
        let id = id.parse::<usize>().unwrap();
        let (mut r, mut g, mut b) = (0, 0, 0);
        let mut items: Vec<(u32, &str)> = vec![];
        for set in sets.split(';') {
            let sublist = set.split(',');
            for list in sublist {
                let list = list.trim_start();
                let (num, color) = list.split_once(' ').unwrap();
                items.push((num.parse::<u32>().unwrap(), color));
            }
        }
        for (num, color) in items {
            use std::cmp::max;
            match color {
                "red" => r = max(r, num),
                "green" => g = max(g, num),
                "blue" => b = max(b, num),
                _ => {}
            }
        }
        Self {
            id,
            red: r,
            green: g,
            blue: b,
        }
    }
}

fn solve_part_one(data: &str) -> u32 {
    data.lines()
        .map(|game| Game::from(game))
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .fold(0, |acc, game| acc + game.id as u32)
}

fn solve_part_two(data: &str) -> u32 {
    data.lines()
        .map(|game| Game::from(game))
        .map(|game| game.red * game.blue * game.green)
        .sum::<u32>()
}

fn main() {
    let test_1 = include_str!("../input/day2_1.test");
    let prod = include_str!("../input/day2.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_1));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
