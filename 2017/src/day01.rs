use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY01 -------");
    let input = fs::read_to_string("inputs/input_day01").expect("Unable to read input!");

    day01_part1(&input);
    day01_part2(&input);
}

fn solve_part1(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut s = 0;
    let nb_digits = digits.len();
    for (i, &d) in digits.iter().enumerate() {
        if d == digits[(i + 1) % nb_digits] {
            s += d;
        }
    }
    s
}

fn day01_part1(input: &str) {
    // Exemple tests
    assert_eq!(solve_part1("1122"), 3);
    assert_eq!(solve_part1("1111"), 4);
    assert_eq!(solve_part1("1234"), 0);
    assert_eq!(solve_part1("91212129"), 9);
    println!("Example OK");

    // Solve puzzle
    let res = solve_part1(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY01 - part 1: OK!");
}

fn day01_part2(_input: &str) {
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
