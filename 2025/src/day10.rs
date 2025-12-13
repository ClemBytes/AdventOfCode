use itertools::Itertools;
use std::{collections::{HashSet, VecDeque}, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY10 -------");
    let example = fs::read_to_string("inputs/example_day10").expect("Unable to read input!");
    let example = Machine::parse(&example);
    let input = fs::read_to_string("inputs/input_day10").expect("Unable to read input!");
    let input = Machine::parse(&input);

    day10_part1(&example, &input);
    day10_part2(&example, &input);
}

#[derive(Debug)]
struct Machine {
    light_diagram: Vec<bool>,
    wiring_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl Machine {
    fn parse(raw_input: &str) -> Vec<Machine> {
        let mut machines = vec![];
        for line in raw_input.lines() {
            let elements: Vec<&str> = line.split_whitespace().collect();

            let mut light_diagram = vec![];
            for c in elements[0].chars() {
                if c == '.' {
                    light_diagram.push(false);
                } else if c == '#' {
                    light_diagram.push(true);
                }
            }

            let nb_elements = elements.len();
            let mut joltage_requirements = vec![];
            let raw_joltage_requirements = elements[nb_elements - 1];
            // Ignore '{' and '}':
            let raw_joltage_requirements =
                &raw_joltage_requirements[1..raw_joltage_requirements.len() - 1];
            let str_joltage_requirements = raw_joltage_requirements.split(",");
            for sjr in str_joltage_requirements {
                joltage_requirements.push(sjr.parse::<usize>().unwrap());
            }

            let mut wiring_schematics = vec![];
            for element_i in elements.iter().take(nb_elements - 1).skip(1) {
                let mut new_wiring = vec![];
                let raw_wiring = element_i;
                // Ignore '(' and ')':
                let raw_wiring = &raw_wiring[1..raw_wiring.len() - 1];
                let str_wiring = raw_wiring.split(",");
                for sw in str_wiring {
                    new_wiring.push(sw.parse::<usize>().unwrap());
                }
                wiring_schematics.push(new_wiring);
            }

            machines.push(Machine {
                light_diagram,
                wiring_schematics,
                joltage_requirements,
            });
        }
        machines
    }
}

fn apply_wiring(current_state: &[bool], wiring: &[usize]) -> Vec<bool> {
    let mut new_state = current_state.to_vec();
    for &w in wiring {
        new_state[w] = !new_state[w];
    }
    new_state
}

fn solve_part1(machines: &[Machine]) -> usize {
    let mut nb_total_presses = 0;
    'main_loop: for machine in machines {
        let objective = machine.light_diagram.clone();
        let nb_lights = objective.len();
        let wirings = machine.wiring_schematics.clone();
        let nb_wirings = wirings.len();
        let mut nb_presses = 0;
        loop {
            if nb_presses > nb_wirings {
                unreachable!("No possible combination!");
            }

            // Get all possible combinations of nb_presses
            let combinations: Vec<Vec<usize>> = (0..nb_wirings).combinations(nb_presses).collect();
            for comb in combinations {
                let mut current_state = vec![false; nb_lights];
                for i in comb {
                    current_state = apply_wiring(&current_state, &wirings[i]);
                }
                if current_state == objective {
                    nb_total_presses += nb_presses;
                    continue 'main_loop;
                }
            }

            nb_presses += 1;
        }
    }
    nb_total_presses
}

fn day10_part1(example: &[Machine], input: &[Machine]) {
    // Exemple tests
    assert_eq!(solve_part1(example), 7);

    // Solve puzzle
    let res = solve_part1(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 475);
    println!("> DAY10 - part 1: OK!");
}

#[derive(Debug, Clone)]
struct State {
    nb_presses: usize,
    joltage_config: Vec<usize>,
}

fn apply_wiring_joltage(current_state: &[usize], wiring: &[usize]) -> Vec<usize> {
    let mut new_state = current_state.to_vec();
    for &w in wiring {
        new_state[w] += 1;
    }
    new_state
}

fn configure_machine(machine: &Machine) -> usize {
    let objective = machine.joltage_requirements.clone();
    let nb_lights = objective.len();
    let wirings = machine.wiring_schematics.clone();
    let start_state = State{
        nb_presses: 0,
        joltage_config: vec![0; nb_lights],
    };
    let mut states_to_explore = VecDeque::new();
    states_to_explore.push_back(start_state);
    let mut seen_configs: HashSet<Vec<usize>> = HashSet::new();
    seen_configs.insert(vec![0; nb_lights]);
    while let Some(state) = states_to_explore.pop_front() {
        if state.joltage_config == objective {
            return state.nb_presses;
        }

        for wiring in wirings.clone() {
            let new_config = apply_wiring_joltage(&state.joltage_config, &wiring);
            if seen_configs.contains(&new_config) {
                continue;
            }
            seen_configs.insert(new_config.clone());

            states_to_explore.push_back(State {
                nb_presses: state.nb_presses + 1,
                joltage_config: new_config,
            });
        }
    }
    unreachable!("Didn't find the correct number of buttons!")
}

fn solve_part2(machines: &[Machine]) -> usize {
    let mut nb_total_presses = 0;
    let nb_machines = machines.len();
    for (i, machine) in machines.iter().enumerate() {
        nb_total_presses += configure_machine(machine);
        println!("{i} / {nb_machines} machines configured!");
    }
    nb_total_presses
}

fn day10_part2(example: &[Machine], input: &[Machine]) {
    // Exemple tests
    assert_eq!(solve_part2(example), 33);
    println!("Example OK");

    // Solve puzzle
    let res = solve_part2(input);
    println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 2: OK!");
}
