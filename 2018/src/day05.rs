use std::collections::HashSet;
use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY05 -------");
    let example = "dabAcCaCBAcCcaDA".to_string();
    let input = fs::read_to_string("inputs/input_day05")
        .expect("Unable to read input!")
        .trim()
        .to_string();

    day05_part1(&example, &input);
    day05_part2(&example, &input);
}

fn full_reaction_filter(polymer: &str, char_to_filter: char) -> usize {
    let lower_char_to_filter = char_to_filter
        .to_lowercase()
        .to_string()
        .chars()
        .next()
        .unwrap();
    let mut stack: Vec<char> = vec![];
    let polymer_vec: Vec<char> = polymer.chars().collect::<Vec<char>>();
    for unit in polymer_vec {
        if char_to_filter != ' ' {
            let lower_unit = unit.to_lowercase().to_string().chars().next().unwrap();
            if lower_unit == lower_char_to_filter {
                continue;
            }
        }
        let size_stack = stack.len();
        if size_stack > 0 {
            let last_char = stack[size_stack - 1];
            if last_char.eq_ignore_ascii_case(&unit) && last_char != unit {
                stack.pop();
                continue;
            }
        }
        stack.push(unit);
    }
    stack.len()
}

fn day05_part1(example: &str, input: &str) {
    // Exemple tests
    assert_eq!(full_reaction_filter(example, ' '), 10);
    println!("Example OK");

    // Solve puzzle
    let res = full_reaction_filter(input, ' ');
    println!("Result part 1: {res}");
    assert_eq!(res, 10766);
    println!("> DAY05 - part 1: OK!");
}

fn len_shortest_polymer(polymer: &str) -> usize {
    let polymer_vec: Vec<char> = polymer.chars().collect::<Vec<char>>();
    let polymer_set = polymer_vec
        .into_iter()
        .map(|c: char| c.to_lowercase().to_string().chars().next().unwrap())
        .collect::<HashSet<char>>();
    let mut min_polymer_size = usize::MAX;
    for c in polymer_set {
        let len_reduced_polymer = full_reaction_filter(polymer, c);
        if len_reduced_polymer < min_polymer_size {
            min_polymer_size = len_reduced_polymer;
        }
    }
    min_polymer_size
}

fn day05_part2(example: &str, input: &str) {
    // Exemple tests
    assert_eq!(len_shortest_polymer(example), 4);
    println!("Example OK");

    // Solve puzzle
    let res = len_shortest_polymer(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 6538);
    println!("> DAY05 - part 2: OK!");
}
