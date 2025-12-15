use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY12 -------");
    let example = fs::read_to_string("inputs/example_day12").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day12").expect("Unable to read input!");
    let input = parse(&input);

    day12(example, input);
}

#[allow(clippy::type_complexity)]
fn parse(raw_input: &str) -> (HashMap<usize, Vec<&str>>, Vec<(usize, usize, Vec<usize>)>) {
    let mut shapes = HashMap::new();
    let mut regions = vec![];
    // let mut is_shape = false;
    let mut index = 0;
    let mut shape = vec![];
    let r1 = Regex::new(r"^([0-5]):$").unwrap();
    let r2 =
        Regex::new(r"^([0-9]*)x([0-9]*): ([0-9]*) ([0-9]*) ([0-9]*) ([0-9]*) ([0-9]*) ([0-9]*)$")
            .unwrap();
    for line in raw_input.lines() {
        if let Some(matches) = r1.captures(line) {
            // is_shape = true;
            index = matches[1].parse::<usize>().unwrap();
            continue;
        }

        if line.is_empty() {
            shapes.insert(index, shape.clone());
            shape.clear();
            // is_shape = false;
            continue;
        }

        if let Some(matches) = r2.captures(line) {
            let mut quantities = vec![];
            for i in 3..=8 {
                quantities.push(matches[i].parse().unwrap());
            }
            regions.push((
                matches[1].parse().unwrap(),
                matches[2].parse().unwrap(),
                quantities.clone(),
            ));
            continue;
        }

        shape.push(line);
    }
    (shapes, regions)
}

#[allow(clippy::type_complexity)]
fn heuristic(input: (HashMap<usize, Vec<&str>>, Vec<(usize, usize, Vec<usize>)>)) {
    let shapes = input.0;
    let regions = input.1;
    let nb_regions_total = regions.len();
    println!("Total nb of regions: {nb_regions_total}");

    // 1. Find number of regions that are trivially impossible:
    // Even with a perfect arrangment, it cannot fit because there are too many
    // presents for the region.

    // 2. Find number of regions that are trivially possible:
    // Even with shapes all full 3 by 3, it fits so any arrangment is ok.

    let mut nb_presents_by_shape = HashMap::new();
    for (shape_index, shape) in shapes {
        let mut nb = 0usize;
        for line in shape {
            for c in line.chars() {
                if c == '#' {
                    nb += 1;
                }
            }
        }
        nb_presents_by_shape.insert(shape_index, nb);
    }

    let mut nb_trivially_impossible = 0usize;
    let mut nb_trivially_possible = 0usize;
    let mut others = 0usize;
    for (wide, long, quantities) in regions {
        let max_size = wide * long;
        let mut nb_necessary_places = 0usize;
        let mut nb_max_places_taken = 0usize;
        for (i, &quantities_shape_i) in quantities.iter().enumerate() {
            nb_necessary_places += quantities_shape_i * nb_presents_by_shape.get(&i).unwrap();
            nb_max_places_taken += quantities_shape_i * 9;
        }

        if max_size < nb_necessary_places {
            nb_trivially_impossible += 1;
        } else if nb_max_places_taken < max_size {
            nb_trivially_possible += 1;
        } else {
            others += 1;
        }
    }

    println!(
        "{nb_trivially_impossible} are trivially impossible to solve, even with best arrangment"
    );
    println!("{nb_trivially_possible} are trivially possible to solve, even with worst arrangment");
    println!("That leaves us {others} cases to deal with");
    println!(
        "In the best case, there are {} regions that can fit all!",
        nb_regions_total - nb_trivially_impossible
    );
}

#[allow(clippy::type_complexity)]
fn day12(
    example: (HashMap<usize, Vec<&str>>, Vec<(usize, usize, Vec<usize>)>),
    input: (HashMap<usize, Vec<&str>>, Vec<(usize, usize, Vec<usize>)>),
) {
    println!("Example:");
    // Exemple tests
    heuristic(example);

    // Solve puzzle
    println!("\nInput:");
    heuristic(input); // 463 is too low
    println!("\n499 (best case scenario) is the answer !");
    println!("> DAY12: OK!");
}
