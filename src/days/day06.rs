use std::fs;

pub fn run() {
    println!("------- DAY06 -------");
    let raw_input = fs::read_to_string("inputs/input_day06").expect("Unable to read input!");
    let input = parse(&raw_input);
    day06_part1(&input);
    day06_part2(&input);
}

fn day06_part1(input: &Vec<Instruction>) {
    // Solve puzzle
    let mut grid: Vec<[u32; 1000]> = vec![[0; 1000]; 1000];
    for instruction in input {
        instruction.apply_instruction_part1(&mut grid);
    }
    // println!("Result part 1: {}", count_lights(&grid));
    assert_eq!(count_lights(&grid), 569999);
    println!("> DAY06 - part 1: OK!");
}

fn day06_part2(input: &Vec<Instruction>) {
    // Solve puzzle
    let mut grid: Vec<[u32; 1000]> = vec![[0; 1000]; 1000];
    for instruction in input {
        instruction.apply_instruction_part2(&mut grid);
    }
    // println!("Result part 2: {}", count_lights(&grid));
    assert_eq!(count_lights(&grid), 17836115);
    println!("> DAY06 - part 2: OK!");
}

enum Command {
    On,
    Off,
    Toggle,
}

struct Instruction {
    command: Command,
    start: (usize, usize),
    end: (usize, usize),
}

impl Instruction {
    fn read_instruction(instruction_text: &str) -> Self {
        let words: Vec<&str> = instruction_text.split_whitespace().collect();
        let command = match words[0] {
            "on" => Command::On,
            "off" => Command::Off,
            "toggle" => Command::Toggle,
            other => panic!("Unknown command {other}!"),
        };
        let start_vec: Vec<&str> = words[1].split(',').collect();
        let start: (usize, usize) = (start_vec[0].parse().unwrap(), start_vec[1].parse().unwrap());
        let end_vec: Vec<&str> = words[3].split(',').collect();
        let end: (usize, usize) = (end_vec[0].parse().unwrap(), end_vec[1].parse().unwrap());

        Instruction {
            command,
            start,
            end,
        }
    }

    fn apply_instruction_part1(&self, grid: &mut [[u32; 1000]]) {
        let command = &self.command;
        for x in grid.iter_mut().take(self.end.0 + 1).skip(self.start.0) {
            for light in x.iter_mut().take(self.end.1 + 1).skip(self.start.1) {
                match command {
                    Command::On => *light = 1,
                    Command::Off => *light = 0,
                    Command::Toggle => *light = 1 - *light,
                }
            }
        }
    }

    fn apply_instruction_part2(&self, grid: &mut [[u32; 1000]]) {
        let command = &self.command;
        for x in grid.iter_mut().take(self.end.0 + 1).skip(self.start.0) {
            for light in x.iter_mut().take(self.end.1 + 1).skip(self.start.1) {
                match command {
                    Command::On => *light += 1,
                    Command::Off => {
                        if *light > 0 {
                            *light -= 1;
                        }
                    }
                    Command::Toggle => *light += 2,
                }
            }
        }
    }
}

fn parse(raw_input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for line in raw_input.lines() {
        instructions.push(Instruction::read_instruction(line));
    }
    instructions
}

fn count_lights(grid: &Vec<[u32; 1000]>) -> u32 {
    let mut count = 0;
    for line in grid {
        for position in line {
            count += position;
        }
    }
    count
}
