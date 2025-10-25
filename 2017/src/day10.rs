use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY10 -------");
    let example = vec![0, 1, 2, 3, 4];
    let input = fs::read_to_string("inputs/input_day10").expect("Unable to read input!");
    let input = parse(&input);

    day10_part1(&example, &input);
    day10_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<u32> {
    raw_input.trim().split(",").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<u32>().unwrap()).collect()
}

fn day10_part1(example: &Vec<u32>, input: &Vec<u32>) {
    println!("{example:?}");
    println!("{input:?}");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 1: OK!");
}

fn day10_part2(_example: &Vec<u32>, _input: &Vec<u32>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 2: OK!");
}
