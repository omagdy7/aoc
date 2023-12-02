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
        for set in sets.split(';') {
            dbg!(&set.trim_start());
            let (first, second) = set.trim_start().split_once(',').unwrap();
            let (num_1, color_1) = first.split_once(' ').unwrap();
            let (num_2, color_2) = second.split_once(' ').unwrap();
            match color_1 {
                "red" => {
                    r += num_1.parse::<u32>().unwrap();
                }
                "green" => {
                    g += num_1.parse::<u32>().unwrap();
                }
                "blue" => {
                    b += num_1.parse::<u32>().unwrap();
                }
                _ => {}
            }
            match color_2 {
                "red" => {
                    r += num_2.parse::<u32>().unwrap();
                }
                "green" => {
                    g += num_2.parse::<u32>().unwrap();
                }
                "blue" => {
                    b += num_2.parse::<u32>().unwrap();
                }
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
    let mut games: Vec<Game> = vec![];
    for game in data.lines() {
        games.push(Game::from(game));
    }
    dbg!(&games);
    42
}

fn solve_part_two(data: &str) -> u32 {
    todo!()
}

fn main() {
    let test_1 = include_str!("../input/day2_1.test");
    let test_2 = include_str!("../input/day2_2.test");
    let prod = include_str!("../input/day2.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    // println!("part_1 prod {:?}", solve_part_one(prod));
    // println!("part_2 test: {:?}", solve_part_two(test_2));
    // println!("part_2 prod {:?}", solve_part_two(prod));
}
