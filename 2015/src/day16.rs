use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY16 -------");
    let input = fs::read_to_string("inputs/input_day16").expect("Unable to read input!");
    let input = parse(&input);

    let mut machine_result: HashMap<String, u32> = HashMap::new();
    machine_result.insert(String::from("children"), 3);
    machine_result.insert(String::from("cats"), 7);
    machine_result.insert(String::from("samoyeds"), 2);
    machine_result.insert(String::from("pomeranians"), 3);
    machine_result.insert(String::from("akitas"), 0);
    machine_result.insert(String::from("vizslas"), 0);
    machine_result.insert(String::from("goldfish"), 5);
    machine_result.insert(String::from("trees"), 3);
    machine_result.insert(String::from("cars"), 2);
    machine_result.insert(String::from("perfumes"), 1);

    day16_part1(&input, machine_result.clone());
    day16_part2(&input, machine_result);
}

fn parse(raw_input: &str) -> Vec<HashMap<String, u32>> {
    let mut sues: Vec<HashMap<String, u32>> = vec![];
    let re = Regex::new(r"^Sue [0-9]+: ([a-z]+): ([0-9]+), ([a-z]+): ([0-9]+), ([a-z]+): ([0-9]+)")
        .unwrap();
    for line in raw_input.lines() {
        let mut sue: HashMap<String, u32> = HashMap::new();
        let matches = re.captures(line).unwrap();
        let thing_name = matches[1].to_string();
        let thing_nb: u32 = matches[2].parse().unwrap();
        sue.insert(thing_name, thing_nb);
        let thing_name = matches[3].to_string();
        let thing_nb: u32 = matches[4].parse().unwrap();
        sue.insert(thing_name, thing_nb);
        let thing_name = matches[5].to_string();
        let thing_nb: u32 = matches[6].parse().unwrap();
        sue.insert(thing_name, thing_nb);
        sues.push(sue);
    }
    sues
}

fn find_sue_number_part1(
    input: &[HashMap<String, u32>],
    machine_result: HashMap<String, u32>,
) -> usize {
    'next_sue: for (i, sue) in input.iter().enumerate() {
        for (thing_name, thing_nb) in sue {
            if !machine_result.contains_key(thing_name) {
                continue 'next_sue;
            }
            if machine_result.get(thing_name).unwrap() != thing_nb {
                continue 'next_sue;
            }
        }
        return i + 1;
    }
    unreachable!("Never found Aunt Sue!");
}

fn find_sue_number_part2(
    input: &[HashMap<String, u32>],
    machine_result: HashMap<String, u32>,
) -> usize {
    'next_sue: for (i, sue) in input.iter().enumerate() {
        for (thing_name, thing_nb) in sue {
            if !machine_result.contains_key(thing_name) {
                continue 'next_sue;
            }
            if thing_name == "cats" || thing_name == "trees" {
                if thing_nb <= machine_result.get(thing_name).unwrap() {
                    continue 'next_sue;
                }
            } else if thing_name == "pomeranians" || thing_name == "goldfish" {
                if thing_nb >= machine_result.get(thing_name).unwrap() {
                    continue 'next_sue;
                }
            } else if machine_result.get(thing_name).unwrap() != thing_nb {
                continue 'next_sue;
            }
        }
        return i + 1;
    }
    unreachable!("Never found Aunt Sue!");
}

fn day16_part1(input: &[HashMap<String, u32>], machine_result: HashMap<String, u32>) {
    // Solve puzzle
    let res = find_sue_number_part1(input, machine_result);
    println!("Result part 1: {res}");
    assert_eq!(res, 40);
    println!("> DAY16 - part 1: OK!");
}

fn day16_part2(input: &[HashMap<String, u32>], machine_result: HashMap<String, u32>) {
    // Solve puzzle
    let res = find_sue_number_part2(input, machine_result);
    println!("Result part 2: {res}");
    assert_eq!(res, 241);
    println!("> DAY16 - part 1: OK!");
}
