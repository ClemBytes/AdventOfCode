use std::{collections::HashSet, fs};

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

fn day07_part2(_example: String, _input: String) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY07 - part 2: OK!");
}
