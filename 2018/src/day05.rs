use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY05 -------");
    let example = "dabAcCaCBAcCcaDA".to_string();
    let input = fs::read_to_string("inputs/input_day05")
        .expect("Unable to read input!")
        .trim()
        .to_string();

    day05_part1(&example, &input);
    day05_part2(&example, &input);
}

fn full_reaction(polymer: &str) -> usize {
    let mut stack: Vec<char> = vec![];
    let polymer_vec: Vec<char> = polymer.chars().collect::<Vec<char>>();
    for unit in polymer_vec {
        let size_stack = stack.len();
        if size_stack > 0 {
            let last_char = stack[size_stack - 1];
            if last_char.eq_ignore_ascii_case(&unit) && last_char != unit {
                stack.pop();
                continue;
            }
        }
        stack.push(unit);
    }
    stack.len()
}

fn day05_part1(example: &str, input: &str) {
    // Exemple tests
    assert_eq!(full_reaction(example), 10);
    println!("Example OK");

    // Solve puzzle
    let res = full_reaction(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 10766);
    println!("> DAY05 - part 1: OK!");
}

fn day05_part2(_example: &str, _input: &str) {
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
