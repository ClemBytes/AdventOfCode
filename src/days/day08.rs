use std::fs;

pub fn run() {
    println!("------- DAY08 -------");
    let example = fs::read_to_string("inputs/example_day08").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day08").expect("Unable to read input!");
    let input = parse(&input);

    day08_part1(&example, &input);
    day08_part2(&example, &input);
}

fn parse(raw_input: &String) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    for line in raw_input.lines() {
        strings.push(String::from(line));
    }
    strings
}

fn day08_part1(example: &Vec<String>, _input: &Vec<String>) {
    // Exemple tests
    println!("Example input: {example:#?}");
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 1: {}");
    // assert_eq!(, );
    // println!("> DAY08 - part 1: OK!");
}

fn day08_part2(_example: &Vec<String>, _input: &Vec<String>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY08 - part 2: OK!");
}
