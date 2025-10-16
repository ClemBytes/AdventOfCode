use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY06 -------");
    let input = fs::read_to_string("inputs/input_day06").expect("Unable to read input!");
    let input: Vec<&str> = input.split_whitespace().collect();
    let input: Vec<u32> = input.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    day06_part1(&input);
    day06_part2(&input);
}

fn day06_part1(_input: &Vec<u32>) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY06 - part 1: OK!");
}

fn day06_part2(_input: &Vec<u32>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY06 - part 2: OK!");
}
