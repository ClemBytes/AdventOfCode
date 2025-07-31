use core::panic;
use std::fs;
use std::collections::HashMap;

pub fn run() {
    println!("------- DAY03 -------");
    let input = fs::read_to_string("inputs/input_day03").unwrap();
    day03_part1(&input);
    day03_part2(&input);
}

fn day03_part1(input: &str) {
    // Exemple tests
    assert_eq!(nb_visited_houses(get_visited_houses(">")), 2);
    assert_eq!(nb_visited_houses(get_visited_houses("^>v<")), 4);
    assert_eq!(nb_visited_houses(get_visited_houses("^v^v^v^v^v")), 2);

    // Solve puzzle
    // println!("Result part 1: {}", nb_visited_houses(get_visited_houses(&input)));
    assert_eq!(nb_visited_houses(get_visited_houses(&input)), 2081);
    println!("> DAY03 - part 1: OK!");
}

fn day03_part2(input: &str) {
    // Exemple tests
    assert_eq!(nb_visited_houses(get_visited_houses_with_robot("^v")), 3);
    assert_eq!(nb_visited_houses(get_visited_houses_with_robot("^>v<")), 3);
    assert_eq!(nb_visited_houses(get_visited_houses_with_robot("^v^v^v^v^v")), 11);

    // Solve puzzle
    // println!(
    //     "Result part 2: {}",
    //     nb_visited_houses(get_visited_houses_with_robot(&input))
    // );
    assert_eq!(nb_visited_houses(get_visited_houses_with_robot(&input)), 2341);
    println!("> DAY03 - part 2: OK!");
}

fn get_visited_houses(input: &str) -> HashMap<(i32, i32), u32> {
    let mut visited_houses:HashMap<(i32, i32), u32> = HashMap::new();
    let mut current_location = (0, 0);
    visited_houses.insert(current_location, 1);
    for direction in input.chars() {
        match direction {
            '^' => { current_location.0 += 1; },
            'v' => { current_location.0 -= 1; },
            '<' => { current_location.1 -= 1; },
            '>' => { current_location.1 += 1; },
            other => panic!("Unknown direction: {other}"),
        }
        let count = visited_houses.entry(current_location).or_insert(0);
        *count += 1;
    }
    visited_houses
}

fn get_visited_houses_with_robot(input: &str) -> HashMap<(i32, i32), u32> {
    let mut visited_houses:HashMap<(i32, i32), u32> = HashMap::new();
    let mut current_location_santa = (0, 0);
    let mut current_location_robot = (0, 0);
    visited_houses.insert(current_location_santa, 2);
    for (i, direction) in input.chars().enumerate() {
        if i%2 == 0 {
            match direction {
                '^' => { current_location_santa.0 += 1; },
                'v' => { current_location_santa.0 -= 1; },
                '<' => { current_location_santa.1 -= 1; },
                '>' => { current_location_santa.1 += 1; },
                other => panic!("Unknown direction: {other}"),
            }
            let count = visited_houses
                .entry(current_location_santa)
                .or_insert(0);
            *count += 1;
        } else {
            match direction {
                '^' => { current_location_robot.0 += 1; },
                'v' => { current_location_robot.0 -= 1; },
                '<' => { current_location_robot.1 -= 1; },
                '>' => { current_location_robot.1 += 1; },
                other => panic!("Unknown direction: {other}"),
            }
            let count = visited_houses
                .entry(current_location_robot)
                .or_insert(0);
            *count += 1;
        }
    }
    visited_houses
}

fn nb_visited_houses(visited_houses: HashMap<(i32, i32), u32>) -> usize {
    visited_houses.len()
}