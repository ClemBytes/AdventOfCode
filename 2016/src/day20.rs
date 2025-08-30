use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY20 -------");
    let example = fs::read_to_string("inputs/example_day20").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day20").expect("Unable to read input!");
    let input = parse(&input);

    day20_part1(&example, &input);
    day20_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"^([0-9]+)-([0-9]+)$").unwrap();
    let mut ranges = vec![];
    for line in raw_input.lines() {
        let matches = re.captures(line).unwrap();
        ranges.push((matches[1].parse().unwrap(), matches[2].parse().unwrap()));
    }
    ranges
}

fn find_lowest_valued_ip(input: Vec<(u32, u32)>, _max_valid: u32) -> u32 {
    let mut input_sorted = input.clone();
    input_sorted.sort();
    let mut min_value = 0;
    for (low, high) in input_sorted {
        if low <= min_value {
            min_value = high + 1;
        }
    }
    min_value
}

fn day20_part1(example: &[(u32, u32)], input: &[(u32, u32)]) {
    // Exemple tests
    assert_eq!(find_lowest_valued_ip(example.to_vec(), 9), 3);
    println!("Example OK");

    // Solve puzzle
    let res = find_lowest_valued_ip(input.to_vec(), u32::MAX);
    println!("Result part 1: {res}");
    assert_eq!(res, 19449262);
    println!("> DAY20 - part 1: OK!");
}

fn day20_part2(_example: &[(u32, u32)], _input: &[(u32, u32)]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY20 - part 2: OK!");
}
