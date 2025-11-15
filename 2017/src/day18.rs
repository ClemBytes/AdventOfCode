use std::{collections::{HashMap, VecDeque}, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY18 -------");
    let example1 = fs::read_to_string("inputs/example_day18-1").expect("Unable to read input!");
    let example1 = parse(&example1);
    let example2 = fs::read_to_string("inputs/example_day18-2").expect("Unable to read input!");
    let example2 = parse(&example2);
    let input = fs::read_to_string("inputs/input_day18").expect("Unable to read input!");
    let input = parse(&input);

    day18_part1(&example1, &input);
    day18_part2(&example2, &input);
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
    Jgz(RegisterOrValue, RegisterOrValue),
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
                let (check, val) = infos.split_once(" ").unwrap();
                let check_int = check.parse::<i64>();
                let check = match check_int {
                    Ok(i) => RegisterOrValue::Value(i),
                    Err(_) => RegisterOrValue::Register(check.parse::<char>().unwrap()),
                };
                let val_int = val.parse::<i64>();
                let val = match val_int {
                    Ok(i) => RegisterOrValue::Value(i),
                    Err(_) => RegisterOrValue::Register(val.parse::<char>().unwrap()),
                };
                Instruction::Jgz(check, val)
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
            Instruction::Jgz(check, reg_or_val) => {
                let check = match check {
                    RegisterOrValue::Register(reg) => *registers.get(&reg).unwrap(),
                    RegisterOrValue::Value(i) => i,
                };
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

    // Solve puzzle
    let res = first_recover(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 4601);
    println!("> DAY18 - part 1: OK!");
}

fn two_programs(input: &(Vec<Instruction>, HashMap<char, i64>)) -> i64 {
    let instructions = input.0.clone();
    let mut registers0 = input.1.clone();
    let mut registers1 = input.1.clone();
    registers1.insert('p', 1);
    let nb_instructions = instructions.len() as i64;
    let mut queue0 = VecDeque::new();
    let mut queue1 = VecDeque::new();
    let mut position0: i64 = 0;
    let mut position1: i64 = 0;
    let mut is_waiting0 = false;
    let mut is_waiting1 = false;
    let mut nb_send_values1 = 0;
    loop {
        if is_waiting0 && is_waiting1 && (queue0.len() == 0) && (queue1.len() == 0) {
            return nb_send_values1;
        }

        if !(is_waiting0 && queue0.len() == 0) {
            if (position0 < 0) || (position0 >= nb_instructions) {
                unreachable!("Out of instructions list!");
            }

            let ins = instructions[position0 as usize].clone();

            match ins {
                Instruction::Snd(reg) => {
                    let &freq = registers0.get(&reg).unwrap();
                    queue1.push_back(freq);
                    position0 += 1;
                }
                Instruction::Set(reg, reg_or_val) => {
                    let val = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers0.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    registers0.insert(reg, val);
                    position0 += 1;
                }
                Instruction::Add(reg, reg_or_val) => {
                    let mut val = *registers0.get(&reg).unwrap();
                    let term = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers0.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    val += term;
                    registers0.insert(reg, val);
                    position0 += 1;
                }
                Instruction::Mul(reg, reg_or_val) => {
                    let mut val = *registers0.get(&reg).unwrap();
                    let factor = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers0.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    val *= factor;
                    registers0.insert(reg, val);
                    position0 += 1;
                }
                Instruction::Mod(reg, reg_or_val) => {
                    let mut val = *registers0.get(&reg).unwrap();
                    let divider = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers0.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    val %= divider;
                    registers0.insert(reg, val);
                    position0 += 1;
                }
                Instruction::Rcv(reg) => {
                    if queue0.len() == 0 {
                        is_waiting0 = true;
                    } else {
                        let val = queue0.pop_front().unwrap();
                        registers0.insert(reg, val);
                        is_waiting0 = false;
                        position0 += 1;
                    }
                }
                Instruction::Jgz(check, reg_or_val) => {
                    let check = match check {
                        RegisterOrValue::Register(reg) => *registers0.get(&reg).unwrap(),
                        RegisterOrValue::Value(i) => i,
                    };
                    if check == 0 {
                        position0 += 1;
                    } else {
                        let val = match reg_or_val {
                            RegisterOrValue::Register(r) => *registers0.get(&r).unwrap(),
                            RegisterOrValue::Value(v) => v,
                        };
                        position0 += val;
                    }
                }
            }
        }

        if !(is_waiting1 && queue1.len() == 0) {
            if (position1 < 0) || (position1 >= nb_instructions) {
                unreachable!("Out of instructions list!");
            }

            let ins = instructions[position1 as usize].clone();

            match ins {
                Instruction::Snd(reg) => {
                    let &freq = registers1.get(&reg).unwrap();
                    queue0.push_back(freq);
                    nb_send_values1 += 1;
                    position1 += 1;
                }
                Instruction::Set(reg, reg_or_val) => {
                    let val = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers1.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    registers1.insert(reg, val);
                    position1 += 1;
                }
                Instruction::Add(reg, reg_or_val) => {
                    let mut val = *registers1.get(&reg).unwrap();
                    let term = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers1.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    val += term;
                    registers1.insert(reg, val);
                    position1 += 1;
                }
                Instruction::Mul(reg, reg_or_val) => {
                    let mut val = *registers1.get(&reg).unwrap();
                    let factor = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers1.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    val *= factor;
                    registers1.insert(reg, val);
                    position1 += 1;
                }
                Instruction::Mod(reg, reg_or_val) => {
                    let mut val = *registers1.get(&reg).unwrap();
                    let divider = match reg_or_val {
                        RegisterOrValue::Register(r) => *registers1.get(&r).unwrap(),
                        RegisterOrValue::Value(v) => v,
                    };
                    val %= divider;
                    registers1.insert(reg, val);
                    position1 += 1;
                }
                Instruction::Rcv(reg) => {
                    if queue1.len() == 0 {
                        is_waiting1 = true;
                    } else {
                        let val = queue1.pop_front().unwrap();
                        registers1.insert(reg, val);
                        is_waiting1 = false;
                        position1 += 1;
                    }
                }
                Instruction::Jgz(check, reg_or_val) => {
                    let check = match check {
                        RegisterOrValue::Register(reg) => *registers1.get(&reg).unwrap(),
                        RegisterOrValue::Value(i) => i,
                    };
                    if check == 0 {
                        position1 += 1;
                    } else {
                        let val = match reg_or_val {
                            RegisterOrValue::Register(r) => *registers1.get(&r).unwrap(),
                            RegisterOrValue::Value(v) => v,
                        };
                        position1 += val;
                    }
                }
            }
        }
    }
}

fn day18_part2(
    example: &(Vec<Instruction>, HashMap<char, i64>),
    input: &(Vec<Instruction>, HashMap<char, i64>),
) {
    // Exemple tests
    assert_eq!(two_programs(example), 3);
    println!("Example OK");

    // Solve puzzle
    let res = two_programs(input);
    println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY18 - part 2: OK!");
}
