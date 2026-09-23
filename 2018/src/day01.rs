use std::collections::HashSet;
use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY01 -------");
    let input = fs::read_to_string("inputs/input_day01").expect("Unable to read input!");
    let input = parse(&input);

    day01_part1(&input);
    day01_part2(&input);
}

fn parse(raw_input: &str) -> Vec<i32> {
    let mut frequency_changes: Vec<i32> = vec![];
    for line in raw_input.lines() {
        let freq: i32 = line.parse().unwrap();
        frequency_changes.push(freq);
    }
    frequency_changes
}

fn day01_part1(input: &[i32]) {
    // Solve puzzle
    let res: i32 = input.iter().sum();
    println!("Result part 1: {res}");
    assert_eq!(res, 510);
    println!("> DAY01 - part 1: OK!");
}

fn found_twice(input: &[i32]) -> i32 {
    let input_len = input.len();
    let mut frequencies_reached: HashSet<i32> = HashSet::new();
    frequencies_reached.insert(0);
    let mut current_index = 0;
    let mut current_frequency = 0;
    loop {
        current_frequency += input[current_index];
        if frequencies_reached.contains(&current_frequency) {
            return current_frequency;
        }
        frequencies_reached.insert(current_frequency);
        current_index += 1;
        current_index %= input_len;
    }
}

fn day01_part2(input: &[i32]) {
    // Exemple tests
    let example: &[i32] = &[1, -1];
    assert_eq!(found_twice(example), 0);
    let example: &[i32] = &[3, 3, 4, -2, -4];
    assert_eq!(found_twice(example), 10);
    let example: &[i32] = &[-6, 3, 8, 5, -6];
    assert_eq!(found_twice(example), 5);
    let example: &[i32] = &[7, 7, -2, -7, -4];
    assert_eq!(found_twice(example), 14);
    println!("Example OK");

    // Solve puzzle
    let res = found_twice(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 69074);
    println!("> DAY01 - part 2: OK!");
}
