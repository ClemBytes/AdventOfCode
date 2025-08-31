use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY21 -------");
    let example = fs::read_to_string("inputs/example_day21").expect("Unable to read input!");
    let example = Operation::from_str(&example);
    let input = fs::read_to_string("inputs/input_day21").expect("Unable to read input!");
    let input = Operation::from_str(&input);

    day21_part1(&example, &input);
    day21_part2(&example, &input);
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePosition(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn from_str(raw_input: &str) -> Vec<Self> {
        let mut operations = vec![];
        let re_swap_position =
            Regex::new(r"^swap position ([0-9]+) with position ([0-9]+)$").unwrap();
        let re_swap_letter = Regex::new(r"^swap letter ([a-z]) with letter ([a-z])$").unwrap();
        let re_rotate_left = Regex::new(r"^rotate left ([0-9]+) steps?$").unwrap();
        let re_rotate_right = Regex::new(r"^rotate right ([0-9]+) steps?$").unwrap();
        let re_rotate_position =
            Regex::new(r"^rotate based on position of letter ([a-z])$").unwrap();
        let re_reverse = Regex::new(r"^reverse positions ([0-9]+) through ([0-9]+)$").unwrap();
        let re_move = Regex::new(r"^move position ([0-9]+) to position ([0-9]+)$").unwrap();
        for line in raw_input.lines() {
            if let Some(matches) = re_swap_position.captures(line) {
                operations.push(Operation::SwapPosition(
                    matches[1].parse().unwrap(),
                    matches[2].parse().unwrap(),
                ));
            } else if let Some(matches) = re_swap_letter.captures(line) {
                operations.push(Operation::SwapLetter(
                    matches[1].parse().unwrap(),
                    matches[2].parse().unwrap(),
                ));
            } else if let Some(matches) = re_rotate_left.captures(line) {
                operations.push(Operation::RotateLeft(matches[1].parse().unwrap()));
            } else if let Some(matches) = re_rotate_right.captures(line) {
                operations.push(Operation::RotateRight(matches[1].parse().unwrap()));
            } else if let Some(matches) = re_rotate_position.captures(line) {
                operations.push(Operation::RotatePosition(matches[1].parse().unwrap()));
            } else if let Some(matches) = re_reverse.captures(line) {
                operations.push(Operation::Reverse(
                    matches[1].parse().unwrap(),
                    matches[2].parse().unwrap(),
                ));
            } else if let Some(matches) = re_move.captures(line) {
                operations.push(Operation::Move(
                    matches[1].parse().unwrap(),
                    matches[2].parse().unwrap(),
                ));
            } else {
                panic!("Unknown operation: {line}");
            }
        }
        operations
    }

    fn apply(&self, s: String) -> String {
        let mut new: Vec<char> = s.clone().chars().collect();
        match *self {
            Operation::SwapPosition(x, y) => {
                new.swap(x, y);
            }
            Operation::SwapLetter(a, b) => {
                let x = new.iter().position(|&l| l == a).unwrap();
                let y = new.iter().position(|&l| l == b).unwrap();
                new.swap(x, y);
            }
            Operation::RotateLeft(x) => {
                new.rotate_left(x);
            }
            Operation::RotateRight(x) => {
                new.rotate_right(x);
            }
            Operation::RotatePosition(a) => {
                let mut x = new.iter().position(|&l| l == a).unwrap() + 1;
                if x > 4 {
                    x += 1;
                }
                x %= new.len();
                new.rotate_right(x);
            }
            Operation::Reverse(x, y) => {
                new[x..=y].reverse();
            }
            Operation::Move(x, y) => {
                let a = new.remove(x);
                new.insert(y, a);
            }
        }
        new.iter().collect()
    }
}

fn solve_part1(operations: &[Operation], start: String, print: bool) -> String {
    let mut s = start.clone();
    for operation in operations {
        s = operation.apply(s);
        if print {
            println!("{s}");
        }
    }
    s
}

fn day21_part1(example: &[Operation], input: &[Operation]) {
    // Exemple tests
    assert_eq!(
        Operation::apply(&Operation::SwapPosition(4, 0), "abcde".to_string()),
        "ebcda"
    );
    assert_eq!(
        Operation::apply(&Operation::SwapLetter('d', 'b'), "ebcda".to_string()),
        "edcba"
    );
    assert_eq!(
        Operation::apply(&Operation::Reverse(0, 4), "edcba".to_string()),
        "abcde"
    );
    assert_eq!(
        Operation::apply(&Operation::RotateLeft(1), "abcde".to_string()),
        "bcdea"
    );
    assert_eq!(
        Operation::apply(&Operation::Move(1, 4), "bcdea".to_string()),
        "bdeac"
    );
    assert_eq!(
        Operation::apply(&Operation::Move(3, 0), "bdeac".to_string()),
        "abdec"
    );
    assert_eq!(
        Operation::apply(&Operation::RotatePosition('b'), "abdec".to_string()),
        "ecabd"
    );
    assert_eq!(
        Operation::apply(&Operation::RotatePosition('d'), "ecabd".to_string()),
        "decab"
    );
    assert_eq!(solve_part1(example, "abcde".to_string(), true), "decab");
    println!("Example OK");

    // Solve puzzle
    let res = solve_part1(input, "abcdefgh".to_string(), false);
    println!("Result part 1: {res}");
    assert_eq!(res, "aefgbcdh");
    println!("> DAY21 - part 1: OK!");
}

fn day21_part2(_example: &[Operation], _input: &[Operation]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY21 - part 2: OK!");
}
