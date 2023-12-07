use std::{cmp::Ordering, collections::HashMap, marker::PhantomData};

struct One;
struct Two;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Round<'a> {
    hand: Hand<'a>,
    bid: usize,
}

#[derive(Debug, Eq, Clone, Copy, Ord)]
enum Hand<'a> {
    FiveOfKind(&'a str),
    FourOfKind(&'a str),
    FullHouse(&'a str),
    ThreeOfKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

#[derive(Debug, Eq, Clone, Copy, Ord)]
enum HandJokers<'a> {
    FiveOfKind(&'a str),
    FourOfKind(&'a str),
    FullHouse(&'a str),
    ThreeOfKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

const LABELS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn stronger(a: &str, b: &str) -> Option<Ordering> {
    for (first, second) in a.chars().zip(b.chars()) {
        let pos_first = LABELS.iter().position(|&x| x == first);
        let pos_second = LABELS.iter().position(|&x| x == second);
        if pos_first > pos_second {
            return Some(Ordering::Greater);
        } else if pos_second > pos_first {
            return Some(Ordering::Less);
        }
    }
    Some(Ordering::Equal)
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        use Hand::*;
        match (self, other) {
            (FiveOfKind(a), FiveOfKind(b)) => a == b,
            (FourOfKind(a), FourOfKind(b)) => a == b,
            (FullHouse(a), FullHouse(b)) => a == b,
            (ThreeOfKind(a), ThreeOfKind(b)) => a == b,
            (TwoPair(a), TwoPair(b)) => a == b,
            (OnePair(a), OnePair(b)) => a == b,
            (HighCard(a), HighCard(b)) => a == b,
            (_, _) => false,
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Hand::*;
        match (self, other) {
            (FiveOfKind(a), FiveOfKind(b)) => stronger(a, b),
            (FiveOfKind(_), _) => Some(Ordering::Greater),
            (_, FiveOfKind(_)) => Some(Ordering::Less),
            (FourOfKind(a), FourOfKind(b)) => stronger(a, b),
            (FourOfKind(_), _) => Some(Ordering::Greater),
            (_, FourOfKind(_)) => Some(Ordering::Less),
            (FullHouse(a), FullHouse(b)) => stronger(a, b),
            (FullHouse(_), _) => Some(Ordering::Greater),
            (_, FullHouse(_)) => Some(Ordering::Less),
            (ThreeOfKind(a), ThreeOfKind(b)) => stronger(a, b),
            (ThreeOfKind(_), _) => Some(Ordering::Greater),
            (_, ThreeOfKind(_)) => Some(Ordering::Less),
            (TwoPair(a), TwoPair(b)) => stronger(a, b),
            (TwoPair(_), _) => Some(Ordering::Greater),
            (_, TwoPair(_)) => Some(Ordering::Less),
            (OnePair(a), OnePair(b)) => stronger(a, b),
            (OnePair(_), _) => Some(Ordering::Greater),
            (_, OnePair(_)) => Some(Ordering::Less),
            (HighCard(a), HighCard(b)) => stronger(a, b),
            (HighCard(_), _) => Some(Ordering::Greater),
            (_, HighCard(_)) => Some(Ordering::Less),
            (_, _) => None,
        }
    }
}

impl<'a> From<&'a str> for Round<'a> {
    fn from(round: &'a str) -> Self {
        let (hand, bid) = round.split_once(' ').unwrap();
        let (hand, bid) = (Hand::from(hand), bid.parse::<usize>().unwrap());
        Self { hand, bid }
    }
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(hand: &'a str) -> Self {
        let mut frq: HashMap<char, usize> = HashMap::new();
        for ch in hand.chars() {
            frq.entry(ch).and_modify(|f| *f += 1).or_insert(1);
        }

        let mut two_pairs = 0;
        let mut three_pairs = 0;
        let mut four_pairs = 0;
        for (_, val) in frq.iter() {
            match val {
                3 => three_pairs += 1,
                2 => two_pairs += 1,
                4 => four_pairs += 1,
                _ => {}
            }
        }

        match (frq.len(), two_pairs, three_pairs, four_pairs) {
            (5, _, _, _) => Hand::HighCard(hand),
            (4, _, _, _) => Hand::OnePair(hand),
            (1, _, _, _) => Hand::FiveOfKind(hand),
            (_, _, _, 1) => Hand::FourOfKind(hand),
            (_, 0, 1, _) => Hand::ThreeOfKind(hand),
            (_, 1, 1, _) => Hand::FullHouse(hand),
            (_, 2, 0, _) => Hand::TwoPair(hand),
            _ => {
                unreachable!()
            }
        }
    }
}

fn solve_part_one(data: &str) -> u64 {
    let mut rounds = data
        .lines()
        .map(|line| Round::from(line))
        .collect::<Vec<_>>();
    rounds.sort_by(|b, a| a.hand.partial_cmp(&b.hand).unwrap());
    let n = rounds.len();
    rounds
        .iter()
        .enumerate()
        .map(|(i, round)| round.bid * (n - i))
        .sum::<usize>() as u64
}
fn solve_part_two(data: &str) -> u64 {
    todo!()
}

fn main() {
    let test = include_str!("../input/day7.test");
    let prod = include_str!("../input/day7.prod");
    // println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod {:?}", solve_part_one(prod));
    // println!("part_2 test: {:?}", solve_part_two(test));
    // println!("part_2 prod {:?}", solve_part_two(prod));
}
