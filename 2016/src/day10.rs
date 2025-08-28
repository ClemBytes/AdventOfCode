use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY10 -------");
    let example = fs::read_to_string("inputs/example_day10").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day10").expect("Unable to read input!");
    let input = parse(&input);

    day10_part1(&example, &input);
    day10_part2(&input);
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Ouput(u32),
    Bot(u32),
}

impl Destination {
    fn from_str(str_dest: &str, id: u32) -> Self {
        match str_dest {
            "output" => Destination::Ouput(id),
            "bot" => Destination::Bot(id),
            other => panic!("Unknown destination: {other}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    low_to: Destination,
    high_to: Destination,
}

fn parse(raw_input: &str) -> (HashMap<u32, Vec<u32>>, HashMap<u32, Instruction>) {
    let mut bots: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut instructions: HashMap<u32, Instruction> = HashMap::new();
    let re_affect = Regex::new(r"^value ([0-9]+) goes to bot ([0-9]+)$").unwrap();
    let re_instruction = Regex::new(
        r"^bot ([0-9]+) gives low to (output|bot) ([0-9]+) and high to (output|bot) ([0-9]+)$",
    )
    .unwrap();
    for line in raw_input.lines() {
        if let Some(matches) = re_affect.captures(line) {
            let value: u32 = matches[1].parse().unwrap();
            let bot: u32 = matches[2].parse().unwrap();
            let values = bots.entry(bot).or_default();
            values.push(value);
            values.sort();
        } else if let Some(matches) = re_instruction.captures(line) {
            let bot: u32 = matches[1].parse().unwrap();
            let low_id: u32 = matches[3].parse().unwrap();
            let low_to = Destination::from_str(&matches[2], low_id);
            let high_id: u32 = matches[5].parse().unwrap();
            let high_to = Destination::from_str(&matches[4], high_id);
            instructions.insert(bot, Instruction { low_to, high_to });
        } else {
            panic!("Unknown command: {line}");
        }
    }
    (bots, instructions)
}

fn solve_part1(
    input: (HashMap<u32, Vec<u32>>, HashMap<u32, Instruction>),
    mut values_to_compare: Vec<u32>,
) -> u32 {
    values_to_compare.sort();
    let (mut bots, instructions) = input;
    loop {
        // println!("instructions: {instructions:?}");
        let (&id, microchips) = bots
            .iter_mut()
            .find(|(_, microchips)| microchips.len() == 2)
            .unwrap();
        microchips.sort();
        if *microchips == values_to_compare {
            return id;
        }
        let low_value = microchips.remove(0);
        let high_value = microchips.remove(0);
        match instructions[&id].low_to {
            Destination::Ouput(_) => {}
            Destination::Bot(other_id) => {
                if let Some(next_bot) = bots.get_mut(&other_id) {
                    next_bot.push(low_value);
                } else {
                    bots.insert(other_id, vec![low_value]);
                }
            }
        }
        match instructions[&id].high_to {
            Destination::Ouput(_) => {}
            Destination::Bot(other_id) => {
                if let Some(next_bot) = bots.get_mut(&other_id) {
                    next_bot.push(high_value);
                } else {
                    bots.insert(other_id, vec![high_value]);
                }
            }
        }
    }
}

fn solve_part2(input: (HashMap<u32, Vec<u32>>, HashMap<u32, Instruction>)) -> u32 {
    let (mut bots, instructions) = input;
    let mut output0 = 0;
    let mut output1 = 0;
    let mut output2 = 0;
    loop {
        if output0 * output1 * output2 != 0 {
            return output0 * output1 * output2;
        }
        let (&id, microchips) = bots
            .iter_mut()
            .find(|(_, microchips)| microchips.len() == 2)
            .unwrap();
        microchips.sort();
        let low_value = microchips.remove(0);
        let high_value = microchips.remove(0);
        match instructions[&id].low_to {
            Destination::Ouput(other_id) => match other_id {
                0 => output0 = low_value,
                1 => output1 = low_value,
                2 => output2 = low_value,
                _ => {}
            },
            Destination::Bot(other_id) => {
                if let Some(next_bot) = bots.get_mut(&other_id) {
                    next_bot.push(low_value);
                } else {
                    bots.insert(other_id, vec![low_value]);
                }
            }
        }
        match instructions[&id].high_to {
            Destination::Ouput(other_id) => match other_id {
                0 => output0 = high_value,
                1 => output1 = high_value,
                2 => output2 = high_value,
                _ => {}
            },
            Destination::Bot(other_id) => {
                if let Some(next_bot) = bots.get_mut(&other_id) {
                    next_bot.push(high_value);
                } else {
                    bots.insert(other_id, vec![high_value]);
                }
            }
        }
    }
}

fn day10_part1(
    example: &(HashMap<u32, Vec<u32>>, HashMap<u32, Instruction>),
    input: &(HashMap<u32, Vec<u32>>, HashMap<u32, Instruction>),
) {
    // Exemple tests
    let res = solve_part1(example.clone(), vec![2, 5]);
    assert_eq!(res, 2);

    // Solve puzzle
    let res = solve_part1(input.clone(), vec![17, 61]);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 1: OK!");
}

fn day10_part2(input: &(HashMap<u32, Vec<u32>>, HashMap<u32, Instruction>)) {
    // Solve puzzle
    let res = solve_part2(input.clone());
    println!("Result part 2: {res}");
    assert_eq!(res, 12803);
    println!("> DAY10 - part 2: OK!");
}
