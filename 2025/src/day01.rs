use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY01 -------");
    let example = fs::read_to_string("inputs/example_day01").expect("Unable to read input!");
    let example = Rotation::parse(&example);
    let input = fs::read_to_string("inputs/input_day01").expect("Unable to read input!");
    let input = Rotation::parse(&input);

    day01_part1(&example, &input);
    day01_part2(&example, &input);
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl Rotation {
    fn parse(raw_input: &str) -> Vec<Self> {
        let mut rotations = vec![];
        let r = Regex::new(r"^(R|L)([0-9]+)$").unwrap();
        for line in raw_input.lines() {
            let matches = r.captures(line).unwrap();
            let nb_clicks = matches[2].parse().unwrap();
            let rotation = match &matches[1] {
                "R" => Rotation::Right(nb_clicks),
                "L" => Rotation::Left(nb_clicks),
                other => unreachable!("Unknown rotation: '{other}', should be 'R' or 'L'!"),
            };
            rotations.push(rotation);
        }
        rotations
    }

    fn apply(&self, max_dial: i32, start_position: i32) -> i32 {
        match self {
            Rotation::Left(i) => (start_position - i).rem_euclid(max_dial + 1),
            Rotation::Right(i) => (start_position + i).rem_euclid(max_dial + 1),
        }
    }

    fn actual_password(rotations: &[Self], max_dial: i32, start_position: i32) -> i32 {
        let mut current_position = start_position;
        let mut nb_zeros = 0;
        if current_position == 0 {
            nb_zeros += 1;
        }
        for rotation in rotations {
            current_position = rotation.apply(max_dial, current_position);
            if current_position == 0 {
                nb_zeros += 1;
            }
        }
        nb_zeros
    }
}

fn day01_part1(example: &[Rotation], input: &[Rotation]) {
    // Exemple tests
    assert_eq!(Rotation::Right(8).apply(99, 11), 19);
    assert_eq!(Rotation::Left(19).apply(99, 19), 0);
    assert_eq!(Rotation::Right(1).apply(99, 99), 0);
    assert_eq!(Rotation::Left(1).apply(99, 0), 99);
    assert_eq!(Rotation::Left(10).apply(99, 5), 95);
    assert_eq!(Rotation::Right(5).apply(99, 95), 0);
    assert_eq!(Rotation::actual_password(example, 99, 50), 3);

    // Solve puzzle
    let res = Rotation::actual_password(input, 99, 50);
    println!("Result part 1: {res}");
    assert_eq!(res, 1084);
    println!("> DAY01 - part 1: OK!");
}

fn day01_part2(_example: &[Rotation], _input: &[Rotation]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY01 - part 2: OK!");
}
