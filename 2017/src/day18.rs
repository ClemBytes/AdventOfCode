use std::{
    collections::{HashMap, VecDeque},
    fs,
};

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

impl RegisterOrValue {
    fn get(&self, registers: &HashMap<char, i64>) -> i64 {
        let val = match self {
            RegisterOrValue::Register(r) => registers.get(r).unwrap(),
            RegisterOrValue::Value(v) => v,
        };
        *val
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Snd(RegisterOrValue),
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
                let reg_int = infos.parse::<i64>();
                let reg = match reg_int {
                    Ok(i) => RegisterOrValue::Value(i),
                    Err(_) => {
                        let reg = infos.parse::<char>().unwrap();
                        registers.entry(reg).or_insert(0);
                        RegisterOrValue::Register(reg)
                    }
                };
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
            Instruction::Snd(reg) => match reg {
                RegisterOrValue::Register(reg) => {
                    let &freq = registers.get(&reg).unwrap();
                    last_sound_played_frequency = Some(freq);
                    position += 1;
                }
                RegisterOrValue::Value(_) => unreachable!(),
            },
            Instruction::Set(reg, reg_or_val) => {
                let val = reg_or_val.get(&registers);
                registers.insert(reg, val);
                position += 1;
            }
            Instruction::Add(reg, reg_or_val) => {
                let mut val = *registers.get(&reg).unwrap();
                let term = reg_or_val.get(&registers);
                val += term;
                registers.insert(reg, val);
                position += 1;
            }
            Instruction::Mul(reg, reg_or_val) => {
                let mut val = *registers.get(&reg).unwrap();
                let factor = reg_or_val.get(&registers);
                val *= factor;
                registers.insert(reg, val);
                position += 1;
            }
            Instruction::Mod(reg, reg_or_val) => {
                let mut val = *registers.get(&reg).unwrap();
                let divider = reg_or_val.get(&registers);
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
                let check = check.get(&registers);
                if check <= 0 {
                    position += 1;
                } else {
                    let val = reg_or_val.get(&registers);
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

fn run_instruction(
    instruction: Instruction,
    position: i64,
    is_waiting: bool,
    queue: &mut VecDeque<i64>,
    other_queue: &mut VecDeque<i64>,
    registers: &mut HashMap<char, i64>,
) -> (i64, bool, bool) {
    let mut pos = position;
    let mut is_wait = is_waiting;
    let mut sent_something = false;
    match instruction {
        Instruction::Snd(reg_or_val) => {
            let freq = reg_or_val.get(registers);
            other_queue.push_back(freq);
            pos += 1;
            sent_something = true;
        }
        Instruction::Set(reg, reg_or_val) => {
            let val = reg_or_val.get(registers);
            registers.insert(reg, val);
            pos += 1;
        }
        Instruction::Add(reg, reg_or_val) => {
            let mut val = *registers.get(&reg).unwrap();
            let term = reg_or_val.get(registers);
            val += term;
            registers.insert(reg, val);
            pos += 1;
        }
        Instruction::Mul(reg, reg_or_val) => {
            let mut val = *registers.get(&reg).unwrap();
            let factor = reg_or_val.get(registers);
            val *= factor;
            registers.insert(reg, val);
            pos += 1;
        }
        Instruction::Mod(reg, reg_or_val) => {
            let mut val = *registers.get(&reg).unwrap();
            let divider = reg_or_val.get(registers);
            val %= divider;
            registers.insert(reg, val);
            pos += 1;
        }
        Instruction::Rcv(reg) => {
            if queue.is_empty() {
                is_wait = true;
            } else {
                let val = queue.pop_front().unwrap();
                registers.insert(reg, val);
                is_wait = false;
                pos += 1;
            }
        }
        Instruction::Jgz(check, reg_or_val) => {
            let check = check.get(registers);
            if check <= 0 {
                pos += 1;
            } else {
                let val = reg_or_val.get(registers);
                pos += val;
            }
        }
    }
    (pos, is_wait, sent_something)
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
        if is_waiting0 && is_waiting1 && (queue0.is_empty()) && (queue1.is_empty()) {
            return nb_send_values1;
        }

        if !(is_waiting0 && queue0.is_empty()) {
            if (position0 < 0) || (position0 >= nb_instructions) {
                unreachable!("Out of instructions list!");
            }

            let ins = instructions[position0 as usize].clone();

            (position0, is_waiting0, _) = run_instruction(
                ins,
                position0,
                is_waiting0,
                &mut queue0,
                &mut queue1,
                &mut registers0,
            );
        }

        if !(is_waiting1 && queue1.is_empty()) {
            if (position1 < 0) || (position1 >= nb_instructions) {
                unreachable!("Out of instructions list!");
            }

            let ins = instructions[position1 as usize].clone();

            let sent_something;
            (position1, is_waiting1, sent_something) = run_instruction(
                ins,
                position1,
                is_waiting1,
                &mut queue1,
                &mut queue0,
                &mut registers1,
            );

            if sent_something {
                nb_send_values1 += 1;
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

    // Solve puzzle
    let res = two_programs(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 6858);
    println!("> DAY18 - part 2: OK!");
}
