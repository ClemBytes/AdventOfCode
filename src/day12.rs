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
    )
    .unwrap();

    day12_part1(&input);
    day12_part2(&input);
}

fn get_number(input: &JsonValue) -> i64 {
    match input {
        JsonValue::Number(number) => number.as_fixed_point_i64(0).unwrap(),
        JsonValue::Array(array) => {
            let mut s: i64 = 0;
            for element in array {
                s += get_number(element);
            }
            s
        }
        JsonValue::Object(object) => {
            let mut s: i64 = 0;
            for element in object.iter() {
                s += get_number(element.1);
            }
            s
        }
        JsonValue::Short(_) => 0,
        JsonValue::String(_) => 0,
        JsonValue::Boolean(_) => 0,
        JsonValue::Null => 0,
    }
}

fn day12_part1(input: &JsonValue) {
    // Exemple tests
    let ex = json::parse(r#"[1,2,3]"#).unwrap();
    let res = get_number(&ex);
    println!("Ex 1: {ex} | sum = {res}");
    assert_eq!(res, 6);

    let ex = json::parse(r#"{"a":2,"b":4}"#).unwrap();
    let res = get_number(&ex);
    println!("Ex 2: {ex} | sum = {res}");
    assert_eq!(res, 6);

    let ex = json::parse(r#"[[[3]]]"#).unwrap();
    let res = get_number(&ex);
    println!("Ex 3: {ex} | sum = {res}");
    assert_eq!(res, 3);

    let ex = json::parse(r#"{"a":{"b":4},"c":-1}"#).unwrap();
    let res = get_number(&ex);
    println!("Ex 4: {ex} | sum = {res}");
    assert_eq!(res, 3);

    let ex = json::parse(r#"{"a":[-1,1]}"#).unwrap();
    let res = get_number(&ex);
    println!("Ex 5: {ex} | sum = {res}");
    assert_eq!(res, 0);

    let ex = json::parse(r#"[-1,{"a":1}]"#).unwrap();
    let res = get_number(&ex);
    println!("Ex 6: {ex} | sum = {res}");
    assert_eq!(res, 0);

    let ex = json::parse(r#"[]"#).unwrap();
    let res = get_number(&ex);
    println!("Ex 7: {ex} | sum = {res}");
    assert_eq!(res, 0);

    let ex = json::parse(r#"{}"#).unwrap();
    let res = get_number(&ex);
    println!("Ex 8: {ex} | sum = {res}");
    assert_eq!(res, 0);

    // Solve puzzle
    let res = get_number(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 119433);
    println!("> DAY12 - part 1: OK!\n");
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
