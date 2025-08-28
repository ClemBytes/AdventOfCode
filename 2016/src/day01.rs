use std::{collections::HashSet, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY01 -------");
    let input = fs::read_to_string("inputs/input_day01").expect("Unable to read input!");
    let input = parse(&input);

    day01_part1(&input);
    day01_part2(&input);
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Right,
    Left,
}

impl Turn {
    fn from_char(ch: char) -> Self {
        match ch {
            'R' => Turn::Right,
            'L' => Turn::Left,
            other => panic!("Unkwnow Turn '{other}'! Should be 'R' or 'L'."),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    turn: Turn,
    nb_blocks: i32,
}

fn parse(raw_input: &str) -> Vec<Instruction> {
    let raw_instructions: Vec<&str> = raw_input.trim().split(", ").collect();
    let mut instructions = vec![];
    for instruction in raw_instructions {
        let mut chars = instruction.chars();
        let first_char = chars.next().unwrap();
        let rest: String = chars.collect();
        let turn = Turn::from_char(first_char);
        let nb_blocks = rest.parse().unwrap();
        instructions.push(Instruction { turn, nb_blocks });
    }
    instructions
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum Facing {
    North,
    South,
    East,
    West,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct State {
    facing: Facing,
    n_s_coord: i32,
    e_w_coord: i32,
}

impl State {
    fn turn(&mut self, t: Turn) {
        self.facing = match t {
            Turn::Right => match self.facing {
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North,
            },
            Turn::Left => match self.facing {
                Facing::North => Facing::West,
                Facing::West => Facing::South,
                Facing::South => Facing::East,
                Facing::East => Facing::North,
            },
        }
    }

    fn walk(&mut self, nb: i32) {
        match self.facing {
            Facing::North => self.n_s_coord += nb,
            Facing::South => self.n_s_coord -= nb,
            Facing::East => self.e_w_coord += nb,
            Facing::West => self.e_w_coord -= nb,
        }
    }
}

fn blocks_away(instructions: &[Instruction]) -> i32 {
    let mut state = State {
        facing: Facing::North,
        n_s_coord: 0,
        e_w_coord: 0,
    };

    for instruction in instructions {
        state.turn(instruction.turn);
        state.walk(instruction.nb_blocks);
    }

    state.n_s_coord.abs() + state.e_w_coord.abs()
}

fn day01_part1(input: &[Instruction]) {
    // Exemple tests
    let ex = [
        Instruction {
            turn: Turn::Right,
            nb_blocks: 2,
        },
        Instruction {
            turn: Turn::Left,
            nb_blocks: 3,
        },
    ];
    assert_eq!(blocks_away(&ex), 5);
    let ex = [
        Instruction {
            turn: Turn::Right,
            nb_blocks: 2,
        },
        Instruction {
            turn: Turn::Right,
            nb_blocks: 2,
        },
        Instruction {
            turn: Turn::Right,
            nb_blocks: 2,
        },
    ];
    assert_eq!(blocks_away(&ex), 2);
    let ex = [
        Instruction {
            turn: Turn::Right,
            nb_blocks: 5,
        },
        Instruction {
            turn: Turn::Left,
            nb_blocks: 5,
        },
        Instruction {
            turn: Turn::Right,
            nb_blocks: 5,
        },
        Instruction {
            turn: Turn::Right,
            nb_blocks: 3,
        },
    ];
    assert_eq!(blocks_away(&ex), 12);

    // Solve puzzle
    let res = blocks_away(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 300);
    println!("> DAY01 - part 1: OK!");
}

fn visit_twice(instructions: &[Instruction]) -> i32 {
    let mut visited_states: HashSet<(i32, i32)> = HashSet::new();

    let mut state = State {
        facing: Facing::North,
        n_s_coord: 0,
        e_w_coord: 0,
    };
    visited_states.insert((state.n_s_coord, state.e_w_coord));

    for instruction in instructions {
        state.turn(instruction.turn);
        for _ in 0..instruction.nb_blocks {
            state.walk(1);
            if visited_states.contains(&(state.n_s_coord, state.e_w_coord)) {
                return state.n_s_coord.abs() + state.e_w_coord.abs();
            }
            visited_states.insert((state.n_s_coord, state.e_w_coord));
        }
    }
    unreachable!();
}

fn day01_part2(input: &[Instruction]) {
    // Exemple tests
    let ex = [
        Instruction {
            turn: Turn::Right,
            nb_blocks: 8,
        },
        Instruction {
            turn: Turn::Right,
            nb_blocks: 4,
        },
        Instruction {
            turn: Turn::Right,
            nb_blocks: 4,
        },
        Instruction {
            turn: Turn::Right,
            nb_blocks: 8,
        },
    ];
    assert_eq!(visit_twice(&ex), 4);

    // Solve puzzle
    let res = visit_twice(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 159);
    println!("> DAY01 - part 2: OK!");
}
