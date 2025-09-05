use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY23 -------");
    let example = fs::read_to_string("inputs/example_day23").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day23").expect("Unable to read input!");
    let input = parse(&input);

    day23_part1(&example, &input);
    day23_part2(&input);
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "a" => Some(Register::A),
            "b" => Some(Register::B),
            "c" => Some(Register::C),
            "d" => Some(Register::D),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpyr(Register, Register),
    Cpyi(i32, Register),
    Inc(Register),
    Dec(Register),
    Jnzri(Register, i32),
    Jnzir(i32, Register),
    Tglr(Register),
}

fn parse(raw_input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in raw_input.lines() {
        let (instruction_kind, infos) = line.split_once(" ").unwrap();
        match instruction_kind {
            "cpy" => {
                let (x, y) = infos.split_once(" ").unwrap();
                let y = Register::from_str(y).unwrap();
                if let Some(x) = Register::from_str(x) {
                    instructions.push(Instruction::Cpyr(x, y));
                } else {
                    let x = x.parse().unwrap();
                    instructions.push(Instruction::Cpyi(x, y));
                }
            }
            "inc" => {
                let x = Register::from_str(infos).unwrap();
                instructions.push(Instruction::Inc(x));
            }
            "dec" => {
                let x = Register::from_str(infos).unwrap();
                instructions.push(Instruction::Dec(x));
            }
            "jnz" => {
                let (x, y) = infos.split_once(" ").unwrap();
                if let Some(x) = Register::from_str(x) {
                    let y = y.parse().unwrap();
                    instructions.push(Instruction::Jnzri(x, y));
                } else {
                    let x = x.parse().unwrap();
                    let y = Register::from_str(y).unwrap();
                    instructions.push(Instruction::Jnzir(x, y));
                }
            }
            "tgl" => {
                let x = Register::from_str(infos).unwrap();
                instructions.push(Instruction::Tglr(x));
            }
            _ => panic!("Unknown instruction: {instruction_kind} ({infos})!"),
        }
    }
    instructions
}

fn execute_code(instructions: &[Instruction], init_a: i32) -> i32 {
    let mut instructions: Vec<Instruction> = instructions.to_vec();
    // Initialize
    let mut a = init_a;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut i = 0;
    let nb_instructions = instructions.len();

    while i < nb_instructions {
        // println!("i: {i} | a: {a} | b: {b} | c: {c} | d: {d}");
        match instructions[i] {
            Instruction::Cpyr(rx, ry) => {
                let val_to_copy = match rx {
                    Register::A => a,
                    Register::B => b,
                    Register::C => c,
                    Register::D => d,
                };
                match ry {
                    Register::A => {
                        a = val_to_copy;
                    }
                    Register::B => {
                        b = val_to_copy;
                    }
                    Register::C => {
                        c = val_to_copy;
                    }
                    Register::D => {
                        d = val_to_copy;
                    }
                };
                i += 1;
            }
            Instruction::Cpyi(val, r) => {
                match r {
                    Register::A => {
                        a = val;
                    }
                    Register::B => {
                        b = val;
                    }
                    Register::C => {
                        c = val;
                    }
                    Register::D => {
                        d = val;
                    }
                };
                i += 1;
            }
            Instruction::Inc(r) => {
                match r {
                    Register::A => {
                        a += 1;
                    }
                    Register::B => {
                        b += 1;
                    }
                    Register::C => {
                        c += 1;
                    }
                    Register::D => {
                        d += 1;
                    }
                };
                i += 1;
            }
            Instruction::Dec(r) => {
                match r {
                    Register::A => {
                        a -= 1;
                    }
                    Register::B => {
                        b -= 1;
                    }
                    Register::C => {
                        c -= 1;
                    }
                    Register::D => {
                        d -= 1;
                    }
                };
                i += 1;
            }
            Instruction::Jnzri(r_test, skip) => {
                let val_test = match r_test {
                    Register::A => a,
                    Register::B => b,
                    Register::C => c,
                    Register::D => d,
                };
                if val_test != 0 {
                    let mut new_index = i as i32;
                    new_index += skip;
                    if new_index >= 0 {
                        i = new_index as usize;
                    }
                } else {
                    i += 1;
                }
            }
            Instruction::Jnzir(val_test, skip) => {
                let skip = match skip {
                    Register::A => a,
                    Register::B => b,
                    Register::C => c,
                    Register::D => d,
                };
                if val_test != 0 {
                    let mut new_index = i as i32;
                    new_index += skip;
                    if new_index >= 0 {
                        i = new_index as usize;
                    }
                } else {
                    i += 1;
                }
            }
            Instruction::Tglr(skip) => {
                let skip = match skip {
                    Register::A => a,
                    Register::B => b,
                    Register::C => c,
                    Register::D => d,
                };
                let index_to_toggle = ((i as i32) + skip) as usize;
                if index_to_toggle > instructions.len() - 1 {
                    i += 1;
                    continue;
                }
                instructions[index_to_toggle] = match instructions[index_to_toggle] {
                    // One argument instructions:
                    Instruction::Inc(r) => Instruction::Dec(r),
                    Instruction::Dec(r) => Instruction::Inc(r),
                    Instruction::Tglr(r) => Instruction::Inc(r),
                    // Two arguments instructions:
                    Instruction::Jnzri(_, _) => panic!("NOOOO"),
                    Instruction::Jnzir(val, r) => Instruction::Cpyi(val, r),
                    Instruction::Cpyi(val, r) => Instruction::Jnzir(val, r),
                    Instruction::Cpyr(r1, r2) => {
                        let val = match r2 {
                            Register::A => a,
                            Register::B => b,
                            Register::C => c,
                            Register::D => d,
                        };
                        Instruction::Jnzri(r1, val)
                    }
                };
                i += 1;
            }
        }
    }
    a
}

// fn toggle_instruction

fn day23_part1(example: &[Instruction], input: &[Instruction]) {
    // Exemple tests
    let res = execute_code(example, 7);
    assert_eq!(res, 3);

    // Solve puzzle
    let res = execute_code(input, 7);
    println!("Result part 1: {res}");
    assert_eq!(res, 11340);
    println!("> DAY23 - part 1: OK!");
}

fn day23_part2(input: &[Instruction]) {
    // Solve puzzle
    let res = execute_code(input, 12);
    println!("Result part 2: {res}");
    assert_eq!(res, 479007900);
    println!("> DAY23 - part 2: OK!");
}
