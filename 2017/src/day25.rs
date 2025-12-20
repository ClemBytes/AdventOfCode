use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY25 -------");
    let example = fs::read_to_string("inputs/example_day25").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day25").expect("Unable to read input!");
    let input = parse(&input);

    day25(example, input);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct StateDescription {
    current_is_0: (usize, Direction, char),
    current_is_1: (usize, Direction, char),
}

fn parse(raw_input: &str) -> (char, usize, HashMap<char, StateDescription>) {
    let mut states = HashMap::new();
    let mut start_state_name = 'A';
    let mut nb_steps = 0;
    let mut current_state_name = start_state_name;
    let mut current_state = StateDescription {
        current_is_0: (0, Direction::Left, 'A'),
        current_is_1: (0, Direction::Left, 'A'),
    };
    let mut current_0_or_1 = 0;
    let mut started = false;
    // Regex
    let begin = Regex::new(r"^Begin in state ([A-Z]).$").unwrap();
    let steps = Regex::new(r"^Perform a diagnostic checksum after ([0-9]*) steps.$").unwrap();
    let state_name = Regex::new(r"^In state ([A-Z]):$").unwrap();
    let current_value_is = Regex::new(r"If the current value is (0|1):$").unwrap();
    let value = Regex::new(r"Write the value (0|1).$").unwrap();
    let direction = Regex::new(r"Move one slot to the (left|right).$").unwrap();
    let next_state_name = Regex::new(r"Continue with state ([A-Z]).$").unwrap();
    for line in raw_input.lines() {
        if let Some(matches) = begin.captures(line) {
            start_state_name = matches[1].chars().collect::<Vec<char>>()[0];
        } else if let Some(matches) = steps.captures(line) {
            nb_steps = matches[1].parse().unwrap();
        } else if let Some(matches) = state_name.captures(line) {
            if !started {
                started = true;
                continue;
            }
            states.insert(current_state_name, current_state);
            current_state_name = matches[1].chars().collect::<Vec<char>>()[0];
        } else if let Some(matches) = current_value_is.captures(line) {
            current_0_or_1 = match &matches[1] {
                "0" => 0,
                "1" => 1,
                other => unreachable!("Cannot be '{other}', should be '0' or '1'"),
            }
        } else if let Some(matches) = value.captures(line) {
            let val = matches[1].parse().unwrap();
            match current_0_or_1 {
                0 => current_state.current_is_0.0 = val,
                1 => current_state.current_is_1.0 = val,
                other => unreachable!("current_0_or_1 cannot be '{other}', should be '0' or '1'"),
            }
        } else if let Some(matches) = direction.captures(line) {
            let dir = match &matches[1] {
                "left" => Direction::Left,
                "right" => Direction::Right,
                other => unreachable!("Cannot be '{other}', should be 'left' or 'right'"),
            };
            match current_0_or_1 {
                0 => current_state.current_is_0.1 = dir,
                1 => current_state.current_is_1.1 = dir,
                other => unreachable!("current_0_or_1 cannot be '{other}', should be '0' or '1'"),
            }
        } else if let Some(matches) = next_state_name.captures(line) {
            let nstate_name = matches[1].chars().collect::<Vec<char>>()[0];
            match current_0_or_1 {
                0 => current_state.current_is_0.2 = nstate_name,
                1 => current_state.current_is_1.2 = nstate_name,
                other => unreachable!("current_0_or_1 cannot be '{other}', should be '0' or '1'"),
            }
        }
    }
    states.insert(current_state_name, current_state);
    (start_state_name, nb_steps, states)
}

fn turing_machine_checksum(input: (char, usize, HashMap<char, StateDescription>)) -> usize {
    // Init
    let (mut current_state_name, nb_steps, states) = input;
    let mut turing_machine = HashMap::new();
    let mut current_position = 0;
    turing_machine.insert(current_position, 0);

    // Apply instructions
    for _ in 0..nb_steps {
        let current_state = *states.get(&current_state_name).unwrap();
        turing_machine.entry(current_position).or_insert(0);
        let current_value = *turing_machine.get(&current_position).unwrap();
        let (value_to_write, direction_to_take, next_state_name);
        match current_value {
            0 => {
                (value_to_write, direction_to_take, next_state_name) = current_state.current_is_0;
            }
            1 => {
                (value_to_write, direction_to_take, next_state_name) = current_state.current_is_1;
            }
            _ => unreachable!("current_value cannot be '{current_value}', should be '0' or '1'"),
        }
        turing_machine.insert(current_position, value_to_write);
        current_position += match direction_to_take {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        current_state_name = next_state_name;
    }

    // Compute Turing machine checksum
    let mut checksum = 0;
    for (_, val) in turing_machine {
        checksum += val;
    }
    checksum
}

fn day25(
    example: (char, usize, HashMap<char, StateDescription>),
    input: (char, usize, HashMap<char, StateDescription>),
) {
    // Exemple tests
    assert_eq!(turing_machine_checksum(example), 3);

    // Solve puzzle
    let res = turing_machine_checksum(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 2846);
    println!("> DAY25 - part 1: OK!");
}
