#![feature(specialization)]
use std::{cmp::Ordering, collections::HashMap, marker::PhantomData};

struct One;
struct Two;

trait PartOne {}
trait PartTwo {}
trait PartOneTwo {}

impl PartOne for One {}
impl PartTwo for Two {}
impl PartOneTwo for One {}
impl PartOneTwo for Two {}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Round<'a, Part = One> {
    hand: Hand<'a, Part>,
    bid: usize,
}

#[derive(Debug, Eq, Clone, Copy, Ord)]
enum Hand<'a, Part = One> {
    FiveOfKind(&'a str),
    FourOfKind(&'a str),
    FullHouse(&'a str),
    ThreeOfKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
    _Phantom(PhantomData<Part>),
}

const LABELS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const LABELS_JOKERS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn stronger_two(a: &str, b: &str) -> Option<Ordering> {
    for (first, second) in a.chars().zip(b.chars()) {
        let pos_first = LABELS_JOKERS.iter().position(|&x| x == first);
        let pos_second = LABELS_JOKERS.iter().position(|&x| x == second);
        if pos_first > pos_second {
            return Some(Ordering::Greater);
        } else if pos_second > pos_first {
            return Some(Ordering::Less);
        }
    }
    Some(Ordering::Equal)
}

fn stronger_one(a: &str, b: &str) -> Option<Ordering> {
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

impl<'a, T> PartialEq for Hand<'a, T> {
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

impl<'a, T> PartialOrd for Hand<'a, T> {
    default fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        unreachable!()
    }
}

impl<'a> PartialOrd for Hand<'a, Two> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Hand::*;
        match (self, other) {
            (FiveOfKind(a), FiveOfKind(b)) => stronger_two(a, b),
            (FiveOfKind(_), _) => Some(Ordering::Greater),
            (_, FiveOfKind(_)) => Some(Ordering::Less),
            (FourOfKind(a), FourOfKind(b)) => stronger_two(a, b),
            (FourOfKind(_), _) => Some(Ordering::Greater),
            (_, FourOfKind(_)) => Some(Ordering::Less),
            (FullHouse(a), FullHouse(b)) => stronger_two(a, b),
            (FullHouse(_), _) => Some(Ordering::Greater),
            (_, FullHouse(_)) => Some(Ordering::Less),
            (ThreeOfKind(a), ThreeOfKind(b)) => stronger_two(a, b),
            (ThreeOfKind(_), _) => Some(Ordering::Greater),
            (_, ThreeOfKind(_)) => Some(Ordering::Less),
            (TwoPair(a), TwoPair(b)) => stronger_two(a, b),
            (TwoPair(_), _) => Some(Ordering::Greater),
            (_, TwoPair(_)) => Some(Ordering::Less),
            (OnePair(a), OnePair(b)) => stronger_two(a, b),
            (OnePair(_), _) => Some(Ordering::Greater),
            (_, OnePair(_)) => Some(Ordering::Less),
            (HighCard(a), HighCard(b)) => stronger_two(a, b),
            (_, _) => None,
        }
    }
}

impl<'a> PartialOrd for Hand<'a, One> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Hand::*;
        match (self, other) {
            (FiveOfKind(a), FiveOfKind(b)) => stronger_one(a, b),
            (FiveOfKind(_), _) => Some(Ordering::Greater),
            (_, FiveOfKind(_)) => Some(Ordering::Less),
            (FourOfKind(a), FourOfKind(b)) => stronger_one(a, b),
            (FourOfKind(_), _) => Some(Ordering::Greater),
            (_, FourOfKind(_)) => Some(Ordering::Less),
            (FullHouse(a), FullHouse(b)) => stronger_one(a, b),
            (FullHouse(_), _) => Some(Ordering::Greater),
            (_, FullHouse(_)) => Some(Ordering::Less),
            (ThreeOfKind(a), ThreeOfKind(b)) => stronger_one(a, b),
            (ThreeOfKind(_), _) => Some(Ordering::Greater),
            (_, ThreeOfKind(_)) => Some(Ordering::Less),
            (TwoPair(a), TwoPair(b)) => stronger_one(a, b),
            (TwoPair(_), _) => Some(Ordering::Greater),
            (_, TwoPair(_)) => Some(Ordering::Less),
            (OnePair(a), OnePair(b)) => stronger_one(a, b),
            (OnePair(_), _) => Some(Ordering::Greater),
            (_, OnePair(_)) => Some(Ordering::Less),
            (HighCard(a), HighCard(b)) => stronger_one(a, b),
            (_, _) => None,
        }
    }
}

impl<'a> From<&'a str> for Round<'a, Two> {
    fn from(round: &'a str) -> Self {
        let (hand, bid) = round.split_once(' ').unwrap();
        let (hand, bid) = (Hand::<Two>::from(hand), bid.parse::<usize>().unwrap());
        Self { hand, bid }
    }
}

impl<'a> From<&'a str> for Round<'a, One> {
    fn from(round: &'a str) -> Self {
        let (hand, bid) = round.split_once(' ').unwrap();
        let (hand, bid) = (Hand::from(hand), bid.parse::<usize>().unwrap());
        Self { hand, bid }
    }
}

impl<'a> From<&'a str> for Hand<'a, Two> {
    fn from(hand: &'a str) -> Self {
        use Hand::*;
        let mut frq: HashMap<char, usize> = HashMap::new();

        for ch in hand.chars() {
            frq.entry(ch).and_modify(|f| *f += 1).or_insert(1);
        }

        let mut cur = 0usize;
        let mut most_char = '1';

        for (&ch, &f) in frq.iter() {
            if ch != 'J' && f >= cur {
                cur = f;
                most_char = ch;
            }
        }

        let jokers = frq.get(&'J');

        let mut two_pairs = 0usize;
        let mut three_pairs = 0usize;
        let mut four_pairs = 0usize;
        for (_, val) in frq.iter() {
            match val {
                3 => three_pairs += 1,
                2 => two_pairs += 1,
                4 => four_pairs += 1,
                _ => {}
            }
        }

        if let Some(_) = jokers {
            let new_hand = hand.replace("J", &most_char.to_string());
            let mut jfrq: HashMap<char, usize> = HashMap::new();
            for ch in new_hand.chars() {
                jfrq.entry(ch).and_modify(|f| *f += 1).or_insert(1);
            }
            let mut two_pairs = 0usize;
            let mut three_pairs = 0usize;
            let mut four_pairs = 0usize;
            for (_, val) in jfrq.iter() {
                match val {
                    3 => three_pairs += 1,
                    2 => two_pairs += 1,
                    4 => four_pairs += 1,
                    _ => {}
                }
            }
            match (jfrq.len(), two_pairs, three_pairs, four_pairs) {
                (5, _, _, _) => HighCard(hand),
                (4, _, _, _) => OnePair(hand),
                (1, _, _, _) => FiveOfKind(hand),
                (_, _, _, 1) => FourOfKind(hand),
                (_, 0, 1, _) => ThreeOfKind(hand),
                (_, 1, 1, _) => FullHouse(hand),
                (_, 2, 0, _) => TwoPair(hand),
                _ => {
                    unreachable!()
                }
            }
        } else {
            match (frq.len(), two_pairs, three_pairs, four_pairs) {
                (5, _, _, _) => HighCard(hand),
                (4, _, _, _) => OnePair(hand),
                (1, _, _, _) => FiveOfKind(hand),
                (_, _, _, 1) => FourOfKind(hand),
                (_, 0, 1, _) => ThreeOfKind(hand),
                (_, 1, 1, _) => FullHouse(hand),
                (_, 2, 0, _) => TwoPair(hand),
                _ => {
                    unreachable!()
                }
            }
        }
    }
}
impl<'a> From<&'a str> for Hand<'a, One> {
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
        .map(|line| Round::<One>::from(line))
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
    let mut rounds = data
        .lines()
        .map(|line| Round::<Two>::from(line))
        .collect::<Vec<_>>();
    rounds.sort_by(|b, a| a.hand.partial_cmp(&b.hand).unwrap());
    let n = rounds.len();
    rounds
        .iter()
        .enumerate()
        .map(|(i, round)| round.bid * (n - i))
        .sum::<usize>() as u64
}

fn main() {
    let test = include_str!("../input/day7.test");
    let prod = include_str!("../input/day7.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test));
    println!("part_2 prod {:?}", solve_part_two(prod));
}
