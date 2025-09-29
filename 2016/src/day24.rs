use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY24 -------");
    let example = fs::read_to_string("inputs/example_day24").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day24").expect("Unable to read input!");
    let input = parse(&input);

    day24_part1(&example, &input);
    day24_part2(&example, &input);
}

#[derive(Debug, Clone, Copy)]
enum Location {
    Wall,
    Open,
    Poi(usize),
}

fn parse(raw_input: &str) -> (Vec<Vec<Location>>, (usize, usize), usize) {
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut max_poi = 0;
    for (i, line) in raw_input.lines().enumerate() {
        let mut new_line = vec![];
        for (j, ch) in line.chars().enumerate() {
            let to_add = match ch {
                '#' => Location::Wall,
                '.' => Location::Open,
                x => {
                    let x = x.to_string().parse().unwrap();
                    if x == 0 {
                        start = (i, j);
                    }
                    if x > max_poi {
                        max_poi = x;
                    }
                    Location::Poi(x)
                }
            };
            new_line.push(to_add);
        }
        grid.push(new_line);
    }
    (grid, start, max_poi)
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    visited_pois: Vec<bool>,
    current_position: (usize, usize),
}

fn find_shortest_route(input: &(Vec<Vec<Location>>, (usize, usize), usize)) -> u32 {
    let (grid, start, max_poi) = input;
    let mut visited_pois = vec![false; max_poi + 1];
    visited_pois[0] = true;
    let initial_state = State {
        visited_pois,
        current_position: *start,
    };
    let mut visited_states: HashSet<State> = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((initial_state, 0));
    while let Some((current_state, nb_steps)) = q.pop_front() {
        if current_state.visited_pois.iter().all(|&b| b) {
            return nb_steps;
        }

        if visited_states.contains(&current_state) {
            continue;
        }
        visited_states.insert(current_state.clone());

        let (x, y) = current_state.current_position;
        let mut current_visited_pois = current_state.visited_pois;
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            match grid[nx][ny] {
                Location::Wall => {}
                Location::Open => {
                    q.push_back((
                        State {
                            visited_pois: current_visited_pois.clone(),
                            current_position: (nx, ny),
                        },
                        nb_steps + 1,
                    ));
                }
                Location::Poi(p) => {
                    let prev = current_visited_pois[p];
                    current_visited_pois[p] = true;
                    // println!("Found {p} at ({nx}, {ny}), nb_steps = {}", nb_steps + 1);
                    // println!("Visited: {current_visited_pois:?}\n");
                    q.push_back((
                        State {
                            visited_pois: current_visited_pois.clone(),
                            current_position: (nx, ny),
                        },
                        nb_steps + 1,
                    ));
                    current_visited_pois[p] = prev;
                }
            }
        }
    }
    unreachable!();
}

fn day24_part1(
    example: &(Vec<Vec<Location>>, (usize, usize), usize),
    input: &(Vec<Vec<Location>>, (usize, usize), usize),
) {
    println!("Example: start: {:?} | max_poi: {}", example.1, example.2);
    println!("  Input: start: {:?} | max_poi: {}", input.1, input.2);
    // Exemple tests
    let res = find_shortest_route(example);
    assert_eq!(res, 14);
    println!("Example OK");

    // Solve puzzle
    let res = find_shortest_route(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 412);
    println!("> DAY24 - part 1: OK!");
}

fn day24_part2(
    _example: &(Vec<Vec<Location>>, (usize, usize), usize),
    _input: &(Vec<Vec<Location>>, (usize, usize), usize),
) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY24 - part 2: OK!");
}
