use itertools::Itertools;
use ndarray::{Array1, Array2};
use std::fs;
use ndarray_linalg::LeastSquaresSvd;

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

fn configure_machine(machine: &Machine) -> usize {
    // We want to solve an equation system. Let's take the first example:
    // Wirings: (3) (1,3) (2) (2,3) (0,2) (0,1)
    // Joltage requirements: {3,5,4,7}
    // I want to find N = N0 + N1 + N2 + … + N5  such that:
    // N1×(3) N2×(1,3) N3×(2) N4×(2,3) N5×(0,2) N6×(0,1) gives {3,5,4,7}
    // So I want:
    // N5 + N6 = 3
    // N2 + N6 = 5
    // N3 + N4 + N5 = 4
    // N1 + N2 + N4 = 7
    // Or as a matrix
    //                 |N1| 
    // |0 0 0 0 1 1|   |N2|   |3|
    // |0 1 0 0 0 1| × |N3| = |5|
    // |0 0 1 1 1 0|   |N4|   |4|
    // |1 1 0 1 0 0|   |N5|   |7|
    //                 |N6|

    // Solution should be: (1, 3, 0, 3, 1, 2)

    let objective = machine.joltage_requirements.clone();
    let objective_f64: Vec<f64> = objective.iter().map(|&x| x as f64).collect();
    let b = Array1::from_vec(objective_f64.clone());
    let nb_lights = objective.len();
    let wirings = machine.wiring_schematics.clone();
    let nb_wirings = wirings.len();

    // Create matrix:
    let mut matrix = vec![];
    for l in 0..nb_lights {
        for w in &wirings {
            if w.contains(&l) {
                matrix.push(1.);
            } else {
                matrix.push(0.);
            }
        }
    }
    println!("matrix:\n{matrix:?}");
    let a = Array2::from_shape_vec((nb_lights, nb_wirings), matrix).unwrap();
    println!("a:\n{a:?}");
    println!("b: {b:?}");

    // Now solve
    let x = a.least_squares(&b).unwrap().solution;

    println!("x: {x:?}");
    let x_int: Vec<usize> = x.iter().map(|v| v.round() as usize).collect();
    println!("x_int: {x_int:?}");

    x_int.iter().sum()
}

fn solve_part2(machines: &[Machine]) -> usize {
    let mut nb_total_presses = 0;
    let nb_machines = machines.len();
    for (i, machine) in machines.iter().enumerate() {
        nb_total_presses += configure_machine(machine);
        println!("{} / {nb_machines} machines configured!", i + 1);
    }
    nb_total_presses
}

fn day10_part2(example: &[Machine], input: &[Machine]) {
    // Exemple tests
    assert_eq!(configure_machine(&example[0]), 10);
    assert_eq!(configure_machine(&example[1]), 12);
    assert_eq!(configure_machine(&example[2]), 11);
    assert_eq!(solve_part2(example), 33);
    println!("Example OK");

    // Solve puzzle
    let res = solve_part2(input);
    println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 2: OK!");
}
