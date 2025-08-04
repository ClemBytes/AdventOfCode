use std::fs;

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

fn parse(raw_input: &String) -> Vec<_> {
    let mut yyy: Vec<_> = vec![];
    for _line in raw_input.lines() {
        // TODO
        // yyy.push(line);
    }
    yyy
}

fn day15_part1(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 1: {}");
    // assert_eq!(, );
    // println!("> DAY15 - part 1: OK!");
}

fn day15_part2(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY15 - part 2: OK!");
}
