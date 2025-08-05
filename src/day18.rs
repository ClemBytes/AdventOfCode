use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY18 -------");
    let example = fs::read_to_string("inputs/example_day18").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day18").expect("Unable to read input!");
    let input = parse(&input);

    day18_part1(&example, &input);
    day18_part2(&example, &input);
}

const NB_TURNS_EXAMPLE: u32 = 4;
const NB_TURNS_INPUT: u32 = 100;

fn parse(raw_input: &String) -> Vec<_> {
    let mut yyy: Vec<_> = vec![];
    for _line in raw_input.lines() {
        // TODO
        // yyy.push(line);
    }
    yyy
}

fn day18_part1(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY18 - part 1: OK!");
}

fn day18_part2(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY18 - part 2: OK!");
}
