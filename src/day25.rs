use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY25 -------");
    let example: (u64, u64) = (2, 1); // row 2, column 1
    let input = fs::read_to_string("inputs/input_day25").expect("Unable to read input!");
    let input = parse(&input);

    day25(example, input);
}

const FIRST_CODE: u64 = 20151125;
const FACTOR: u64 = 252533;
const DIVIDER: u64 = 33554393;

fn parse(raw_input: &str) -> (u64, u64) {
    let re = Regex::new(r"^To continue, please consult the code grid in the manual\.  Enter the code at row ([0-9]+), column ([0-9]+)\.$").unwrap();
    let matches = re.captures(raw_input.trim()).unwrap();
    let row: u64 = matches[1].parse().unwrap();
    let col: u64 = matches[2].parse().unwrap();
    (row, col)
}

fn code_number(coords: (u64, u64)) -> u64 {
    let (row, col) = coords;
    if row == 1 {
        return (col * (col + 1)) / 2;
    }
    code_number((row - 1, col + 1)) - 1
}

#[test]
fn test_code_number() {
    assert_eq!(code_number((1, 1)), 1);
    assert_eq!(code_number((1, 2)), 3);
    assert_eq!(code_number((1, 3)), 6);
    assert_eq!(code_number((1, 4)), 10);
    assert_eq!(code_number((1, 5)), 15);
    assert_eq!(code_number((1, 6)), 21);
    assert_eq!(code_number((2, 1)), 2);
    assert_eq!(code_number((2, 2)), 5);
    assert_eq!(code_number((2, 3)), 9);
    assert_eq!(code_number((2, 4)), 14);
    assert_eq!(code_number((2, 5)), 20);
    assert_eq!(code_number((3, 1)), 4);
    assert_eq!(code_number((3, 2)), 8);
    assert_eq!(code_number((3, 3)), 13);
    assert_eq!(code_number((3, 4)), 19);
    assert_eq!(code_number((4, 1)), 7);
    assert_eq!(code_number((4, 2)), 12);
    assert_eq!(code_number((4, 3)), 18);
    assert_eq!(code_number((5, 1)), 11);
    assert_eq!(code_number((5, 2)), 17);
    assert_eq!(code_number((6, 1)), 16);
}

fn find_code(coords: (u64, u64)) -> u64 {
    let n = code_number(coords);
    let mut code = FIRST_CODE;
    for _ in 0..n - 1 {
        code = (code * FACTOR) % DIVIDER;
    }
    code
}

fn day25(example: (u64, u64), input: (u64, u64)) {
    // Exemple tests
    println!("Example: row = {} | col = {}", example.0, example.1);
    let code_number_example = code_number(example);
    assert_eq!(code_number_example, 2);
    let res = find_code(example);
    assert_eq!(res, 31916031);

    // Solve puzzle
    let code_number_input = code_number(input);
    println!(
        "Input: row = {} | col = {} | code_number = {}",
        input.0, input.1, code_number_input
    );
    let res = find_code(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 19980801);
    println!("> DAY25 - part 1: OK!");
}
