#[derive(Debug)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<usize>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<usize>,
}

impl Passport {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

impl From<&str> for Passport {
    fn from(value: &str) -> Self {
        let mut passport = Passport::new();
        let v = value.replace("\n", " ");
        let v: Vec<&str> = v.split(" ").collect();
        for ele in v {
            if ele.is_empty() {
                continue;
            }
            let (key, value) = ele.split_once(':').unwrap();
            match key {
                "byr" => passport.byr = Some(value.parse::<usize>().unwrap()),
                "iyr" => passport.iyr = Some(value.parse::<usize>().unwrap()),
                "eyr" => passport.eyr = Some(value.parse::<usize>().unwrap()),
                "hgt" => {
                    passport.hgt = Some(
                        value
                            .trim_end_matches(|c| c == 'c' || c == 'm' || c == 'i' || c == 'n')
                            .parse::<usize>()
                            .unwrap(),
                    )
                }
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                "cid" => passport.cid = Some(value.parse::<usize>().unwrap()),
                _ => unreachable!(),
            }
        }
        passport
    }
}

fn solve_part_one(input: &str) -> usize {
    let mut cnt: usize = 0;
    let passports: Vec<Passport> = input
        .split("\n\n")
        .map(|pass| Passport::from(pass))
        .collect();
    for passport in passports {
        if passport.byr == None
            || passport.iyr == None
            || passport.eyr == None
            || passport.hgt == None
            || passport.hcl == None
            || passport.ecl == None
            || passport.pid == None
        {
            continue;
        }
        cnt += 1;
    }
    cnt
}

fn solve_part_two(input: &str) -> usize {
    todo!()
}

fn main() {
    let test = include_str!("../input/day_4.test");
    let prod = include_str!("../input/day_4.prod");
    println!("Test_1: {}", solve_part_one(test));
    println!("Prod_1: {}", solve_part_one(prod));
    // println!("Test_2: {}", solve_part_two(test));
    // println!("Prod_2: {}", solve_part_two(prod));
}
