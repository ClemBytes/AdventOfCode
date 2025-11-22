use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY20 -------");
    let example = fs::read_to_string("inputs/example_day20").expect("Unable to read input!");
    let example = Particle::parse(&example);
    let input = fs::read_to_string("inputs/input_day20").expect("Unable to read input!");
    let input = Particle::parse(&input);

    day20_part1(&example, &input);
    day20_part2(&example, &input);
}

#[derive(Debug)]
struct Particle {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
    acceleration: (i32, i32, i32),
}

impl Particle {
    fn parse(raw_input: &str) -> HashMap<usize, Self> {
        let mut particles = HashMap::new();
        let r = Regex::new(r"^p=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, v=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, a=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>$").unwrap();
        for (i, line) in raw_input.lines().enumerate() {
            let matches = r.captures(line).unwrap();
            let particle = Particle{
                position: (matches[1].parse().unwrap(), matches[2].parse().unwrap(), matches[3].parse().unwrap()),
                velocity: (matches[4].parse().unwrap(), matches[5].parse().unwrap(), matches[6].parse().unwrap()),
                acceleration: (matches[7].parse().unwrap(), matches[8].parse().unwrap(), matches[9].parse().unwrap()),
            };
            particles.insert(i, particle);
        }
        particles
    }
}

fn day20_part1(example: &HashMap<usize, Particle>, _input: &HashMap<usize, Particle>) {
    println!("{example:#?}");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY20 - part 1: OK!");
}

fn day20_part2(_example: &HashMap<usize, Particle>, _input: &HashMap<usize, Particle>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY20 - part 2: OK!");
}
