use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY08 -------");
    let example = fs::read_to_string("inputs/example_day08").expect("Unable to read input!");
    let example = Instruction::parse(&example);
    let input = fs::read_to_string("inputs/input_day08").expect("Unable to read input!");
    let input = Instruction::parse(&input);

    day08(&example, &input);
}

#[derive(Debug, Clone, Copy)]
enum RotationOn {
    Column,
    Row,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    on: RotationOn,
    id: usize,
    shift: usize,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Rect(usize, usize),
    Rotate(Rotation),
}

impl Instruction {
    fn parse(raw_input: &str) -> Vec<Self> {
        let mut instructions: Vec<Instruction> = vec![];
        let re_rect = Regex::new(r"([0-9]+)x([0-9]+)").unwrap();
        let re_rotate_col = Regex::new(r"x=([0-9]+) by ([0-9]+)").unwrap();
        let re_rotate_row = Regex::new(r"y=([0-9]+) by ([0-9]+)").unwrap();
        for line in raw_input.lines() {
            let (kind, infos) = line.split_once(" ").unwrap();
            match kind {
                "rect" => {
                    let matches = re_rect.captures(infos).unwrap();
                    instructions.push(Instruction::Rect(
                        matches[1].parse().unwrap(),
                        matches[2].parse().unwrap(),
                    ));
                }
                "rotate" => {
                    let (on, infos_rotation) = infos.split_once(" ").unwrap();
                    match on {
                        "column" => {
                            let matches = re_rotate_col.captures(infos_rotation).unwrap();
                            instructions.push(Instruction::Rotate(Rotation {
                                on: RotationOn::Column,
                                id: matches[1].parse().unwrap(),
                                shift: matches[2].parse().unwrap(),
                            }));
                        }
                        "row" => {
                            let matches = re_rotate_row.captures(infos_rotation).unwrap();
                            instructions.push(Instruction::Rotate(Rotation {
                                on: RotationOn::Row,
                                id: matches[1].parse().unwrap(),
                                shift: matches[2].parse().unwrap(),
                            }));
                        }
                        _ => panic!("Unknown rotation: {on} ({infos_rotation})"),
                    }
                }
                _ => panic!("Unknown instruction: {kind} ({infos})"),
            }
        }
        instructions
    }

    // Avoid having clippy making me replace my for loops by grid.iter_mut().take(…)
    #[allow(clippy::needless_range_loop)]
    fn apply(&self, grid: &mut [Vec<bool>]) {
        let nb_rows = grid.len();
        let nb_cols = grid[0].len();
        match self {
            Instruction::Rect(a, b) => {
                for i in 0..*b {
                    for j in 0..*a {
                        grid[i][j] = true;
                    }
                }
            }
            Instruction::Rotate(infos) => match infos.on {
                RotationOn::Column => {
                    let mut column_to_rotate = vec![];
                    for i in 0..nb_rows {
                        column_to_rotate.push(grid[i][infos.id]);
                    }
                    for i in 0..nb_rows {
                        grid[(i + infos.shift) % nb_rows][infos.id] = column_to_rotate[i];
                    }
                }
                RotationOn::Row => {
                    let row_to_rotate = grid[infos.id].clone();
                    for j in 0..nb_cols {
                        grid[infos.id][(j + infos.shift) % nb_cols] = row_to_rotate[j];
                    }
                }
            },
        }
    }
}

fn initiate_grid(nb_rows: usize, nb_cols: usize) -> Vec<Vec<bool>> {
    vec![vec![false; nb_cols]; nb_rows]
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        let mut new_row = String::new();
        for pos in row {
            if *pos {
                new_row.push('#');
            } else {
                new_row.push('.');
            }
        }
        println!("{new_row}");
        new_row.clear();
    }
}

fn count_lit(grid: &Vec<Vec<bool>>) -> u32 {
    let mut s = 0;
    for row in grid {
        for pos in row {
            if *pos {
                s += 1;
            }
        }
    }
    s
}

fn day08(example: &Vec<Instruction>, input: &Vec<Instruction>) {
    let mut grid = initiate_grid(3, 7);
    for instruction in example {
        instruction.apply(&mut grid);
        // print_grid(&grid);
        // println!();
    }
    // Exemple tests
    assert_eq!(count_lit(&grid), 6);

    // Solve puzzle
    let mut grid = initiate_grid(6, 50);
    for instruction in input {
        instruction.apply(&mut grid);
    }
    let res = count_lit(&grid);
    println!("Result part 1: {res}");
    assert_eq!(res, 121);
    println!("> DAY08 - part 1: OK!");
    println!("Result part 2:");
    print_grid(&grid);
    println!("> DAY08 - part 2: OK! (RURUCEOEIL)");
}
