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
    id: u32,
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
                id: matches[1].parse().unwrap(),
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

fn count_claimed_squares(claims: &[Claim]) -> HashMap<(u32, u32), Vec<u32>> {
    let mut claimed_squares = HashMap::new();
    for claim in claims {
        for i in 0..claim.width {
            for j in 0..claim.height {
                let list = claimed_squares
                    .entry((claim.left_edge + i, claim.top_edge + j))
                    .or_insert(vec![]);
                list.push(claim.id);
            }
        }
    }
    claimed_squares
}

fn solve_part1(claims: &[Claim]) -> u32 {
    let claimed_squares = count_claimed_squares(claims);
    let mut nb = 0;
    for ids_list in claimed_squares.values() {
        if ids_list.len() > 1 {
            nb += 1;
        }
    }
    nb
}

fn solve_part2(claims: &[Claim]) -> u32 {
    let claimed_squares = count_claimed_squares(claims);
    let mut ids = HashMap::new();
    for claim in claims {
        ids.insert(claim.id, true);
    }
    for ids_list in claimed_squares.values() {
        if ids_list.len() > 1 {
            for id in ids_list {
                ids.insert(*id, false);
            }
        }
    }
    for (id, is_unique) in ids {
        if is_unique {
            return id;
        }
    }
    unreachable!();
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

fn day03_part2(example: &[Claim], input: &[Claim]) {
    // Exemple tests
    assert_eq!(solve_part2(example), 3);
    println!("Example OK");

    // Solve puzzle
    let res = solve_part2(input);
    println!("Result part 2: {res}"); // 1215 is too low
    assert_eq!(res, 1260);
    println!("> DAY03 - part 2: OK!");
}
