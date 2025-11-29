use std::{collections::HashMap, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY22 -------");
    let example = fs::read_to_string("inputs/example_day22").expect("Unable to read input!");
    let example = parse(&example);
    let example_start_position = (1, 1);
    let input = fs::read_to_string("inputs/input_day22").expect("Unable to read input!");
    let input = parse(&input);
    let input_start_position = (12, 12);

    day22_part1(
        &example,
        example_start_position,
        &input,
        input_start_position,
    );
    day22_part2(
        &example,
        example_start_position,
        &input,
        input_start_position,
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Node {
    Clean,
    Infected,
}

impl Node {
    fn revert(&self) -> Self {
        match self {
            Node::Clean => Node::Infected,
            Node::Infected => Node::Clean,
        }
    }
}

fn parse(raw_input: &str) -> HashMap<(i32, i32), Node> {
    let mut start_grid = HashMap::new();
    for (i, line) in raw_input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let node = match c {
                '.' => Node::Clean,
                '#' => Node::Infected,
                _ => unreachable!("Impossible state '{c}', should be '.' or '#'!"),
            };
            start_grid.insert((i as i32, j as i32), node);
        }
    }
    start_grid
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_position(&self, current_position: (i32, i32)) -> (i32, i32) {
        let (ci, cj) = current_position;
        match self {
            Direction::Up => (ci - 1, cj),
            Direction::Right => (ci, cj + 1),
            Direction::Down => (ci + 1, cj),
            Direction::Left => (ci, cj - 1),
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn solve_part1(
    nb_bursts: i32,
    start_position: (i32, i32),
    start_grid: &HashMap<(i32, i32), Node>,
) -> i32 {
    let mut current_position = start_position;
    let mut current_direction = Direction::Up;
    let mut grid = start_grid.clone();
    let mut nb_new_infections = 0;
    for _ in 0..nb_bursts {
        grid.entry(current_position).or_insert(Node::Clean);

        // Turn left or right
        let n = grid.get(&current_position).unwrap();
        current_direction = match n {
            Node::Clean => current_direction.turn_left(),
            Node::Infected => current_direction.turn_right(),
        };

        // Infect or clean
        let n = n.revert();
        if matches!(n, Node::Infected) {
            nb_new_infections += 1;
        }
        grid.insert(current_position, n);

        // Move
        current_position = current_direction.next_position(current_position);
    }
    nb_new_infections
}

fn day22_part1(
    example: &HashMap<(i32, i32), Node>,
    example_start_position: (i32, i32),
    input: &HashMap<(i32, i32), Node>,
    input_start_position: (i32, i32),
) {
    // Exemple tests
    assert_eq!(solve_part1(7, example_start_position, example), 5);
    assert_eq!(solve_part1(70, example_start_position, example), 41);
    assert_eq!(solve_part1(10_000, example_start_position, example), 5_587);

    // Solve puzzle
    let res = solve_part1(10_000, input_start_position, input);
    println!("Result part 1: {res}");
    assert_eq!(res, 5266);
    println!("> DAY22 - part 1: OK!");
}

fn day22_part2(
    _example: &HashMap<(i32, i32), Node>,
    _example_start_position: (i32, i32),
    _input: &HashMap<(i32, i32), Node>,
    _input_start_position: (i32, i32),
) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY22 - part 2: OK!");
}
