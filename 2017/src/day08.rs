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

fn parse_and_apply(raw_input: &str) -> Vec<i32> {
    let mut registers = HashMap::new();
    let r = Regex::new(r"^([a-z]+) (inc|dec) (-?[0-9]+) if ([a-z]+) (<|>|<=|>=|==|!=) (-?[0-9]+)$")
        .unwrap();
    let mut max_values = vec![];
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
        let (_, &max) = registers.iter().max_by_key(|&(_k, v)| *v).unwrap();
        max_values.push(max);
        // println!("{registers:#?}");
    }
    max_values
}

fn day08_part1(example: &[i32], input: &[i32]) {
    // Exemple tests
    let &res = example.last().unwrap();
    assert_eq!(res, 1);

    // Solve puzzle
    let &res = input.last().unwrap();
    println!("Result part 1: {res}");
    assert_eq!(res, 3089);
    println!("> DAY08 - part 1: OK!");
}

fn day08_part2(example: &[i32], input: &[i32]) {
    // Exemple tests
    assert_eq!(*example.iter().max().unwrap(), 10);

    // Solve puzzle
    let res = *input.iter().max().unwrap();
    println!("Result part 2: {res}");
    assert_eq!(res, 5391);
    println!("> DAY08 - part 2: OK!");
}
