use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY01 -------");
    let input = fs::read_to_string("inputs/input_day01").expect("Unable to read input!");
    let input = parse(&input);

    day01_part1(&input);
    // day01_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<i32> {
    let mut frequency_changes: Vec<i32> = vec![];
    for line in raw_input.lines() {
        let freq: i32 = line.parse().unwrap();
        frequency_changes.push(freq);
    }
    frequency_changes
}

fn day01_part1(input: &Vec<i32>) {
    // Solve puzzle
    let res: i32 = input.into_iter().sum();
    println!("Result part 1: {res}");
    assert_eq!(res, 510);
    println!("> DAY01 - part 1: OK!");
}

// fn day01_part2(_example: &Vec<_>, _input: &Vec<_>) {
//     println!("TODO - part2");
//     // Exemple tests
//     // assert_eq!(, 0);
//     // println!("Example OK");
//
//     // Solve puzzle
//     // let res =
//     // println!("Result part 2: {res}");
//     // assert_eq!(res, );
//     // println!("> DAY01 - part 2: OK!");
// }
