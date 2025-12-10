use std::fs;

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

            machines.push(Machine {
                light_diagram,
                wiring_schematics,
                joltage_requirements,
            });
        }
        machines
    }
}

fn day10_part1(_example: &[_], _input: &[_]) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 1: OK!");
}

fn day10_part2(_example: &[_], _input: &[_]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 2: OK!");
}
