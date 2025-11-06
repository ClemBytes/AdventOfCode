use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY16 -------");
    let input = fs::read_to_string("inputs/input_day16").expect("Unable to read input!");
    let input = parse(&input);

    day16_part1(&input);
    day16_part2(&input);
}

#[derive(Debug)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse(raw_input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let raw_instructions: Vec<&str> = raw_input.trim().split(",").collect();
    for ins in raw_instructions {
        let t = ins.chars().next().unwrap();
        let details = &ins[1..];
        let instruction = match t {
            's' => {
                let nb = details.parse().unwrap();
                Instruction::Spin(nb)
            }
            'x' => {
                let positions = details.split_once("/").unwrap();
                Instruction::Exchange(positions.0.parse().unwrap(), positions.1.parse().unwrap())
            }
            'p' => {
                let programs = details.split_once("/").unwrap();
                Instruction::Partner(programs.0.parse().unwrap(), programs.1.parse().unwrap())
            }
            other => {
                unreachable!("Unkown dance move: {other}");
            }
        };
        instructions.push(instruction);
    }
    instructions
}

fn dance(instructions: &[Instruction], programs: String) -> String {
    let mut res = programs.clone();
    let nb_programs = res.len();
    for instruction in instructions {
        let mut chars: Vec<char> = res.chars().collect();
        res.clear();
        match *instruction {
            Instruction::Spin(n) => {
                for ch in &chars[(nb_programs - n)..] {
                    res.push(*ch);
                }
                for ch in &chars[..(nb_programs - n)] {
                    res.push(*ch);
                }
            }
            Instruction::Exchange(p1, p2) => {
                chars.swap(p1, p2);
                res = chars.into_iter().collect();
            }
            Instruction::Partner(p1, p2) => {
                let p1 = chars.iter().position(|&c| c == p1).unwrap();
                let p2 = chars.iter().position(|&c| c == p2).unwrap();
                chars.swap(p1, p2);
                res = chars.into_iter().collect();
            }
        }
    }
    res
}

fn day16_part1(input: &[Instruction]) {
    // Exemple tests
    let example = vec![
        Instruction::Spin(1),
        Instruction::Exchange(3, 4),
        Instruction::Partner('e', 'b'),
    ];
    assert_eq!(dance(&example, "abcde".to_string()), "baedc");

    // Solve puzzle
    let res = dance(input, "abcdefghijklmnop".to_string());
    println!("Result part 1: {res}");
    assert_eq!(res, "olgejankfhbmpidc");
    println!("> DAY16 - part 1: OK!");
}

fn day16_part2(_input: &[Instruction]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY16 - part 2: OK!");
}
