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

    fn from(s: &str) -> Self {
        if s.parse::<u16>().is_ok() {
            Wire::Signal(s.parse().unwrap())
        } else {
            Wire::Identifier(String::from(s))
        }
    }
}

enum LogicGates {
    Assign(Wire),
    Not(Wire),
    And(Wire, Wire),
    Or(Wire, Wire),
    Lshift(Wire, Wire),
    Rshift(Wire, Wire),
}

impl LogicGates {
    fn get_result(&self, map: &HashMap<String, u16>) -> Option<u16> {
        match self {
            LogicGates::Assign(input) => input.get_signal(map),
            LogicGates::Not(input) => {
                let input = input.get_signal(map)?;
                Some(!input)
            }
            LogicGates::And(a, b) => {
                let a = a.get_signal(map)?;
                let b = b.get_signal(map)?;
                Some(a & b)
            }
            LogicGates::Or(a, b) => {
                let a = a.get_signal(map)?;
                let b = b.get_signal(map)?;
                Some(a | b)
            }
            LogicGates::Lshift(a, shift) => {
                let a = a.get_signal(map)?;
                let shift = shift.get_signal(map)?;
                Some(a << shift)
            }
            LogicGates::Rshift(a, shift) => {
                let a = a.get_signal(map)?;
                let shift = shift.get_signal(map)?;
                Some(a >> shift)
            }
        }
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
        let gate = if prefix.contains("NOT") {
            LogicGates::Not(Wire::from(prefix_words[1]))
        } else if prefix.contains("AND") {
            let first = Wire::from(prefix_words[0]);
            let second = Wire::from(prefix_words[2]);
            LogicGates::And(first, second)
        } else if prefix.contains("OR") {
            let first = Wire::from(prefix_words[0]);
            let second = Wire::from(prefix_words[2]);
            LogicGates::Or(first, second)
        } else if prefix.contains("LSHIFT") {
            let first = Wire::from(prefix_words[0]);
            let second = Wire::from(prefix_words[2]);
            LogicGates::Lshift(first, second)
        } else if prefix.contains("RSHIFT") {
            let first = Wire::from(prefix_words[0]);
            let second = Wire::from(prefix_words[2]);
            LogicGates::Rshift(first, second)
        } else {
            LogicGates::Assign(Wire::from(prefix_words[0]))
        };

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
