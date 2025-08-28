use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY03 -------");
    let raw_input = fs::read_to_string("inputs/input_day03").expect("Unable to read input!");
    let input_part1 = parse_part1(&raw_input);
    let raw_example = fs::read_to_string("inputs/example_day03").expect("Unable to read input!");
    let example_part2 = parse_part2(&raw_example);
    let input_part2 = parse_part2(&raw_input);

    day03_part1(&input_part1);
    day03_part2(&example_part2, &input_part2);
}

fn parse_part1(raw_input: &str) -> Vec<[u32; 3]> {
    let mut triangles: Vec<[u32; 3]> = vec![];
    let re = Regex::new(r"^ *([0-9]+) +([0-9]+) +([0-9]+)$").unwrap();
    for line in raw_input.lines() {
        let matches = re.captures(line).unwrap();
        let mut sides = [
            matches[1].parse().unwrap(),
            matches[2].parse().unwrap(),
            matches[3].parse().unwrap(),
        ];
        sides.sort();
        triangles.push(sides);
    }
    triangles
}

fn parse_part2(raw_input: &str) -> Vec<[u32; 3]> {
    let mut triangles: Vec<[u32; 3]> = vec![];
    let re = Regex::new(r"^ *([0-9]+) +([0-9]+) +([0-9]+)$").unwrap();
    let mut temp1 = vec![];
    let mut temp2 = vec![];
    let mut temp3 = vec![];
    for (i, line) in raw_input.lines().enumerate() {
        let matches = re.captures(line).unwrap();
        temp1.push(matches[1].parse().unwrap());
        temp2.push(matches[2].parse().unwrap());
        temp3.push(matches[3].parse().unwrap());
        if i % 3 == 2 {
            temp1.sort();
            temp2.sort();
            temp3.sort();

            triangles.push(temp1.clone().try_into().unwrap());
            triangles.push(temp2.clone().try_into().unwrap());
            triangles.push(temp3.clone().try_into().unwrap());

            temp1.clear();
            temp2.clear();
            temp3.clear();
        }
    }
    triangles
}

fn constructible(triangle: &[u32; 3]) -> bool {
    triangle[0] + triangle[1] > triangle[2]
}

fn day03_part1(input: &[[u32; 3]]) {
    // Exemple tests
    assert!(!constructible(&[5, 10, 25]));

    // Solve puzzle
    let res = input.iter().copied().filter(constructible).count();
    println!("Result part 1: {res}");
    assert_eq!(res, 1050);
    println!("> DAY03 - part 1: OK!");
}

fn day03_part2(example: &[[u32; 3]], input: &[[u32; 3]]) {
    // Exemple tests
    let res = example.iter().copied().filter(constructible).count();
    assert_eq!(res, 6);

    // Solve puzzle
    let res = input.iter().copied().filter(constructible).count();
    println!("Result part 2: {res}");
    assert_eq!(res, 1921);
    println!("> DAY03 - part 2: OK!");
}
