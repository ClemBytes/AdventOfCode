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

fn find_lowest_valued_ip(input: Vec<(u32, u32)>) -> u32 {
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

fn find_nb_allowed_ips(input: Vec<(u32, u32)>, max_valid: u32) -> u32 {
    let mut input_sorted = input.clone();
    input_sorted.sort();
    let mut current_max = input_sorted[0].1;
    let mut nb_allowed_ips = input_sorted[0].0;
    for intervall in input_sorted {
        if intervall.0 <= current_max {
            current_max = current_max.max(intervall.1);
        } else {
            nb_allowed_ips += intervall.0 - current_max - 1;
            current_max = intervall.1;
        }
    }
    let to_add = max_valid - current_max;
    nb_allowed_ips + to_add
}

fn day20_part1(example: &[(u32, u32)], input: &[(u32, u32)]) {
    // Exemple tests
    assert_eq!(find_lowest_valued_ip(example.to_vec()), 3);
    println!("Example OK");

    // Solve puzzle
    let res = find_lowest_valued_ip(input.to_vec());
    println!("Result part 1: {res}");
    assert_eq!(res, 19449262);
    println!("> DAY20 - part 1: OK!");
}

fn day20_part2(example: &[(u32, u32)], input: &[(u32, u32)]) {
    // Exemple tests
    assert_eq!(find_nb_allowed_ips(example.to_vec(), 9), 2);
    println!("Example OK");

    // Solve puzzle
    let res = find_nb_allowed_ips(input.to_vec(), u32::MAX);
    println!("Result part 2: {res}");
    assert_eq!(res, 119);
    println!("> DAY20 - part 2: OK!");
}
