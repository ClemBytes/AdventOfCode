
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[test]
fn test() {
    run();
}


pub fn run() {
    println!("------- DAY09 -------");
    let example = fs::read_to_string("inputs/example_day09").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day09").expect("Unable to read input!");
    let input = parse(&input);

    day09_part1(&example, &input);
    day09_part2(&example, &input);
}

fn parse(raw_input: &str) -> (HashMap<(&str, &str), u32>, HashSet<&str>) {
    let mut distances: HashMap<(&str, &str), u32> = HashMap::new();
    let mut cities_names: HashSet<&str> = HashSet::new();
    for line in raw_input.lines() {
        let infos: Vec<&str> = line.split(" = ").collect();
        let (prefix, dist): (&str, &str) = (infos[0], infos[1]);
        let cities: Vec<&str> = prefix.split(" to ").collect();
        let dist: u32 = dist.parse().unwrap();
        distances.insert((cities[0], cities[1]), dist);
        distances.insert((cities[1], cities[0]), dist);
        cities_names.insert(cities[0]);
        cities_names.insert(cities[1]);
    }
    (distances, cities_names)
}

// neighbors: HashMap<&str, Vec<(&str, u32)>>

fn get_distance(city1: &str, city2: &str, distances: &HashMap<(&str, &str), u32>) -> u32 {
    let couple = (city1, city2);
    *distances.get(&couple).unwrap()
}

fn recursive_search_min(
    distances: &HashMap<(&str, &str), u32>,
    cities_names: &mut VecDeque<&str>,
    current_city: &str,
) -> u32 {
    if cities_names.is_empty() {
        return 0;
    }
    let mut min_dist = u32::MAX;
    let n = cities_names.len();
    for _ in 0..n {
        let city = cities_names.pop_front().unwrap();
        let total_dist: u32 = get_distance(current_city, city, distances)
            + recursive_search_min(distances, cities_names, city);
        if total_dist < min_dist {
            min_dist = total_dist;
        }
        cities_names.push_back(city);
    }
    min_dist
}

fn search_min(distances: &HashMap<(&str, &str), u32>, cities_names: &mut VecDeque<&str>) -> u32 {
    let mut min_dist = u32::MAX;
    let n = cities_names.len();
    for _ in 0..n {
        let city = cities_names.pop_front().unwrap();
        let total_dist = recursive_search_min(distances, cities_names, city);
        if total_dist < min_dist {
            min_dist = total_dist;
        }
        cities_names.push_back(city);
    }
    min_dist
}

fn recursive_search_max(
    distances: &HashMap<(&str, &str), u32>,
    cities_names: &mut VecDeque<&str>,
    current_city: &str,
) -> u32 {
    if cities_names.is_empty() {
        return 0;
    }
    let mut max_dist = 0;
    let n = cities_names.len();
    for _ in 0..n {
        let city = cities_names.pop_front().unwrap();
        let total_dist = get_distance(current_city, city, distances)
            + recursive_search_max(distances, cities_names, city);
        if total_dist > max_dist {
            max_dist = total_dist;
        }
        cities_names.push_back(city);
    }
    max_dist
}

fn search_max(distances: &HashMap<(&str, &str), u32>, cities_names: &mut VecDeque<&str>) -> u32 {
    let mut max_dist = 0;
    let n = cities_names.len();
    for _ in 0..n {
        let city = cities_names.pop_front().unwrap();
        let total_dist = recursive_search_max(distances, cities_names, city);
        if total_dist > max_dist {
            max_dist = total_dist;
        }
        cities_names.push_back(city);
    }
    max_dist
}

fn day09_part1(
    example: &(HashMap<(&str, &str), u32>, HashSet<&str>),
    input: &(HashMap<(&str, &str), u32>, HashSet<&str>),
) {
    // Exemple tests
    let (example_distances, example_cities_names) = example;
    let mut example_cities_names: VecDeque<&str> = example_cities_names.iter().cloned().collect();
    println!(
        "Example part1 : {}",
        search_min(example_distances, &mut example_cities_names)
    );
    assert_eq!(
        search_min(example_distances, &mut example_cities_names),
        605
    );

    // Solve puzzle
    let (input_distances, input_cities_names) = input;
    let mut input_cities_names: VecDeque<&str> = input_cities_names.iter().cloned().collect();
    println!(
        "Result part 1: {}",
        search_min(input_distances, &mut input_cities_names)
    );
    assert_eq!(search_min(input_distances, &mut input_cities_names), 141);
    println!("> DAY09 - part 1: OK!");
}

fn day09_part2(
    example: &(HashMap<(&str, &str), u32>, HashSet<&str>),
    input: &(HashMap<(&str, &str), u32>, HashSet<&str>),
) {
    // Exemple tests
    let (example_distances, example_cities_names) = example;
    let mut example_cities_names: VecDeque<&str> = example_cities_names.iter().cloned().collect();
    println!(
        "Example part1 : {}",
        search_max(example_distances, &mut example_cities_names)
    );
    assert_eq!(
        search_max(example_distances, &mut example_cities_names),
        982
    );

    // Solve puzzle
    let (input_distances, input_cities_names) = input;
    let mut input_cities_names: VecDeque<&str> = input_cities_names.iter().cloned().collect();
    println!(
        "Result part 2: {}",
        search_max(input_distances, &mut input_cities_names)
    );
    assert_eq!(
        search_max(input_distances, &mut input_cities_names),
        736
    );
    println!("> DAY09 - part 2: OK!");
}
