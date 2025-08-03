use std::fs;

use json::JsonValue;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY12 -------");
    let input = json::parse(
        fs::read_to_string("inputs/input_day12")
            .expect("Unable to read input!")
            .as_str(),
    ).unwrap();

    day12_part1(&input);
    day12_part2(&input);
}

fn get_number(input: &JsonValue) -> i32 {
    
}

fn day12_part1(_input: &JsonValue) {
    // Exemple tests
    println!("Ex 1:{:#}\n", json::parse(r#"[1,2,3]"#).unwrap());
    println!("Ex 2:{:#}\n", json::parse(r#"{"a":2,"b":4}"#).unwrap());
    println!("Ex 3:{:#}\n", json::parse(r#"[[[3]]]"#).unwrap());
    println!("Ex 4:{:#}\n", json::parse(r#"{"a":{"b":4},"c":-1}"#).unwrap());
    println!("Ex 5:{:#}\n", json::parse(r#"{"a":[-1,1]}"#).unwrap());
    println!("Ex 6:{:#}\n", json::parse(r#"[-1,{"a":1}]"#).unwrap());
    println!("Ex 7:{:#}\n", json::parse(r#"[]"#).unwrap());
    println!("Ex 8:{:#}\n", json::parse(r#"{}"#).unwrap());
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 1: {}");
    // assert_eq!(, );
    // println!("> DAY12 - part 1: OK!");
}

fn day12_part2(_input: &JsonValue) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY12 - part 2: OK!");
}
