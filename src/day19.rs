use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY19 -------");
    let input = fs::read_to_string("inputs/input_day19").expect("Unable to read input!");
    let input = parse(&input);

    day19_part1(&input);
    day19_part2(&input);
}

fn parse(raw_input: &String) -> Vec<_> {
    let mut yyy: Vec<_> = vec![];
    for _line in raw_input.lines() {
        // TODO
        // yyy.push(line);
    }
    yyy
}

fn day19_part1(_input: &Vec<_>) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY19 - part 1: OK!");
}

fn day19_part2(_input: &Vec<_>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY19 - part 2: OK!");
}
