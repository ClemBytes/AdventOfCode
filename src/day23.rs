use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY23 -------");

    let example = vec![
        Instructions::Increment(Register::A),
        Instructions::JumpIfOne(Register::A, 2),
        Instructions::Triple(Register::A),
        Instructions::Increment(Register::A),
    ];

    let input = fs::read_to_string("inputs/input_day23").expect("Unable to read input!");
    let input = parse(&input);

    day23_part1(&example, &input);
    day23_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<Instructions> {
    let mut instructions: Vec<Instructions> = vec![];
    for line in raw_input.lines() {
        let (command, arguments) = line.split_once(" ").unwrap();
        match command {
            "hlf" => {
                let register = match arguments {
                    "a" => Register::A,
                    "b" => Register::B,
                    other => {
                        panic!("Unknown register: {other}");
                    }
                };
                instructions.push(Instructions::Half(register));
            }
            "tpl" => {
                let register = match arguments {
                    "a" => Register::A,
                    "b" => Register::B,
                    other => {
                        panic!("Unknown register: {other}");
                    }
                };
                instructions.push(Instructions::Triple(register));
            }
            "inc" => {
                let register = match arguments {
                    "a" => Register::A,
                    "b" => Register::B,
                    other => {
                        panic!("Unknown register: {other}");
                    }
                };
                instructions.push(Instructions::Increment(register));
            }
            "jmp" => {
                let offset: i32 = arguments.parse().unwrap();
                instructions.push(Instructions::Jump(offset));
            }
            "jie" => {
                let (register, offset) = arguments.split_once(", ").unwrap();
                let register = match register {
                    "a" => Register::A,
                    "b" => Register::B,
                    other => {
                        panic!("Unknown register: {other}");
                    }
                };
                let offset: i32 = offset.parse().unwrap();
                instructions.push(Instructions::JumpIfEven(register, offset));
            }
            "jio" => {
                let (register, offset) = arguments.split_once(", ").unwrap();
                let register = match register {
                    "a" => Register::A,
                    "b" => Register::B,
                    other => {
                        panic!("Unknown register: {other}");
                    }
                };
                let offset: i32 = offset.parse().unwrap();
                instructions.push(Instructions::JumpIfOne(register, offset));
            }
            other => {
                panic!("Unknwon command {other}");
            }
        }
    }
    instructions
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

#[derive(Debug)]
enum Instructions {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

fn program(instructions: &[Instructions]) -> (u32, u32) {
    let mut a = 0;
    let mut b = 0;
    let mut index: i32 = 0;
    while (index as usize) < instructions.len() && index >= 0 {
        match &instructions[index as usize] {
            Instructions::Half(register) => {
                match register {
                    Register::A => {
                        a /= 2;
                    }
                    Register::B => {
                        b /= 2;
                    }
                };
                index += 1;
            }
            Instructions::Triple(register) => {
                match register {
                    Register::A => {
                        a *= 3;
                    }
                    Register::B => {
                        b *= 3;
                    }
                };
                index += 1;
            }
            Instructions::Increment(register) => {
                match register {
                    Register::A => {
                        a += 1;
                    }
                    Register::B => {
                        b += 1;
                    }
                };
                index += 1;
            }
            Instructions::Jump(offset) => {
                index += offset;
            }
            Instructions::JumpIfEven(register, offset) => {
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
            Instructions::JumpIfOne(register, offset) => {
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

fn day23_part1(example: &[Instructions], input: &[Instructions]) {
    // Exemple tests
    let (a, b) = program(example);
    println!("Result example part 1: a = {a} | b = {b}");
    assert_eq!(a, 2);

    // Solve puzzle
    let (a, b) = program(input);
    println!("Result part 1: a = {a} | b = {b}");
    assert_eq!(b, 307);
    println!("> DAY23 - part 1: OK!");
}

fn day23_part2(_example: &[Instructions], _input: &[Instructions]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY23 - part 2: OK!");
}
