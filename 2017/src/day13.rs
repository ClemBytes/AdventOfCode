use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY13 -------");
    let example = fs::read_to_string("inputs/example_day13").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day13").expect("Unable to read input!");
    let input = parse(&input);

    day13_part1(&example, &input);
    day13_part2(&example, &input);
}

fn parse(raw_input: &str) -> HashMap<u32, u32> {
    let mut firewall = HashMap::new();
    let r = Regex::new(r"^([0-9]+): ([0-9]+)$").unwrap();
    for line in raw_input.lines() {
        let matches = r.captures(line).unwrap();
        let depth = matches[1].parse::<u32>().unwrap();
        let range = matches[2].parse::<u32>().unwrap();
        firewall.insert(depth, range);
    }
    firewall
}

fn caught(picosecond: u32, firewall: &HashMap<u32, u32>) -> bool {
    if !firewall.contains_key(&picosecond) {
        return false;
    }
    
    true
}

fn severity(firewall: &HashMap<u32, u32>) -> u32 {
    0
}

fn day13_part1(example: &HashMap<u32, u32>, input: &HashMap<u32, u32>) {
    // Exemple tests
    assert!(caught(0, example));
    assert!(!caught(1, example));
    assert!(!caught(2, example));
    assert!(!caught(3, example));
    assert!(!caught(4, example));
    assert!(!caught(5, example));
    assert!(caught(6, example));
    assert_eq!(severity(example), 24);
    println!("Examples OK");

    // Solve puzzle
    let res = severity(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY13 - part 1: OK!");
}

fn day13_part2(_example: &HashMap<u32, u32>, _input: &HashMap<u32, u32>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY13 - part 2: OK!");
}
