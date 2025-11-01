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

fn caught(picosecond: u32, depth: u32, firewall: &HashMap<u32, u32>) -> bool {
    if !firewall.contains_key(&depth) {
        return false;
    }
    let &range = firewall.get(&depth).unwrap();
    // println!("picosecond: {picosecond} | range: {range} | (range * 2 - 2): {}", (range * 2 - 2));
    if (picosecond == 0) && (depth == 0) {
        return true;
    }
    if picosecond % (range * 2 - 2) == 0 {
        return true;
    }
    false
}

fn severity(firewall: &HashMap<u32, u32>, delay: u32) -> u32 {
    let mut sev = 0;
    for (&depth, &range) in firewall {
        let picosecond = depth + delay;
        if caught(picosecond, depth, firewall) {
            sev += picosecond * range;
        }
    }
    sev
}

fn day13_part1(example: &HashMap<u32, u32>, input: &HashMap<u32, u32>) {
    // Exemple tests
    assert!(caught(0, 0, example));
    assert!(!caught(1, 1, example));
    assert!(!caught(2, 2, example));
    assert!(!caught(3, 3, example));
    assert!(!caught(4, 4, example));
    assert!(!caught(5, 5, example));
    assert!(caught(6, 6, example));
    assert_eq!(severity(example, 0), 24);

    // Solve puzzle
    let res = severity(input, 0);
    println!("Result part 1: {res}");
    assert_eq!(res, 1840);
    println!("> DAY13 - part 1: OK!");
}

fn smallest_delay(firewall: &HashMap<u32, u32>) -> u32 {
    let mut delay = 0;
    'big_loop: loop {
        for &depth in firewall.keys() {
            let picosecond = depth + delay;
            if caught(picosecond, depth, firewall) {
                delay += 1;
                continue 'big_loop;
            }
        }
        return delay;
    }
}

fn day13_part2(example: &HashMap<u32, u32>, input: &HashMap<u32, u32>) {
    // Exemple tests
    assert_eq!(smallest_delay(example), 10);

    // Solve puzzle
    let res = smallest_delay(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 3850260);
    println!("> DAY13 - part 2: OK!");
}
