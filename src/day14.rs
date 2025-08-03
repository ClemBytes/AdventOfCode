use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY14 -------");
    let input = fs::read_to_string("inputs/input_day14").expect("Unable to read input!");
    let mut input = parse(&input);

    day14_part1(&input);
    day14_part2(&mut input);
}

struct Reindeer {
    speed: u32,
    duration_flight: u32,
    duration_rest: u32,
    points: u32,
    distance: u32,
    is_flying: bool,
    time_before_state_change: u32,
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
            points: 0,
            distance: 0,
            is_flying: true,
            time_before_state_change: duration_flight,
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
        points: 0,
        distance: 0,
        is_flying: true,
        time_before_state_change: 10,
    };
    assert_eq!(get_distance_at_time_t(&comet, 1000), 1120);
    let dancer = Reindeer {
        speed: 16,
        duration_flight: 11,
        duration_rest: 162,
        points: 0,
        distance: 0,
        is_flying: true,
        time_before_state_change: 11,
    };
    assert_eq!(get_distance_at_time_t(&dancer, 1000), 1056);

    // Solve puzzle
    let res = get_distance_max(input, 2503);
    println!("Result part 1: {res}");
    assert_eq!(res, 2660);
    println!("> DAY14 - part 1: OK!");
}

fn race_with_points(reindeers_list: &mut [Reindeer], t: u32) {
    for _ in 0..t {
        let mut max_distance = 0;
        for reindeer in reindeers_list.iter_mut() {
            if reindeer.is_flying {
                reindeer.distance += reindeer.speed;
                reindeer.time_before_state_change -= 1;
                if reindeer.time_before_state_change == 0 {
                    reindeer.is_flying = false;
                    reindeer.time_before_state_change = reindeer.duration_rest;
                }
            } else {
                reindeer.time_before_state_change -= 1;
                if reindeer.time_before_state_change == 0 {
                    reindeer.is_flying = true;
                    reindeer.time_before_state_change = reindeer.duration_flight;
                }
            }
            if reindeer.distance > max_distance {
                max_distance = reindeer.distance;
            }
        }
        for reindeer in reindeers_list.iter_mut() {
            if reindeer.distance == max_distance {
                reindeer.points += 1;
            }
        }
    }
}

fn get_points_max(reindeers_list: &[Reindeer]) -> u32 {
    reindeers_list
        .iter()
        .map(|reindeer| reindeer.points)
        .max()
        .unwrap()
}

fn day14_part2(input: &mut [Reindeer]) {
    // Exemple tests
    let mut reindeers_list = vec![
        Reindeer {
            speed: 14,
            duration_flight: 10,
            duration_rest: 127,
            points: 0,
            distance: 0,
            is_flying: true,
            time_before_state_change: 10,
        },
        Reindeer {
            speed: 16,
            duration_flight: 11,
            duration_rest: 162,
            points: 0,
            distance: 0,
            is_flying: true,
            time_before_state_change: 11,
        },
    ];
    race_with_points(&mut reindeers_list, 1000);
    assert_eq!(reindeers_list[0].points, 312);
    assert_eq!(reindeers_list[1].points, 689);

    // Solve puzzle
    race_with_points(input, 2503);
    let res = get_points_max(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 1256);
    println!("> DAY14 - part 2: OK!");
}
