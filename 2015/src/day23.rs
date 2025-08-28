use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY23 -------");

    let example = vec![
        Instruction::Increment(Register::A),
        Instruction::JumpIfOne(Register::A, 2),
        Instruction::Triple(Register::A),
        Instruction::Increment(Register::A),
    ];

    let input = fs::read_to_string("inputs/input_day23").expect("Unable to read input!");
    let input = parse(&input);

    day23_part1(&example, &input);
    day23_part2(&input);
}

fn parse_argument(name: &str) -> Register {
    match name {
        "a" => Register::A,
        "b" => Register::B,
        other => panic!("Unknown register: {other}"),
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let (command, arguments) = line.split_once(" ").unwrap();
    match command {
        "hlf" => {
            let register = parse_argument(arguments);
            Instruction::Half(register)
        }
        "tpl" => {
            let register = parse_argument(arguments);
            Instruction::Triple(register)
        }
        "inc" => {
            let register = parse_argument(arguments);
            Instruction::Increment(register)
        }
        "jmp" => {
            let offset: i32 = arguments.parse().unwrap();
            Instruction::Jump(offset)
        }
        "jie" => {
            let (register, offset) = arguments.split_once(", ").unwrap();
            let register = parse_argument(register);
            let offset: i32 = offset.parse().unwrap();
            Instruction::JumpIfEven(register, offset)
        }
        "jio" => {
            let (register, offset) = arguments.split_once(", ").unwrap();
            let register = parse_argument(register);
            let offset: i32 = offset.parse().unwrap();
            Instruction::JumpIfOne(register, offset)
        }
        other => panic!("Unknwon command {other}"),
    }
}

fn parse(raw_input: &str) -> Vec<Instruction> {
    raw_input.lines().map(parse_instruction).collect()
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

fn program(instructions: &[Instruction], start_a: u32, start_b: u32) -> (u32, u32) {
    let mut a = start_a;
    let mut b = start_b;
    let mut index: i32 = 0;
    while (index as usize) < instructions.len() && index >= 0 {
        match &instructions[index as usize] {
            Instruction::Half(register) => {
                match register {
                    Register::A => a /= 2,
                    Register::B => b /= 2,
                }
                index += 1;
            }
            Instruction::Triple(register) => {
                match register {
                    Register::A => a *= 3,
                    Register::B => b *= 3,
                }
                index += 1;
            }
            Instruction::Increment(register) => {
                match register {
                    Register::A => a += 1,
                    Register::B => b += 1,
                }
                index += 1;
            }
            Instruction::Jump(offset) => {
                index += offset;
            }
            Instruction::JumpIfEven(register, offset) => {
                let considered_register_value = match register {
                    Register::A => a,
                    Register::B => b,
                };
                if considered_register_value % 2 == 0 {
                    index += offset;
                } else {
                    index += 1;
                }
            }
            Instruction::JumpIfOne(register, offset) => {
                let considered_register_value = match register {
                    Register::A => a,
                    Register::B => b,
                };
                if considered_register_value == 1 {
                    index += offset;
                } else {
                    index += 1;
                }
            }
        }
    }
    (a, b)
}

fn day23_part1(example: &[Instruction], input: &[Instruction]) {
    // Exemple tests
    let (a, b) = program(example, 0, 0);
    println!("Result example part 1: a = {a} | b = {b}");
    assert_eq!(a, 2);

    // Solve puzzle
    let (a, b) = program(input, 0, 0);
    println!("Result part 1: a = {a} | b = {b}");
    assert_eq!(b, 307);
    println!("> DAY23 - part 1: OK!");
}

fn day23_part2(input: &[Instruction]) {
    // Solve puzzle
    let (a, b) = program(input, 1, 0);
    println!("Result part 1: a = {a} | b = {b}");
    assert_eq!(b, 160);
    println!("> DAY23 - part 2: OK!");
}
