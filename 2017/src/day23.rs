use std::{collections::HashMap, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY23 -------");
    let input = fs::read_to_string("inputs/input_day23").expect("Unable to read input!");
    let input = Instruction::parse(&input);

    day23_part1(&input);
    day23_part2(&input);
}

#[derive(Debug, Clone)]
enum RegisterOrValue {
    Register(char),
    Value(i64),
}

impl RegisterOrValue {
    fn parse(rv: &str) -> Self {
        let res = rv.parse::<i64>();
        match res {
            Ok(v) => RegisterOrValue::Value(v),
            Err(_) => RegisterOrValue::Register(rv.parse::<char>().unwrap()),
        }
    }

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
    Set(char, RegisterOrValue),
    Sub(char, RegisterOrValue),
    Mul(char, RegisterOrValue),
    Jnz(RegisterOrValue, i64),
}

impl Instruction {
    fn parse(raw_input: &str) -> Vec<Self> {
        let mut instructions = vec![];
        for line in raw_input.lines() {
            let (kind, infos) = line.split_once(" ").unwrap();
            let i = match kind {
                "set" => {
                    let (x, y) = infos.split_once(" ").unwrap();
                    Instruction::Set(x.parse::<char>().unwrap(), RegisterOrValue::parse(y))
                }
                "sub" => {
                    let (x, y) = infos.split_once(" ").unwrap();
                    Instruction::Sub(x.parse::<char>().unwrap(), RegisterOrValue::parse(y))
                }
                "mul" => {
                    let (x, y) = infos.split_once(" ").unwrap();
                    Instruction::Mul(x.parse::<char>().unwrap(), RegisterOrValue::parse(y))
                }
                "jnz" => {
                    let (x, y) = infos.split_once(" ").unwrap();
                    Instruction::Jnz(RegisterOrValue::parse(x), y.parse::<i64>().unwrap())
                }
                _ => unreachable!("Unknown instruction: '{kind}'!"),
            };
            instructions.push(i);
        }
        instructions
    }
}

fn solve_part1(instructions: &[Instruction], registers: HashMap<char, i64>) -> i64 {
    let mut nb_mul = 0;
    let nb_instructions = instructions.len() as i64;
    let mut position = 0;
    let mut registers = registers.clone();
    loop {
        // Termination condition
        if (position < 0) || (position >= nb_instructions) {
            return nb_mul;
        }

        match &instructions[position as usize] {
            Instruction::Set(r, rv) => {
                let v = rv.get(&registers);
                registers.insert(*r, v);
                position += 1;
            }
            Instruction::Sub(r, rv) => {
                let v = rv.get(&registers);
                let previous_val = registers.get(r).unwrap();
                registers.insert(*r, *previous_val - v);
                position += 1;
            }
            Instruction::Mul(r, rv) => {
                nb_mul += 1;
                let v = rv.get(&registers);
                let previous_val = registers.get(r).unwrap();
                registers.insert(*r, *previous_val * v);
                position += 1;
            }
            Instruction::Jnz(rv, v) => {
                let check = rv.get(&registers);
                if check != 0 {
                    position += v;
                } else {
                    position += 1;
                }
            }
        }
    }
}

fn day23_part1(input: &[Instruction]) {
    // Solve puzzle
    let mut registers = HashMap::new();
    registers.insert('a', 0i64);
    registers.insert('b', 0i64);
    registers.insert('c', 0i64);
    registers.insert('d', 0i64);
    registers.insert('e', 0i64);
    registers.insert('f', 0i64);
    registers.insert('g', 0i64);
    registers.insert('h', 0i64);
    let res = solve_part1(input, registers);
    println!("Result part 1: {res}");
    assert_eq!(res, 5929);
    println!("> DAY23 - part 1: OK!");
}

fn solve_part2() -> i64 {
    let mut h = 0;
    let mut b = 79 * 100 - 100_000; // -92_100
    let c = b - 17_000; // -109_100
    loop {
        let mut f = 1;
        let mut d = 2;
        let mut e = 2;
        let mut g = d * e - b;
        while g != 0 {
            e = 2;
            g = d * e - b;
            while g != 0 {
                if d * e - b == 0 {
                    f = 0;
                }
                e -= 1;
                g = e - b;
            }
            // g == 0
            d -= 1;
            g = d - b;
        }
        // g == 0
        if f == 0 {
            h -= 1;
        }
        g = b - c;
        if g == 0 {
            return h;
        }
        b -= 17;
    }
}

fn day23_part2(_input: &[Instruction]) {
    // Solve puzzle
    let res = solve_part2();
    println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY23 - part 2: OK!");
}
