use std::{collections::HashSet, ops::Add, ops::AddAssign};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(s: i64, e: i64) -> Self {
        Self { start: s, end: e }
    }

    fn add(&mut self, inc: i64) {
        self.start += inc;
        self.end += inc;
    }

    fn contains(&self, val: i64) -> bool {
        (self.start..self.end).contains(&val)
    }

    fn is_in(&self, other: &Range) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn overlap(&self, other: &Range) -> Option<(Range, Range)> {
        use std::cmp::{max, min};
        if self.start > other.start && self.start > other.end {
            return None;
        }
        let overlap_rng = Range::new(max(self.start, other.start), min(self.end, other.end - 1));
        let rest = Range::new(overlap_rng.end + 1, max(self.start, other.start));
        Some((overlap_rng, rest))
    }
}

fn solve_part_one(data: &str) -> u32 {
    let almanac = Almanac::from(data);
    let mut seeds = almanac.seeds.clone();
    for (_, maps) in almanac.maps.iter() {
        for seed in seeds.iter_mut() {
            let tmp = seed.clone();
            for seed_map in maps.iter() {
                if seed_map.contains(tmp) {
                    *seed += (seed_map.dest as i64 - seed_map.src as i64) as i64;
                }
            }
        }
    }
    *seeds.iter().min().unwrap() as u32
}

fn solve_part_two(data: &str) -> u64 {
    let almanac = Almanac::from(data);
    let mut seeds: Vec<Range> = vec![];
    for i in (0..almanac.seeds.len()).step_by(2) {
        seeds.push(Range::new(
            almanac.seeds[i],
            almanac.seeds[i] + almanac.seeds[i + 1],
        ))
    }
    for (_, maps) in almanac.maps.iter() {
        dbg!(&seeds);
        let mut new_seeds: HashSet<Range> = HashSet::from_iter(seeds.clone().into_iter());
        for seed in seeds.iter_mut() {
            let tmp = seed.clone();
            for seed_map in maps.iter() {
                if seed_map.is_in(&tmp) {
                    (*seed).add(seed_map.dest as i64 - seed_map.src as i64);
                } else if let Some((mut overlap_rng, rest)) = seed_map.overlap(&tmp) {
                    overlap_rng.add(seed_map.dest as i64 - seed_map.src as i64);
                    new_seeds.remove(seed);
                    new_seeds.insert(overlap_rng);
                    new_seeds.insert(rest);
                }
            }
        }
        seeds = Vec::from_iter(new_seeds.into_iter());
    }
    // dbg!(&seeds);
    (*seeds).iter().map(|rng| rng.start).min().unwrap() as u64
}

type Seeds = Vec<i64>;

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Seeds,
    maps: Vec<(&'a str, Vec<SeedMap>)>,
}

impl<'a> From<&'a str> for Almanac<'a> {
    fn from(data: &'a str) -> Self {
        let data: Vec<&str> = data.split("\n\n").collect();
        let (_, seeds) = data[0].split_once(": ").unwrap();
        let seeds: Vec<_> = seeds
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut maps: Vec<(&'a str, Vec<SeedMap>)> = Vec::new();
        for map in data.iter().skip(1) {
            let (desc, seed_maps) = map.split_once(':').unwrap();
            let (name, _) = desc.split_once(' ').unwrap();
            let mut v = vec![];
            for smap in seed_maps.trim_start().trim_end().split('\n') {
                v.push(SeedMap::from(smap));
            }
            maps.push((name, v));
        }
        Self { seeds, maps }
    }
}

#[derive(Debug)]
struct SeedMap {
    src: usize,
    dest: usize,
    range_len: usize,
}

impl SeedMap {
    fn is_in(&self, seed: &Range) -> bool {
        let rng = Range::new(self.src as i64, self.src as i64 + self.range_len as i64);
        seed.is_in(&rng)
    }

    fn overlap(&self, seed: &Range) -> Option<(Range, Range)> {
        let rng = Range::new(self.src as i64, self.src as i64 + self.range_len as i64);
        seed.overlap(&rng)
    }

    fn contains(&self, seed: i64) -> bool {
        (self.src..self.src + self.range_len).contains(&(seed as usize))
    }
}

impl From<&str> for SeedMap {
    fn from(seed_map: &str) -> Self {
        let map: Vec<_> = seed_map
            .split(' ')
            .map(|x| x.parse::<usize>().expect("Should be parsabel to usize"))
            .collect();
        Self {
            src: map[1],
            dest: map[0],
            range_len: map[2],
        }
    }
}

fn main() {
    let test = include_str!("../input/day5.test");
    let prod = include_str!("../input/day5.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    println!("part_1 prod {:?}", solve_part_one(prod));
    println!("part_2 test: {:?}", solve_part_two(test));
    // println!("part_2 prod {:?}", solve_part_two(prod));
}
