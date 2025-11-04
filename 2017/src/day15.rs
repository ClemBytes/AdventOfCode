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

fn parse(raw_input: &str) -> (u32, u32) {
    let mut a = 0;
    let mut b = 0;
    let r = Regex::new(r"^Generator (A|B) starts with ([0-9]+)$").unwrap();
    for line in raw_input.lines() {
        let matches = r.captures(line).unwrap();
        let name = &matches[1];
        let val = matches[2].parse().unwrap();
        match name {
            "A" => {a = val},
            "B" => {b = val},
            other => {unreachable!("Unknown generator name: {other}")},
        }
    }
    (a, b)
}

fn day15_part1(_example: (u32, u32), _input: (u32, u32)) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY15 - part 1: OK!");
}

fn day15_part2(_example: (u32, u32), _input: (u32, u32)) {
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
