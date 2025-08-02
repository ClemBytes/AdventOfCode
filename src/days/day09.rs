use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

fn get_distance(city1: &str, city2: &str, distances: HashMap<(&str, &str), u32>) -> u32 {
    let couple = (city1, city2);
    *distances.get(&couple).unwrap()
}

fn recursive_search(
    distances: HashMap<(&str, &str), u32>,
    cities_names: Vec<&str>,
    current_city: &str,
) -> u32 {
    if cities_names.is_empty() {
        return 0;
    }
    let mut min_dist = u32::MAX;
    for city in &cities_names {
        let cities_name_reduced = cities_names
            .iter()
            .copied()
            .filter(|&x| x != *city)
            .collect();
        let total_dist = get_distance(current_city, city, distances.clone())
            + recursive_search(distances.clone(), cities_name_reduced, city);
        if total_dist < min_dist {
            min_dist = total_dist;
        }
    }
    min_dist
}

fn search(distances: HashMap<(&str, &str), u32>, cities_names: Vec<&str>) -> u32 {
    let mut min_dist = u32::MAX;
    for city in &cities_names {
        let cities_name_reduced = cities_names
            .iter()
            .copied()
            .filter(|&x| x != *city)
            .collect();
        let total_dist = recursive_search(distances.clone(), cities_name_reduced, city);
        if total_dist < min_dist {
            min_dist = total_dist;
        }
    }
    min_dist
}

fn day09_part1(
    example: &(HashMap<(&str, &str), u32>, HashSet<&str>),
    input: &(HashMap<(&str, &str), u32>, HashSet<&str>),
) {
    // Exemple tests
    let (example_distances, example_cities_names) = example;
    let example_cities_names: Vec<&str> = example_cities_names.iter().cloned().collect();
    // println!(
    //     "Example part1 : {}",
    //     search(example_distances.clone(), example_cities_names.clone())
    // );
    assert_eq!(
        search(example_distances.clone(), example_cities_names.clone()),
        605
    );

    // Solve puzzle
    let (input_distances, input_cities_names) = input;
    let input_cities_names: Vec<&str> = input_cities_names.iter().cloned().collect();
    // println!(
    //     "Result part 1: {}",
    //     search(input_distances.clone(), input_cities_names.clone())
    // );
    assert_eq!(
        search(input_distances.clone(), input_cities_names.clone()),
        141
    );
    println!("> DAY09 - part 1: OK!");
}

fn day09_part2(
    _example: &(HashMap<(&str, &str), u32>, HashSet<&str>),
    _input: &(HashMap<(&str, &str), u32>, HashSet<&str>),
) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY09 - part 2: OK!");
}
