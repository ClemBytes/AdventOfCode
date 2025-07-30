use std::fs;

pub fn run() {
    println!("---------------------");
    println!("------- DAY01 -------");
    println!("---------------------");
    let input = fs::read_to_string("inputs/input_day01")
        .expect("Unable to read input!");
    day01_part1(&input);
    day01_part2(&input);
}

fn day01_part1(input: &String) {
    // Exemple tests
    let test = String::from("(())");
    assert_eq!(get_floor(&test), 0);
    let test = String::from("()()");
    assert_eq!(get_floor(&test), 0);
    let test = String::from("(((");
    assert_eq!(get_floor(&test), 3);
    let test = String::from("(()(()(");
    assert_eq!(get_floor(&test), 3);
    let test = String::from("))(((((");
    assert_eq!(get_floor(&test), 3);
    let test = String::from("())");
    assert_eq!(get_floor(&test), -1);
    let test = String::from("))(");
    assert_eq!(get_floor(&test), -1);
    let test = String::from(")))");
    assert_eq!(get_floor(&test), -3);
    let test = String::from(")())())");
    assert_eq!(get_floor(&test), -3);

    assert_eq!(get_floor(&input), 138);
    println!("> DAY01 - part 1: OK!");
}

fn day01_part2(input: &String) {
    let test = String::from(")");
    assert_eq!(basement_when(&test), 1);
    let test = String::from("()())");
    assert_eq!(basement_when(&test), 5);

    assert_eq!(basement_when(&input), 1771);
    println!("> DAY01 - part 2: OK!");
}

fn get_floor(instructions: &String) -> i32 {
    let mut floor = 0;
    for parenthesis in instructions.chars() {
        match parenthesis {
            '(' => { floor += 1; },
            ')' => { floor -= 1; },
            _   => { println!("Unkwnow character {parenthesis}! Should be ( or )."); },
        }
    }
    floor
    // We can do easier by counting number of '(', number of ')' and substract…
}

fn basement_when(instructions: &String) -> u32 {
    let mut floor = 0;
    let mut position = 1;
    for parenthesis in instructions.chars() {
        match parenthesis {
            '(' => { floor += 1; },
            ')' => { floor -= 1; },
            _   => { println!("Unkwnow character {parenthesis}! Should be ( or )."); },
        }
        if floor == -1 {
            return position;
        }
        position += 1;
    }
    println!("Never found basement!");
    position
}