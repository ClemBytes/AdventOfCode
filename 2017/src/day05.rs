use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY05 -------");
    let example = fs::read_to_string("inputs/example_day05").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day05").expect("Unable to read input!");
    let input = parse(&input);

    day05_part1(&example, &input);
    day05_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<i32> {
    let mut instructions: Vec<i32> = vec![];
    for line in raw_input.lines() {
        instructions.push(line.parse::<i32>().unwrap());
    }
    instructions
}

fn nb_steps_to_escape(instructions: &[i32]) -> u32 {
    let mut instructions = instructions.to_owned();
    let mut nb_steps = 0;
    let mut position = 0i32;
    let nb_instructions = instructions.len() as i32;
    while position >= 0 && position < nb_instructions {
        let next_position = position + instructions[position as usize];
        instructions[position as usize] += 1;
        position = next_position;
        nb_steps += 1;
    }
    nb_steps
}

fn day05_part1(example: &[i32], input: &[i32]) {
    // Exemple tests
    assert_eq!(nb_steps_to_escape(example), 5);

    // Solve puzzle
    let res = nb_steps_to_escape(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 372671);
    println!("> DAY05 - part 1: OK!");
}

fn day05_part2(_example: &[i32], _input: &[i32]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY05 - part 2: OK!");
}
