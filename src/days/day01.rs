use std::fs;

pub fn run() {
    println!("-------------------");
    println!("------ DAY01 ------");
    println!("-------------------");
    day01_part1();
    day01_part2();
}

fn day01_part1() {
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

    let input = fs::read_to_string("inputs/input_day01")
        .expect("Unable to read input!");
    assert_eq!(get_floor(&input), 138);
    println!("> DAY01 - part 1: OK!");
}

fn day01_part2() {
    println!("TODO day01 - part2");
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
}