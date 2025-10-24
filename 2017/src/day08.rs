use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY08 -------");
    let example = fs::read_to_string("inputs/example_day08").expect("Unable to read input!");
    let example = parse_and_apply(&example);
    let input = fs::read_to_string("inputs/input_day08").expect("Unable to read input!");
    let input = parse_and_apply(&input);

    day08_part1(&example, &input);
    day08_part2(&example, &input);
}

fn parse_and_apply(raw_input: &str) -> HashMap<String, i32> {
    let mut registers = HashMap::new();
    let r = Regex::new(r"^([a-z]+) (inc|dec) (-?[0-9]+) if ([a-z]+) (<|>|<=|>=|==|!=) (-?[0-9]+)$")
        .unwrap();
    for line in raw_input.lines() {
        if let Some(matches) = r.captures(line) {
            let r1 = matches[1].to_string();
            let operation = &matches[2];
            let amount: i32 = matches[3].parse().unwrap();
            let r2 = matches[4].to_string();
            let comp_operator = &matches[5];
            let comp_value: i32 = matches[6].parse().unwrap();

            let r2value = registers.entry(r2.clone()).or_insert(0);
            let comp_result = match comp_operator {
                "<" => *r2value < comp_value,
                ">" => *r2value > comp_value,
                "<=" => *r2value <= comp_value,
                ">=" => *r2value >= comp_value,
                "==" => *r2value == comp_value,
                "!=" => *r2value != comp_value,
                c => {
                    unreachable!("Not a comparison operator: {c} (should be '<' or '>')")
                }
            };
            // println!("{line}");
            // println!("{comp_result} | r2value ({}) = {} | comp_value = {comp_value}", r2.clone(), *r2value);

            if comp_result {
                let r1value = registers.entry(r1).or_insert(0);
                *r1value += match operation {
                    "inc" => amount,
                    "dec" => -amount,
                    c => {
                        unreachable!("Not an operations: {c} (should be 'inc' or 'dec')")
                    }
                }
            }
        } else {
            unreachable!("Bad instruction: {line}");
        }
        // println!("{registers:#?}");
    }
    registers
}

fn day08_part1(example: &HashMap<String, i32>, input: &HashMap<String, i32>) {
    // Exemple tests
    let (_, &max) = example.iter().max_by_key(|&(_k, v)| *v).unwrap();
    assert_eq!(max, 1);

    // Solve puzzle
    let (_, &res) = input.iter().max_by_key(|&(_k, v)| *v).unwrap();
    println!("Result part 1: {res}");
    assert_eq!(res, 3089);
    println!("> DAY08 - part 1: OK!");
}

fn day08_part2(_example: &HashMap<String, i32>, _input: &HashMap<String, i32>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY08 - part 2: OK!");
}
