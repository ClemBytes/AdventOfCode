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

fn solve(input: &str, part2: bool) -> u32 {
    let digits: Vec<u32> = input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut s = 0;
    let nb_digits = digits.len();
    let next = match part2 {
        true => nb_digits / 2,
        false => 1,
    };
    for (i, &d) in digits.iter().enumerate() {
        if d == digits[(i + next) % nb_digits] {
            s += d;
        }
    }
    s
}

fn day01_part1(input: &str) {
    // Exemple tests
    assert_eq!(solve("1122", false), 3);
    assert_eq!(solve("1111", false), 4);
    assert_eq!(solve("1234", false), 0);
    assert_eq!(solve("91212129", false), 9);
    println!("Example OK");

    // Solve puzzle
    let res = solve(input, false);
    println!("Result part 1: {res}");
    assert_eq!(res, 1089);
    println!("> DAY01 - part 1: OK!");
}

fn day01_part2(input: &str) {
    // Exemple tests
    assert_eq!(solve("1212", true), 6);
    assert_eq!(solve("1221", true), 0);
    assert_eq!(solve("123425", true), 4);
    assert_eq!(solve("123123", true), 12);
    assert_eq!(solve("12131415", true), 4);
    println!("Example OK");

    // Solve puzzle
    let res = solve(input, true);
    println!("Result part 2: {res}");
    assert_eq!(res, 1156);
    println!("> DAY01 - part 2: OK!");
}
