use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY14 -------");
    let input = fs::read_to_string("inputs/input_day14").expect("Unable to read input!");
    let input = parse(&input);

    day14_part1(&input);
    day14_part2(&input);
}

struct Reindeer {
    speed: u32,
    duration_flight: u32,
    duration_rest: u32,
}

fn parse(raw_input: &str) -> Vec<Reindeer> {
    let mut reindeers_list: Vec<Reindeer> = vec![];
    let re = Regex::new(r"^[A-Z][a-z]+ can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds\.$").unwrap();
    for line in raw_input.lines() {
        let matches = re.captures(line).unwrap();
        let speed: u32 = matches[1].parse().unwrap();
        let duration_flight: u32 = matches[2].parse().unwrap();
        let duration_rest: u32 = matches[3].parse().unwrap();
        println!("{speed}, {duration_flight}, {duration_rest}");
        let new_reindeer = Reindeer {
            speed,
            duration_flight,
            duration_rest,
        };
        reindeers_list.push(new_reindeer);
    }
    reindeers_list
}

fn get_distance_at_time_t(reindeer: &Reindeer, t: u32) -> u32 {
    // Get number complete loops fly+rest
    let duration_loop = reindeer.duration_flight + reindeer.duration_rest;
    let number_complete_loops = t / (duration_loop);
    // Get distance during those loops
    let mut distance = number_complete_loops * reindeer.speed * reindeer.duration_flight;
    // Get time left after those loops
    let time_left = t % duration_loop;
    // Update distance after those loops
    if time_left > reindeer.duration_flight {
        distance += reindeer.duration_flight * reindeer.speed;
    } else {
        distance += time_left * reindeer.speed;
    }
    distance
}

fn get_distance_max(reindeers_list: &[Reindeer], t: u32) -> u32 {
    reindeers_list
        .iter()
        .map(|reindeer| get_distance_at_time_t(reindeer, t))
        .max()
        .unwrap()
}

fn day14_part1(input: &[Reindeer]) {
    // Exemple tests
    let comet = Reindeer {
        speed: 14,
        duration_flight: 10,
        duration_rest: 127,
    };
    assert_eq!(get_distance_at_time_t(&comet, 1000), 1120);
    let dancer = Reindeer {
        speed: 16,
        duration_flight: 11,
        duration_rest: 162,
    };
    assert_eq!(get_distance_at_time_t(&dancer, 1000), 1056);

    // Solve puzzle
    let res = get_distance_max(input, 2503);
    println!("Result part 1: {res}");
    assert_eq!(res, 2660);
    println!("> DAY14 - part 1: OK!");
}

fn race_with_points(reindeers_list: &[Reindeer], t: u32) -> u32 {
    let nb_reindeers = reindeers_list.len();
    let mut distances: Vec<u32> = vec![0; nb_reindeers];
    let mut points: Vec<u32> = vec![0; nb_reindeers];
    for second in 1..(t + 1) {
        for (i, reindeer) in reindeers_list.iter().enumerate() {
            distances[i] = get_distance_at_time_t(reindeer, second);
        }
        let &max_distance = distances.iter().max().unwrap();
        for i in 0..nb_reindeers {
            if distances[i] == max_distance {
                points[i] += 1;
            }
        }
    }
    *points.iter().max().unwrap()
}

fn day14_part2(input: &[Reindeer]) {
    // Exemple tests
    let reindeers_list = vec![
        Reindeer {
            speed: 14,
            duration_flight: 10,
            duration_rest: 127,
        },
        Reindeer {
            speed: 16,
            duration_flight: 11,
            duration_rest: 162,
        },
    ];
    assert_eq!(race_with_points(&reindeers_list, 1000), 689);

    // Solve puzzle
    let res = race_with_points(input, 2503);
    println!("Result part 2: {res}");
    assert_eq!(res, 1256);
    println!("> DAY14 - part 2: OK!");
}
