use crate::day10::complete_knot_hash;
use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY14 -------");
    let example = String::from("flqrgnkx");
    let input = fs::read_to_string("inputs/input_day14").expect("Unable to read input!");

    day14_part1(example.clone(), input.clone());
    day14_part2(example, input);
}

fn translate_line(line: String) -> String {
    let mut res = String::new();
    for ch in line.chars() {
        res.push_str(&format!("{:04b}", ch.to_digit(16).unwrap()));
    }
    res
}

fn create_grid(input: String) -> Vec<String> {
    let mut grid = Vec::new();
    for i in 0..128 {
        let mut line = input.clone();
        line.push_str(&format!("-{i}"));
        let hash = complete_knot_hash(line);
        grid.push(translate_line(hash));
    }
    grid
}

fn nb_used_squares(input: String) -> usize {
    let grid = create_grid(input);
    let mut res = 0;
    for line in grid {
        res += line.chars().filter(|&c| c == '1').count();
    }
    res
}

fn day14_part1(example: String, input: String) {
    assert_eq!(format!("{:04b}", '0'.to_digit(16).unwrap()), "0000");
    assert_eq!(format!("{:04b}", '1'.to_digit(16).unwrap()), "0001");
    assert_eq!(format!("{:04b}", 'e'.to_digit(16).unwrap()), "1110");
    assert_eq!(format!("{:04b}", 'f'.to_digit(16).unwrap()), "1111");
    assert_eq!(
        translate_line("a0c2017".to_string()),
        "1010000011000010000000010111"
    );
    // Exemple tests
    assert_eq!(nb_used_squares(example), 8108);

    // Solve puzzle
    let res = nb_used_squares(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 8316);
    println!("> DAY14 - part 1: OK!");
}

fn day14_part2(_example: String, _input: String) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY14 - part 2: OK!");
}
