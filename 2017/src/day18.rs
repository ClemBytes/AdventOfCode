use std::{collections::HashMap, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY18 -------");
    let example = fs::read_to_string("inputs/example_day18").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day18").expect("Unable to read input!");
    let input = parse(&input);

    day18_part1(&example, &input);
    day18_part2(&example, &input);
}

#[derive(Debug, Clone)]
enum RegisterOrValue {
    Register(char),
    Value(i64),
}

#[derive(Debug, Clone)]
enum Instruction {
    Snd(char),
    Set(char, RegisterOrValue),
    Add(char, RegisterOrValue),
    Mul(char, RegisterOrValue),
    Mod(char, RegisterOrValue),
    Rcv(char),
    Jgz(char, RegisterOrValue),
}

fn parse_two_infos(infos: &str) -> (char, RegisterOrValue) {
    let (reg, val) = infos.split_once(" ").unwrap();
    let reg = reg.parse::<char>().unwrap();
    let val_int = val.parse::<i64>();
    let val = match val_int {
        Ok(i) => RegisterOrValue::Value(i),
        Err(_) => RegisterOrValue::Register(val.parse::<char>().unwrap()),
    };
    (reg, val)
}

fn parse(raw_input: &str) -> (Vec<Instruction>, HashMap<char, i64>) {
    let mut instructions: Vec<Instruction> = vec![];
    let mut registers = HashMap::new();
    for line in raw_input.lines() {
        let (kind, infos) = line.split_once(" ").unwrap();
        let ins = match kind {
            "snd" => {
                let reg = infos.parse::<char>().unwrap();
                registers.entry(reg).or_insert(0);
                Instruction::Snd(reg)
            }
            "set" => {
                let (reg, val) = parse_two_infos(infos);
                match val {
                    RegisterOrValue::Register(c) => {
                        registers.entry(c).or_insert(0);
                    }
                    RegisterOrValue::Value(_) => (),
                }
                Instruction::Set(reg, val)
            }
            "add" => {
                let (reg, val) = parse_two_infos(infos);
                match val {
                    RegisterOrValue::Register(c) => {
                        registers.entry(c).or_insert(0);
                    }
                    RegisterOrValue::Value(_) => (),
                }
                Instruction::Add(reg, val)
            }
            "mul" => {
                let (reg, val) = parse_two_infos(infos);
                match val {
                    RegisterOrValue::Register(c) => {
                        registers.entry(c).or_insert(0);
                    }
                    RegisterOrValue::Value(_) => (),
                }
                Instruction::Mul(reg, val)
            }
            "mod" => {
                let (reg, val) = parse_two_infos(infos);
                match val {
                    RegisterOrValue::Register(c) => {
                        registers.entry(c).or_insert(0);
                    }
                    RegisterOrValue::Value(_) => (),
                }
                Instruction::Mod(reg, val)
            }
            "rcv" => {
                let reg = infos.parse::<char>().unwrap();
                registers.entry(reg).or_insert(0);
                Instruction::Rcv(reg)
            }
            "jgz" => {
                let (reg, val) = parse_two_infos(infos);
                match val {
                    RegisterOrValue::Register(c) => {
                        registers.entry(c).or_insert(0);
                    }
                    RegisterOrValue::Value(_) => (),
                }
                Instruction::Jgz(reg, val)
            }
            other => unreachable!("Unknown instruction: {other}"),
        };
        instructions.push(ins);
    }
    (instructions, registers)
}

fn first_recover(input: &(Vec<Instruction>, HashMap<char, i64>)) -> i64 {
    let instructions = input.0.clone();
    let mut registers = input.1.clone();
    let nb_instructions = instructions.len() as i64;
    let mut last_sound_played_frequency: Option<i64> = None;
    let mut position: i64 = 0;
    loop {
        if (position < 0) || (position >= nb_instructions) {
            unreachable!("Out of instructions list!");
        }

        let ins = instructions[position as usize].clone();

        match ins {
            Instruction::Snd(reg) => {
                let &freq = registers.get(&reg).unwrap();
                last_sound_played_frequency = Some(freq);
                position += 1;
            }
            Instruction::Set(reg, reg_or_val) => {
                let val = match reg_or_val {
                    RegisterOrValue::Register(r) => *registers.get(&r).unwrap(),
                    RegisterOrValue::Value(v) => v,
                };
                registers.insert(reg, val);
                position += 1;
            }
            Instruction::Add(reg, reg_or_val) => {
                let mut val = *registers.get(&reg).unwrap();
                let term = match reg_or_val {
                    RegisterOrValue::Register(r) => *registers.get(&r).unwrap(),
                    RegisterOrValue::Value(v) => v,
                };
                val += term;
                registers.insert(reg, val);
                position += 1;
            }
            Instruction::Mul(reg, reg_or_val) => {
                let mut val = *registers.get(&reg).unwrap();
                let factor = match reg_or_val {
                    RegisterOrValue::Register(r) => *registers.get(&r).unwrap(),
                    RegisterOrValue::Value(v) => v,
                };
                val *= factor;
                registers.insert(reg, val);
                position += 1;
            }
            Instruction::Mod(reg, reg_or_val) => {
                let mut val = *registers.get(&reg).unwrap();
                let divider = match reg_or_val {
                    RegisterOrValue::Register(r) => *registers.get(&r).unwrap(),
                    RegisterOrValue::Value(v) => v,
                };
                val %= divider;
                registers.insert(reg, val);
                position += 1;
            }
            Instruction::Rcv(reg) => {
                let &val = registers.get(&reg).unwrap();
                if val != 0 {
                    return last_sound_played_frequency.unwrap();
                }
                position += 1;
            }
            Instruction::Jgz(reg, reg_or_val) => {
                let check = *registers.get(&reg).unwrap();
                if check == 0 {
                    position += 1;
                } else {
                    let val = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    position += val;
                }
            }
        }
    }
}

fn day18_part1(
    example: &(Vec<Instruction>, HashMap<char, i64>),
    input: &(Vec<Instruction>, HashMap<char, i64>),
) {
    // Exemple tests
    assert_eq!(first_recover(example), 4);
    println!("Example OK");

    // Solve puzzle
    let res = first_recover(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY18 - part 1: OK!");
}

fn day18_part2(
    _example: &(Vec<Instruction>, HashMap<char, i64>),
    _input: &(Vec<Instruction>, HashMap<char, i64>),
) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY18 - part 2: OK!");
}
