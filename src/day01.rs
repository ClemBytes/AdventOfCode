// DAY01: https://adventofcode.com/2015/day/1

use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY01 -------");
    let input = fs::read_to_string("inputs/input_day01").expect("Unable to read input!");
    day01_part1(&input);
    day01_part2(&input);
}

fn day01_part1(input: &str) {
    // Exemple tests
    assert_eq!(get_floor("(())"), 0);
    assert_eq!(get_floor("()()"), 0);
    assert_eq!(get_floor("((("), 3);
    assert_eq!(get_floor("(()(()("), 3);
    assert_eq!(get_floor("))((((("), 3);
    assert_eq!(get_floor("())"), -1);
    assert_eq!(get_floor("))("), -1);
    assert_eq!(get_floor(")))"), -3);
    assert_eq!(get_floor(")())())"), -3);

    // Solve puzzle
    assert_eq!(get_floor(input), 138);
    println!("> DAY01 - part 1: OK!");
}

fn day01_part2(input: &str) {
    // Exemple tests
    assert_eq!(basement_when(")"), 1);
    assert_eq!(basement_when("()())"), 5);

    // Solve puzzle
    assert_eq!(basement_when(input), 1771);
    println!("> DAY01 - part 2: OK!");
}

fn get_floor(instructions: &str) -> i32 {
    // Part 1
    let mut floor = 0;
    for parenthesis in instructions.chars() {
        match parenthesis {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("Unkwnow character {parenthesis}! Should be ( or )."),
        }
    }
    floor
    // We can do easier by counting number of '(', number of ')' and substract…
}

fn basement_when(instructions: &str) -> u32 {
    // Part 2
    let mut floor = 0;
    let mut position = 1;
    for parenthesis in instructions.chars() {
        match parenthesis {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("Unkwnow character {parenthesis}! Should be ( or )."),
        }
        if floor == -1 {
            return position;
        }
        position += 1;
    }
    unreachable!("Never found basement!");
}
