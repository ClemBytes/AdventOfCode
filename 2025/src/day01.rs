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

    fn bad_password(rotations: &[Self], max_dial: i32, start_position: i32) -> i32 {
        let mut current_position = start_position;
        let mut nb_zeros_pointed = 0;
        if current_position == 0 {
            nb_zeros_pointed += 1;
        }
        for rotation in rotations {
            current_position = rotation.apply(max_dial, current_position);
            if current_position == 0 {
                nb_zeros_pointed += 1;
            }
        }
        nb_zeros_pointed
    }

    /* AGROUGROU BOUUUUH
    fn good_password(rotations: &[Self], max_dial: i32, start_position: i32) -> i32 {
        let mut current_position = start_position;
        let mut nb_zeros_clicked = 0;
        if current_position == 0 {
            nb_zeros_clicked += 1;
        }
        for rotation in rotations {
            match rotation {
                Rotation::Left(i) => {
                    let new_position_without_modulo = current_position - *i;
                    if new_position_without_modulo <= 0 {
                        if current_position != 0 {
                            nb_zeros_clicked += 1;
                        }
                        let nb_more_zeros = new_position_without_modulo.abs() / (max_dial + 1);
                        nb_zeros_clicked += nb_more_zeros;
                    }
                },
                Rotation::Right(i) => {
                    let new_position_without_modulo = current_position + *i;
                    if new_position_without_modulo > max_dial {
                        nb_zeros_clicked += 1;
                        let nb_more_zeros = (new_position_without_modulo - max_dial) / (max_dial + 1);
                        nb_zeros_clicked += nb_more_zeros;
                    }
                },
            }
            let new_position = rotation.apply(max_dial, current_position);
            // println!("rotation: {rotation:?} | nb_zeros_clicked: {nb_zeros_clicked} | current_position: {current_position} | new_position: {new_position}");
            current_position = new_position;
        }
        nb_zeros_clicked
    }
    */

    fn stupid_good_password(rotations: &[Self], max_dial: i32, start_position: i32) -> i32 {
        let mut current_position = start_position;
        let mut nb_zeros_clicked = 0;
        if current_position == 0 {
            nb_zeros_clicked += 1;
        }
        for rotation in rotations {
            match rotation {
                Rotation::Left(i) => {
                    for _ in 0..*i {
                        current_position -= 1;
                        if current_position == 0 {
                            nb_zeros_clicked += 1;
                        }
                        if current_position == -1 {
                            current_position = max_dial;
                        }
                    }
                }
                Rotation::Right(i) => {
                    for _ in 0..*i {
                        current_position += 1;
                        if current_position == max_dial + 1 {
                            current_position = 0;
                            nb_zeros_clicked += 1;
                        }
                    }
                }
            }
        }
        nb_zeros_clicked
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
    assert_eq!(Rotation::bad_password(example, 99, 50), 3);

    // Solve puzzle
    let res = Rotation::bad_password(input, 99, 50);
    println!("Result part 1: {res}");
    assert_eq!(res, 1084);
    println!("> DAY01 - part 1: OK!");
}

fn day01_part2(example: &[Rotation], input: &[Rotation]) {
    // Exemple tests
    assert_eq!(
        Rotation::stupid_good_password(&[Rotation::Right(10)], 99, 95),
        1
    );
    assert_eq!(
        Rotation::stupid_good_password(&[Rotation::Left(10)], 99, 5),
        1
    );
    assert_eq!(
        Rotation::stupid_good_password(&[Rotation::Right(100)], 99, 0),
        2
    );
    assert_eq!(
        Rotation::stupid_good_password(&[Rotation::Right(1000)], 99, 50),
        10
    );
    assert_eq!(
        Rotation::stupid_good_password(&[Rotation::Left(1000)], 99, 50),
        10
    );
    assert_eq!(Rotation::stupid_good_password(example, 99, 50), 6);

    // Solve puzzle
    let res = Rotation::stupid_good_password(input, 99, 50);
    println!("Result part 2: {res}");
    assert_eq!(res, 6475);
    println!("> DAY01 - part 2: OK!");
}
