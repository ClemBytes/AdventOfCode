use std::{collections::HashSet, fs};

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

fn parse(raw_input: &str) -> HashSet<(usize, usize)> {
    let mut components = HashSet::new();
    for line in raw_input.lines() {
        let (a, b) = line.split_once("/").unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        if components.contains(&(a, b)) {
            println!("({a}, {b}) already in set!");
        }
        components.insert((a, b));
    }
    components
}

fn strongest_bridge(components: HashSet<(usize, usize)>, current_port: usize) -> usize {
    let mut best = 0;
    for component in components.clone() {
        let (a, b) = component;
        if a == current_port {
            let mut new_components = components.clone();
            new_components.remove(&component);
            best = best.max(strongest_bridge(new_components.clone(), b) + a + b);
        }

        if b == current_port {
            let mut new_components = components.clone();
            new_components.remove(&component);
            best = best.max(strongest_bridge(new_components.clone(), a) + a + b);
        }
    }
    best
}

fn day24_part1(example: &HashSet<(usize, usize)>, input: &HashSet<(usize, usize)>) {
    // Exemple tests
    assert_eq!(strongest_bridge(example.clone(), 0), 31);

    // Solve puzzle
    let res = strongest_bridge(input.clone(), 0);
    println!("Result part 1: {res}");
    assert_eq!(res, 1859);
    println!("> DAY24 - part 1: OK!");
}

fn day24_part2(_example: &HashSet<(usize, usize)>, _input: &HashSet<(usize, usize)>) {
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
