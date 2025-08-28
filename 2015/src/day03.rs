// DAY03: https://adventofcode.com/2015/day/3

use core::panic;
use std::collections::HashSet;
use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY03 -------");
    let input = fs::read_to_string("inputs/input_day03").unwrap();
    day03_part1(&input);
    day03_part2(&input);
}

fn day03_part1(input: &str) {
    // Exemple tests
    assert_eq!(get_visited_houses(">"), 2);
    assert_eq!(get_visited_houses("^>v<"), 4);
    assert_eq!(get_visited_houses("^v^v^v^v^v"), 2);

    // Solve puzzle
    // println!(
    //     "Result part 1: {}",
    //     get_visited_houses(&input)
    // );
    assert_eq!(get_visited_houses(input), 2081);
    println!("> DAY03 - part 1: OK!");
}

fn day03_part2(input: &str) {
    // Exemple tests
    assert_eq!(get_visited_houses_with_robot("^v"), 3);
    assert_eq!(get_visited_houses_with_robot("^>v<"), 3);
    assert_eq!(get_visited_houses_with_robot("^v^v^v^v^v"), 11);

    // Solve puzzle
    // println!(
    //     "Result part 2: {}",
    //     get_visited_houses_with_robot(&input)
    // );
    assert_eq!(get_visited_houses_with_robot(input), 2341);
    println!("> DAY03 - part 2: OK!");
}

fn get_visited_houses(input: &str) -> usize {
    let mut visited_houses = HashSet::new();
    let mut current_location = (0, 0);
    visited_houses.insert(current_location);
    for direction in input.chars() {
        match direction {
            '^' => current_location.0 += 1,
            'v' => current_location.0 -= 1,
            '<' => current_location.1 -= 1,
            '>' => current_location.1 += 1,
            other => panic!("Unknown direction: {other}"),
        }
        visited_houses.insert(current_location);
    }
    visited_houses.len()
}

fn get_visited_houses_with_robot(input: &str) -> usize {
    let mut visited_houses = HashSet::new();
    let mut santa = (0, 0);
    let mut robot = (0, 0);
    visited_houses.insert(santa);
    for (i, direction) in input.chars().enumerate() {
        let person = if i % 2 == 0 { &mut santa } else { &mut robot };
        match direction {
            '^' => person.0 += 1,
            'v' => person.0 -= 1,
            '<' => person.1 -= 1,
            '>' => person.1 += 1,
            other => panic!("Unknown direction: {other}"),
        }
        visited_houses.insert(*person);
    }
    visited_houses.len()
}
