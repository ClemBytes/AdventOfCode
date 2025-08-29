use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY15 -------");
    let example = fs::read_to_string("inputs/example_day15").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day15").expect("Unable to read input!");
    let input = parse(&input);

    day15_part1(&example, &input);
    day15_part2(&example, &input);
}

#[derive(Debug, Copy, Clone)]
struct Disc {
    nb_positions: u32,
    start_position: u32,
}

impl Disc {
    fn from_str(s: &str) -> Self {
        let re = Regex::new(
            r"^Disc #[0-9]+ has ([0-9]+) positions; at time=0, it is at position ([0-9]+)\.$",
        )
        .unwrap();
        let matches = re.captures(s).unwrap();
        Disc {
            nb_positions: matches[1].parse().unwrap(),
            start_position: matches[2].parse().unwrap(),
        }
    }

    fn position_at_time(&self, time: u32) -> u32 {
        (self.start_position + time) % self.nb_positions
    }
}

fn parse(raw_input: &str) -> Vec<Disc> {
    let mut discs = vec![];
    for line in raw_input.lines() {
        discs.push(Disc::from_str(line.trim()));
    }
    discs
}

fn test_position(discs: &[Disc], start_time: u32) -> bool {
    for i in 1..discs.len() + 1 {
        if discs[i - 1].position_at_time(start_time + (i as u32)) != 0 {
            return false;
        }
    }
    true
}

fn solve_part1(input: &[Disc]) -> u32 {
    let mut time = 0;
    loop {
        if test_position(input, time) {
            return time;
        }
        time += 1;
    }
}

fn day15_part1(example: &[Disc], input: &[Disc]) {
    // Exemple tests
    let res = solve_part1(example);
    assert_eq!(res, 5);
    println!("Example OK");

    // Solve puzzle
    let res = solve_part1(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY15 - part 1: OK!");
}

fn day15_part2(_example: &[Disc], _input: &[Disc]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY15 - part 2: OK!");
}
