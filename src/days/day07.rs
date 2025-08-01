use std::{collections::HashMap, fs};

pub fn run() {
    println!("------- DAY07 -------");
    // let example_path = "inputs/example_day07";
    // let raw_example = fs::read_to_string(example_path).expect("Unable to read input!");
    // let input = parse(&raw_example);

    let input_path = "inputs/input_day07";
    let raw_input = fs::read_to_string(input_path).expect("Unable to read input!");
    let input = parse(&raw_input);

    day07_part1(input);
    day07_part2();
}

fn day07_part1(instructions: Vec<Instruction>) {
    // Example tests
    // let final_state = apply_instructions(instructions);
    // println!("Example final state: {:#?}", final_state);

    // Solve puzzle
    let final_state = apply_instructions(instructions);
    // println!("Result part 1: {}", final_state.get("a").unwrap());
    assert_eq!(*final_state.get("a").unwrap(), 46065);
    println!("> DAY07 - part 1: OK!");
}

fn day07_part2() {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY07 - part 2: OK!");
}

fn parse(raw_input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for line in raw_input.lines() {
        instructions.push(Instruction::read_instruction(line));
    }
    instructions
}

fn apply_instructions(mut instructions: Vec<Instruction>) -> HashMap<String, u16> {
    let mut map: HashMap<String, u16> = HashMap::new();
    while !instructions.is_empty() {
        let mut instructions_used: Vec<usize> = vec![];
        for (i, instruction) in instructions.iter().enumerate() {
            let mut used = false;

            let res = instruction.gate.get_result(&map);
            if res.is_some() {
                let sig = res.unwrap();
                map.insert(instruction.output.clone(), sig);
                used = true;
            }

            if used {
                instructions_used.push(i);
            }
        }
        // Delete used instructions
        for i in instructions_used.iter().rev() {
            instructions.remove(*i);
        }
    }

    map
}

enum Wire {
    Identifier(String),
    Signal(u16),
}

impl Wire {
    fn get_signal(&self, map: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Wire::Identifier(id) => {
                if map.contains_key(id) {
                    return Some(*map.get(id).unwrap());
                }
            }
            Wire::Signal(id) => {
                return Some(*id);
            }
        }
        None
    }
}

enum LogicGates {
    Assign(Wire),
    Not(String),
    And(Wire, Wire),
    Or(Wire, Wire),
    Lshift(String, u16),
    Rshift(String, u16),
}

impl LogicGates {
    fn get_result(&self, map: &HashMap<String, u16>) -> Option<u16> {
        match self {
            // ASSIGN
            LogicGates::Assign(input) => {
                return input.get_signal(map);
            }

            // NOT
            LogicGates::Not(input) => {
                if map.contains_key(input) {
                    return Some(!*map.get(input).unwrap());
                }
            }

            // AND
            LogicGates::And(a, b) => {
                let sig_a = a.get_signal(map);
                let sig_b = b.get_signal(map);
                if let (Some(sig_a_val), Some(sig_b_val)) = (sig_a, sig_b) {
                    return Some(sig_a_val & sig_b_val);
                }
            }

            // OR
            LogicGates::Or(a, b) => {
                let sig_a = a.get_signal(map);
                let sig_b = b.get_signal(map);
                if let (Some(sig_a_val), Some(sig_b_val)) = (sig_a, sig_b) {
                    return Some(sig_a_val | sig_b_val);
                }
            }

            // LSHIFT
            LogicGates::Lshift(a, dec) => {
                if map.contains_key(a) {
                    let sig_a = map.get(a).unwrap();
                    return Some(sig_a << dec);
                }
            }

            // RSHIFT
            LogicGates::Rshift(a, dec) => {
                if map.contains_key(a) {
                    let sig_a = map.get(a).unwrap();
                    return Some(sig_a >> dec);
                }
            }
        }
        None
    }
}

struct Instruction {
    gate: LogicGates,
    output: String,
}

impl Instruction {
    fn read_instruction(raw_instruction: &str) -> Self {
        let content: Vec<&str> = raw_instruction.split(" -> ").collect();
        let output = String::from(content[1]);
        let prefix = content[0];
        let prefix_words: Vec<&str> = prefix.split(' ').collect();
        let gate: LogicGates;
        if prefix.contains("NOT") {
            gate = LogicGates::Not(String::from(prefix_words[1]));
        } else if prefix.contains("AND") {
            let first = prefix_words[0];
            let first: Wire = if first.parse::<u16>().is_ok() {
                Wire::Signal(first.parse().unwrap())
            } else {
                Wire::Identifier(String::from(first))
            };

            let second = prefix_words[2];
            let second: Wire = if second.parse::<u16>().is_ok() {
                Wire::Signal(second.parse().unwrap())
            } else {
                Wire::Identifier(String::from(second))
            };

            gate = LogicGates::And(first, second);
        } else if prefix.contains("OR") {
            let first = prefix_words[0];
            let first: Wire = if first.parse::<u16>().is_ok() {
                Wire::Signal(first.parse().unwrap())
            } else {
                Wire::Identifier(String::from(first))
            };

            let second = prefix_words[2];
            let second: Wire = if second.parse::<u16>().is_ok() {
                Wire::Signal(second.parse().unwrap())
            } else {
                Wire::Identifier(String::from(second))
            };

            gate = LogicGates::Or(first, second);
        } else if prefix.contains("LSHIFT") {
            let first = String::from(prefix_words[0]);
            let second: u16 = prefix_words[2].parse().unwrap();
            gate = LogicGates::Lshift(first, second);
        } else if prefix.contains("RSHIFT") {
            let first = String::from(prefix_words[0]);
            let second: u16 = prefix_words[2].parse().unwrap();
            gate = LogicGates::Rshift(first, second);
        } else {
            let signal_or_id = prefix_words[0];
            let signal_or_id: Wire = if signal_or_id.parse::<u16>().is_ok() {
                Wire::Signal(signal_or_id.parse().unwrap())
            } else {
                Wire::Identifier(String::from(signal_or_id))
            };

            gate = LogicGates::Assign(signal_or_id);
        }

        Instruction { gate, output }
    }
}

/*
// Checks
println!("123 in bytes:          {:016b}", 123_u16);
println!("NOT 123 in bytes:      {:016b}", !123_u16);
println!("123 AND 456 in bytes:  {:016b}", 123_u16 & 456_u16);
println!(
    "72 in bytes:           {:016b} (should be equal to previous)",
    72_u16
);
println!("123 OR 456 in bytes:   {:016b}", 123_u16 | 456_u16);
println!(
    "507 in bytes:          {:016b} (should be equal to previous)",
    507_u16
);
println!("123 LSHIFT 2 in bytes: {:016b}", 123_u16 << 2);
println!(
    "492 in bytes:          {:016b} (should be equal to previous)",
    492_u16
);
println!("456 RSHIFT 2 in bytes: {:016b}", 456_u16 >> 2);
println!(
    "114 in bytes:          {:016b} (should be equal to previous)",
    114_u16
);
*/
