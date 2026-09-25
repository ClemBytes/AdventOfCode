use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY03 -------");
    let example = fs::read_to_string("inputs/example_day03").expect("Unable to read input!");
    let example = Claim::parse(&example);
    let input = fs::read_to_string("inputs/input_day03").expect("Unable to read input!");
    let input = Claim::parse(&input);

    day03_part1(&example, &input);
    day03_part2(&example, &input);
}

#[derive(Debug, Clone)]
struct Claim {
    _id: u32,
    left_edge: u32,
    top_edge: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn parse(raw_input: &str) -> Vec<Self> {
        let mut claims = vec![];
        let r = Regex::new(r"^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$").unwrap();
        for line in raw_input.lines() {
            let matches = r.captures(line).unwrap();
            let claim = Self {
                _id: matches[1].parse().unwrap(),
                left_edge: matches[2].parse().unwrap(),
                top_edge: matches[3].parse().unwrap(),
                width: matches[4].parse().unwrap(),
                height: matches[5].parse().unwrap(),
            };
            claims.push(claim);
        }
        claims
    }
}

fn solve_part1(claims: &[Claim]) -> u32 {
    let mut claimed_squares = HashMap::new();
    for claim in claims {
        for i in 0..claim.width {
            for j in 0..claim.height {
                let count = claimed_squares
                    .entry((claim.left_edge + i, claim.top_edge + j))
                    .or_insert(0);
                *count += 1;
            }
        }
    }
    let mut nb = 0;
    for nb_times_claimed in claimed_squares.values() {
        if *nb_times_claimed > 1 {
            nb += 1;
        }
    }
    nb
}

fn day03_part1(example: &[Claim], input: &[Claim]) {
    // Exemple tests
    assert_eq!(solve_part1(example), 4);
    println!("Example OK");

    // Solve puzzle
    let res = solve_part1(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 116489);
    println!("> DAY03 - part 1: OK!");
}

fn day03_part2(_example: &[Claim], _input: &[Claim]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY03 - part 2: OK!");
}
