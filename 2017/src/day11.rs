use std::{collections::HashSet, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY11 -------");
    let binding = fs::read_to_string("inputs/input_day11").expect("Unable to read input!");
    let input = binding.trim();
    let input = parse(input);

    day11_part1(&input);
    day11_part2(&input);
}

#[derive(Debug)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

fn parse(raw_input: &str) -> Vec<Direction> {
    let mut directions = vec![];
    let raw_directions: Vec<&str> = raw_input.split(",").collect();
    for dir in raw_directions {
        let d = match dir {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "s" => Direction::S,
            "sw" => Direction::SW,
            "nw" => Direction::NW,
            other => unreachable!("Unkwnon direction: {other}"),
        };
        directions.push(d);
    }
    directions
}

fn move_one_direction(start: (i32, i32, i32), direction: &Direction) -> (i32, i32, i32) {
    let (mut x, mut y, mut z) = start;
    match direction {
        Direction::N => {
            x += 1;
            z -= 1;
        }
        Direction::NE => {
            x += 1;
            y += 1;
        }
        Direction::SE => {
            y += 1;
            z += 1;
        }
        Direction::S => {
            x -= 1;
            z += 1;
        }
        Direction::SW => {
            x -= 1;
            y -= 1;
        }
        Direction::NW => {
            y -= 1;
            z -= 1;
        }
    }
    (x, y, z)
}

fn fewer_steps(child_path: &[Direction]) -> i32 {
    let mut position = (0, 0, 0);
    for direction in child_path {
        position = move_one_direction(position, direction);
    }
    position.0.abs().max(position.1.abs()).max(position.2.abs())
}

fn day11_part1(input: &[Direction]) {
    // Exemple tests
    let ex = vec![Direction::NE, Direction::NE, Direction::NE];
    assert_eq!(fewer_steps(&ex), 3);
    let ex = vec![Direction::NE, Direction::NE, Direction::SW, Direction::SW];
    assert_eq!(fewer_steps(&ex), 0);
    let ex = vec![Direction::NE, Direction::NE, Direction::S, Direction::S];
    assert_eq!(fewer_steps(&ex), 2);
    let ex = vec![
        Direction::SE,
        Direction::SW,
        Direction::SE,
        Direction::SW,
        Direction::SW,
    ];
    assert_eq!(fewer_steps(&ex), 3);

    // Solve puzzle
    let res = fewer_steps(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 810);
    println!("> DAY11 - part 1: OK!");
}

fn steps_further(child_path: &[Direction]) -> i32 {
    let mut position = (0, 0, 0);
    let mut steps_away = HashSet::new();
    for direction in child_path {
        position = move_one_direction(position, direction);
        steps_away.insert(position.0.abs().max(position.1.abs()).max(position.2.abs()));
    }
    *steps_away.iter().max().unwrap()
}

fn day11_part2(input: &[Direction]) {
    // Solve puzzle
    let res = steps_further(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 1567);
    println!("> DAY11 - part 2: OK!");
}
