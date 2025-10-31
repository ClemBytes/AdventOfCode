use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY12 -------");
    let example = fs::read_to_string("inputs/example_day12").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day12").expect("Unable to read input!");
    let input = parse(&input);

    day12_part1(&example, &input);
    day12_part2(&example, &input);
}

fn parse(raw_input: &str) -> HashMap<u32, Vec<u32>> {
    let mut pipes: HashMap<u32, Vec<u32>> = HashMap::new();
    let r = Regex::new(r"^([0-9]+) <-> ([0-9, ]+)$").unwrap();
    for line in raw_input.lines() {
        let matches = r.captures(line).unwrap();
        let id: u32 = matches[1].parse().unwrap();
        let connected_to = &matches[2];
        let connected_to: Vec<&str> = connected_to.split(", ").collect();
        let connected_to: Vec<u32> = connected_to
            .iter()
            .map(|&x| x.parse::<u32>().unwrap())
            .collect();
        pipes.insert(id, connected_to);
    }
    pipes
}

fn size_group(id: u32, pipes: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut to_check: VecDeque<u32> = VecDeque::new();
    let mut visited: HashSet<u32> = HashSet::new();
    let id_list = pipes.get(&id).unwrap();
    for &i in id_list {
        to_check.push_back(i);
    }
    visited.insert(id);
    while let Some(new_id) = to_check.pop_front() {
        if visited.contains(&new_id) {
            continue;
        }
        let id_list = pipes.get(&new_id).unwrap();
        for &i in id_list {
            to_check.push_back(i);
        }
        visited.insert(new_id);
    }
    visited.len() as u32
}

fn day12_part1(example: &HashMap<u32, Vec<u32>>, input: &HashMap<u32, Vec<u32>>) {
    // Exemple tests
    assert_eq!(size_group(0, example), 6);

    // Solve puzzle
    let res = size_group(0, input);
    println!("Result part 1: {res}");
    assert_eq!(res, 145);
    println!("> DAY12 - part 1: OK!");
}

fn day12_part2(_example: &HashMap<u32, Vec<u32>>, _input: &HashMap<u32, Vec<u32>>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY12 - part 2: OK!");
}
