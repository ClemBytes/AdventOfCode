use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAYXX -------");
    let example = fs::read_to_string("inputs/example_dayXX").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_dayXX").expect("Unable to read input!");
    let input = parse(&input);

    dayXX_part1(&example, &input);
    dayXX_part2(&example, &input);
}

fn parse(raw_input: &String) -> Vec<_> {
    let mut yyy: Vec<_> = vec![];
    for _line in raw_input.lines() {
        // TODO
        // yyy.push(line);
    }
    yyy
}


fn dayXX_part1(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 1: {}");
    // assert_eq!(, );
    // println!("> DAYXX - part 1: OK!");
}

fn dayXX_part2(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAYXX - part 2: OK!");
}