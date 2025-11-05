use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY15 -------");
    let example = (65, 8921);
    let input = fs::read_to_string("inputs/input_day15").expect("Unable to read input!");
    let input = parse(&input);

    day15_part1(example, input);
    day15_part2(example, input);
}

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const DIVIDER: u64 = 2147483647;

fn parse(raw_input: &str) -> (u64, u64) {
    let mut a = 0;
    let mut b = 0;
    let r = Regex::new(r"^Generator (A|B) starts with ([0-9]+)$").unwrap();
    for line in raw_input.lines() {
        let matches = r.captures(line).unwrap();
        let name = &matches[1];
        let val = matches[2].parse().unwrap();
        match name {
            "A" => a = val,
            "B" => b = val,
            other => {
                unreachable!("Unknown generator name: {other}")
            }
        }
    }
    (a, b)
}

fn judge_final_count(nb_pairs: u64, input: (u64, u64)) -> u64 {
    let mut count = 0;
    let (mut a, mut b) = input;
    for _ in 1..=nb_pairs {
        a = (a * FACTOR_A) % DIVIDER;
        b = (b * FACTOR_B) % DIVIDER;
        if (a & 0xFFFF) == (b & 0xFFFF) {
            count += 1;
        }
    }
    count
}

fn day15_part1(example: (u64, u64), input: (u64, u64)) {
    // let a = example.0;
    // let a_3 = (((((a * FACTOR_A) % DIVIDER) * FACTOR_A) % DIVIDER) * FACTOR_A) % DIVIDER;
    // let a_3_16 = (a_3)&0xFFFF;
    // println!("example A: {} | {} | {} | {:b} ", a, a_3, a_3_16, a_3_16);
    // let b = example.1;
    // let b_3 = (((((b * FACTOR_B) % DIVIDER) * FACTOR_B) % DIVIDER) * FACTOR_B) % DIVIDER;
    // let b_3_16 = (b_3)&0xFFFF;
    // println!("example B: {} | {} | {} | {:b} ", b, b_3, b_3_16, b_3_16);
    // println!("input: {input:?}");

    // Exemple tests
    assert_eq!(judge_final_count(5, example), 1);
    assert_eq!(judge_final_count(40_000_000, example), 588);

    // Solve puzzle
    let res = judge_final_count(40_000_000, input);
    println!("Result part 1: {res}");
    assert_eq!(res, 573);
    println!("> DAY15 - part 1: OK!");
}

fn day15_part2(_example: (u64, u64), _input: (u64, u64)) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY15 - part 2: OK!");
}
