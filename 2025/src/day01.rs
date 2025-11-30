use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY01 -------");
    let example = fs::read_to_string("inputs/example_day01").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day01").expect("Unable to read input!");
    let input = parse(&input);

    day01_part1(&example, &input);
    day01_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<_> {
    let mut yyy: Vec<_> = vec![];
    for _line in raw_input.lines() {
        // TODO
        // yyy.push(line);
    }
    yyy
}

fn day01_part1(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY01 - part 1: OK!");
}

fn day01_part2(_example: &Vec<_>, _input: &Vec<_>) {
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
