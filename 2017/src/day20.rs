use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY20 -------");
    let example1 = fs::read_to_string("inputs/example_day20-1").expect("Unable to read input!");
    let example1 = Particle::parse(&example1);
    let example2 = fs::read_to_string("inputs/example_day20-2").expect("Unable to read input!");
    let example2 = Particle::parse(&example2);
    let input = fs::read_to_string("inputs/input_day20").expect("Unable to read input!");
    let input = Particle::parse(&input);

    day20_part1(&example1, &input);
    day20_part2(&example2, &input);
}

#[derive(Debug, Clone)]
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
            let particle = Particle {
                position: (
                    matches[1].parse().unwrap(),
                    matches[2].parse().unwrap(),
                    matches[3].parse().unwrap(),
                ),
                velocity: (
                    matches[4].parse().unwrap(),
                    matches[5].parse().unwrap(),
                    matches[6].parse().unwrap(),
                ),
                acceleration: (
                    matches[7].parse().unwrap(),
                    matches[8].parse().unwrap(),
                    matches[9].parse().unwrap(),
                ),
            };
            particles.insert(i, particle);
        }
        particles
    }

    fn iterate(&self) -> Self {
        let mut p = self.position;
        let mut v = self.velocity;
        let a = self.acceleration;
        v.0 += a.0;
        v.1 += a.1;
        v.2 += a.2;
        p.0 += v.0;
        p.1 += v.1;
        p.2 += v.2;
        Particle {
            position: p,
            velocity: v,
            acceleration: a,
        }
    }

    fn manhattan_distance(&self) -> i32 {
        let p = &self.position;
        p.0.abs() + p.1.abs() + p.2.abs()
    }

    fn closest_after(n: i32, particles: &HashMap<usize, Self>) -> usize {
        let mut current = particles.clone();
        for _ in 0..n {
            current = current.iter().map(|(i, p)| (*i, p.iterate())).collect();
        }

        current
            .iter()
            .min_by_key(|(_, p)| p.manhattan_distance())
            .map(|(i, _)| *i)
            .unwrap()
    }

    fn collisions(n: i32, particles: &HashMap<usize, Self>) -> usize {
        let mut current = particles.clone();
        for _ in 0..n {
            // New states
            current = current.iter().map(|(i, p)| (*i, p.iterate())).collect();

            // Find collisions
            let mut positions: HashMap<(i32, i32, i32), Vec<usize>> = HashMap::new();
            for (i, particle) in &current {
                let count = positions.entry(particle.position).or_default();
                count.push(*i);
            }
            // Apply collisions
            for (_, particles_ids) in positions {
                if particles_ids.len() > 1 {
                    for particle_id in particles_ids {
                        current.remove(&particle_id);
                    }
                }
            }
        }
        current.len()
    }
}

fn day20_part1(example: &HashMap<usize, Particle>, input: &HashMap<usize, Particle>) {
    // Exemple tests
    assert_eq!(Particle::closest_after(3, example), 0);

    // Solve puzzle
    let mut res = 0;
    for n in [10, 100, 1_000, 2_000] {
        res = Particle::closest_after(n, input);
        println!(
            "Result part1 after {n} iterations: {}",
            Particle::closest_after(n, input)
        );
    }
    println!("Result part 1: {res}");
    assert_eq!(res, 125);
    println!("> DAY20 - part 1: OK!");
}

fn day20_part2(example: &HashMap<usize, Particle>, input: &HashMap<usize, Particle>) {
    // Exemple tests
    assert_eq!(Particle::collisions(5, example), 1);

    // Solve puzzle
    let mut res = 0;
    for n in [10, 100, 1_000] {
        res = Particle::collisions(n, input);
        println!(
            "Result part2 after {n} iterations: {}",
            Particle::collisions(n, input)
        );
    }
    println!("Result part 2: {res}");
    assert_eq!(res, 461);
    println!("> DAY20 - part 2: OK!");
}
