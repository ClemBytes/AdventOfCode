use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY02 -------");
    let example = fs::read_to_string("inputs/example_day02").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day02").expect("Unable to read input!");
    let input = parse(&input);

    day02_part1(&example, &input);
    day02_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<Vec<u32>> {
    let mut spreadsheet: Vec<Vec<u32>> = vec![];
    for line in raw_input.lines() {
        let l: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        spreadsheet.push(l);
    }
    spreadsheet
}

fn checksum(spreadsheet: &[Vec<u32>]) -> u32 {
    let mut s = 0;
    for line in spreadsheet {
        s += line.iter().max().unwrap() - line.iter().min().unwrap();
    }
    s
}

fn day02_part1(example: &[Vec<u32>], input: &[Vec<u32>]) {
    // Exemple tests
    assert_eq!(checksum(example), 18);
    println!("Example OK");

    // Solve puzzle
    let res = checksum(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 39126);
    println!("> DAY02 - part 1: OK!");
}

fn day02_part2(_example: &[Vec<u32>], _input: &[Vec<u32>]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY02 - part 2: OK!");
}
