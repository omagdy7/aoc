#[derive(Debug)]
struct ScratchCard {
    winning: Vec<u32>,
    numbers: Vec<u32>,
    cnt: usize,
}

impl ScratchCard {
    fn get_matching(&self) -> u32 {
        let mut cnt = 0;
        for number in self.numbers.iter() {
            if self.winning.contains(&number) {
                cnt += 1;
            }
        }
        cnt
    }
}

impl From<&str> for ScratchCard {
    fn from(value: &str) -> Self {
        let (_, sets) = value.split_once(':').expect("Should be splittable by ':'");
        let (winning, numbers) = sets.split_once('|').expect("Should be splittable by '|'");
        let (winning, numbers) = (
            winning
                .trim_start()
                .trim_end()
                .split(' ')
                .filter(|x| x.parse::<u32>().is_ok())
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
            numbers
                .trim_start()
                .trim_end()
                .split(' ')
                .filter(|x| x.parse::<u32>().is_ok())
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
        Self {
            winning,
            numbers,
            cnt: 1,
        }
    }
}

fn solve_part_one(data: &str) -> u32 {
    let mut scratch_cards: Vec<ScratchCard> = vec![];
    for line in data.lines() {
        scratch_cards.push(ScratchCard::from(line))
    }
    scratch_cards
        .iter()
        .map(|card| card.get_matching())
        .filter(|x| *x > 0)
        .map(|x| 2_u32.pow(std::cmp::max(x - 1, 0) as u32))
        .sum::<u32>()
}

fn solve_part_two(data: &str) -> u32 {
    let mut scratch_cards: Vec<ScratchCard> = vec![];
    for line in data.lines() {
        scratch_cards.push(ScratchCard::from(line))
    }
    let n = scratch_cards.len();
    for i in 0..n - 1 {
        let matches = scratch_cards[i].get_matching();
        let cnt = scratch_cards[i].cnt;
        for _ in 0..cnt {
            for j in (i + 1)..(i + matches as usize + 1) {
                scratch_cards[j].cnt += 1;
            }
        }
    }
    scratch_cards.iter().map(|card| card.cnt).sum::<usize>() as u32
}

fn main() {
    let test_1 = include_str!("../input/day4.test");
    let prod = include_str!("../input/day4.prod");
    println!("part_1 test: {:?}", solve_part_one(test_1));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test_1));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
