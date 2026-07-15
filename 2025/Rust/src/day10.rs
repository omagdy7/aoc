#[derive(Debug)]
struct Manual {
    lights: Vec<bool>,
    buttons: Vec<Vec<i64>>,
    joltage_requirements: Vec<i64>,
}

impl From<&str> for Manual {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split(' ').collect();

        let [first, middle @ .., last] = &parts[..] else {
            panic!("need at least two parts");
        };

        let lights: Vec<bool> = first[1..first.len() - 1]
            .chars()
            .map(|ch| ch == '#')
            .collect();

        let buttons: Vec<Vec<i64>> = middle
            .iter()
            .map(|x| {
                let mut wiring = vec![];
                for ch in x.chars() {
                    if ch.is_ascii_digit() {
                        wiring.push(ch.to_digit(10).unwrap() as i64);
                    }
                }
                wiring
            })
            .collect();
        let joltage_requirements: Vec<i64> = last[1..last.len() - 1]
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        Self {
            lights,
            buttons,
            joltage_requirements,
        }
    }
}

fn simulate_button_press(button: &Vec<i64>, lights: &mut Vec<bool>) {
    for i in button {
        lights[*i as usize] = !lights[*i as usize];
    }
}

fn solve_part_one(data: &str) -> i64 {
    let mut ans: i64 = 0;
    for line in data.lines() {
        let manual = Manual::from(line);
        dbg!(&manual);
        let mut min_steps = i64::MAX;
        for mask in 0..=(1usize << manual.buttons.len()) {
            let mut original_lights = manual.lights.clone();
            let mut cnt = 0;
            for j in 0..manual.buttons.len() {
                let bit = (mask >> j) & 1;
                if bit == 1 {
                    simulate_button_press(&manual.buttons[j], &mut original_lights);
                    cnt += 1;
                }
            }
            if original_lights.iter().all(|x| !x) {
                min_steps = min_steps.min(cnt);
            }
        }
        ans += min_steps;
    }
    ans
}

fn solve_part_two(data: &str) -> i64 {
    todo!()
}

fn main() {
    let test = include_str!("../input/day_10.test");
    let prod = include_str!("../input/day_10.prod");
    println!("part_1 test: {:?}", solve_part_one(test));
    // println!("part_1 prod: {:?}", solve_part_one(prod));
    // println!("part_2 test: {:?}", solve_part_two(test));
    // println!("part_2 prod: {:?}", solve_part_two(prod));
}
