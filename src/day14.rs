use std::fs;

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
    for line in raw_input.lines() {
        // First split: delete "XX can fly "
        let line = line.split(" can fly ").collect::<Vec<&str>>()[1];
        // Then: get speed
        let split: Vec<&str> = line.split(" km/s for ").collect();
        let speed: u32 = split[0].parse().unwrap();
        let line = split[1];
        // Then: get duration_flight
        let split: Vec<&str> = line.split(" seconds, but then must rest for ").collect();
        let duration_flight: u32 = split[0].parse().unwrap();
        let line = split[1];
        // Finally: get duration_rest
        let split: Vec<&str> = line.split(" seconds.").collect();
        let duration_rest: u32 = split[0].parse().unwrap();
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

fn day14_part2(_input: &[Reindeer]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY14 - part 2: OK!");
}
