use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY07 -------");
    let example = fs::read_to_string("inputs/example_day07").expect("Unable to read input!");
    let input = fs::read_to_string("inputs/input_day07").expect("Unable to read input!");

    day07_part1(example.clone(), input.clone());
    day07_part2(example, input);
}

fn count_splits(diagram: String) -> usize {
    let mut nb_splits = 0;
    let mut current_beams = HashSet::new();
    for line in diagram.lines() {
        let mut new_beams = HashSet::new();
        for (x, c) in line.chars().enumerate() {
            // First line with source S
            if c == 'S' {
                new_beams.insert(x);
                break;
            }

            // Other lines
            if current_beams.contains(&x) {
                if c == '.' {
                    new_beams.insert(x);
                } else if c == '^' {
                    new_beams.insert(x - 1);
                    new_beams.insert(x + 1);
                    nb_splits += 1;
                }
            }
        }
        current_beams = new_beams;
    }
    nb_splits
}

fn day07_part1(example: String, input: String) {
    // Exemple tests
    assert_eq!(count_splits(example), 21);

    // Solve puzzle
    let res = count_splits(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 1711);
    println!("> DAY07 - part 1: OK!");
}

fn count_timelines(diagram: String) -> usize {
    let mut current_beams = HashMap::new();
    for line in diagram.lines() {
        let mut new_beams = HashMap::new();
        for (x, c) in line.chars().enumerate() {
            // First line with source S
            if c == 'S' {
                new_beams.insert(x, 1);
                break;
            }

            // Other lines
            if current_beams.contains_key(&x) {
                let x_val = *current_beams.get(&x).unwrap();
                if c == '.' {
                    let count = new_beams.entry(x).or_insert(0);
                    *count += x_val;
                } else if c == '^' {
                    let count = new_beams.entry(x - 1).or_insert(0);
                    *count += x_val;
                    let count = new_beams.entry(x + 1).or_insert(0);
                    *count += x_val;
                }
            }
        }
        current_beams = new_beams;
    }
    current_beams.values().sum()
}

fn day07_part2(example: String, input: String) {
    // Exemple tests
    assert_eq!(count_timelines(example), 40);

    // Solve puzzle
    let res = count_timelines(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 36706966158365);
    println!("> DAY07 - part 2: OK!");
}
