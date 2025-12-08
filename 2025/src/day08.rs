use std::{collections::HashMap, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY08 -------");
    let example = fs::read_to_string("inputs/example_day08").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day08").expect("Unable to read input!");
    let input = parse(&input);

    day08_part1(&example, &input);
    day08_part2(&example, &input);
}

fn parse(raw_input: &str) -> HashMap<usize, (i64, i64, i64)> {
    let mut boxes = HashMap::new();
    for (id, line) in raw_input.lines().enumerate() {
        let raw_coordinates: Vec<&str> = line.split(",").collect();
        boxes.insert(
            id,
            (
                raw_coordinates[0].parse().unwrap(),
                raw_coordinates[1].parse().unwrap(),
                raw_coordinates[2].parse().unwrap(),
            ),
        );
    }
    boxes
}

fn squared_distance(box1: (i64, i64, i64), box2: (i64, i64, i64)) -> i64 {
    (box1.0 - box2.0) * (box1.0 - box2.0)
        + (box1.1 - box2.1) * (box1.1 - box2.1)
        + (box1.2 - box2.2) * (box1.2 - box2.2)
}

fn compute_all_distances(boxes: &HashMap<usize, (i64, i64, i64)>) -> Vec<(i64, (usize, usize))> {
    let mut distances = vec![];
    let nb_boxes = boxes.len();
    for i in 0..nb_boxes {
        let box1 = *boxes.get(&i).unwrap();
        for j in i + 1..nb_boxes {
            let box2 = *boxes.get(&j).unwrap();
            distances.push((squared_distance(box1, box2), (i, j)));
        }
    }
    distances.sort();
    distances
}

fn solve_part1(
    boxes: &HashMap<usize, (i64, i64, i64)>,
    n: usize,
    nb_biggest_groups: usize,
) -> usize {
    let nb_boxes = boxes.len();

    // Compute n shortest distances
    let distances = compute_all_distances(boxes);
    let distances = distances[..n].to_vec();

    // Create hashmap where each box id is alone in its own junction
    let mut junctions = HashMap::new();
    for id in 0..nb_boxes {
        junctions.insert(id, id);
    }

    // Apply junctions
    for (_, pair) in distances {
        // Get group of the first id, to keep
        let group_to_keep = *junctions.get(&pair.0).unwrap();

        // Get group of second id, to change
        let group_to_change = *junctions.get(&pair.1).unwrap();

        // Then change all groups that need to be changed
        let mut ids_to_change = vec![];
        for id in 0..nb_boxes {
            if *junctions.get(&id).unwrap() == group_to_change {
                ids_to_change.push(id);
            }
        }
        for id in ids_to_change {
            junctions.insert(id, group_to_keep);
        }
    }

    // Get groups sizes
    let mut groups = HashMap::new();
    for (_, ref_to) in junctions {
        let count = groups.entry(ref_to).or_insert(0);
        *count += 1;
    }

    // Find nb_groups biggest groups and multiply nb boxes
    let mut res = 1;
    for _ in 0..nb_biggest_groups {
        let (biggest_group, nb_boxes) = groups.iter().max_by_key(|(_, v)| *v).unwrap();
        res *= nb_boxes;
        let key = *biggest_group;
        groups.remove(&key);
    }

    res
}

fn day08_part1(example: &HashMap<usize, (i64, i64, i64)>, input: &HashMap<usize, (i64, i64, i64)>) {
    // Exemple tests
    assert_eq!(squared_distance((162, 817, 812), (425, 690, 689)), 100427);
    assert_eq!(solve_part1(example, 10, 3), 40);

    // Solve puzzle
    let res = solve_part1(input, 1000, 3);
    println!("Result part 1: {res}");
    assert_eq!(res, 131580);
    println!("> DAY08 - part 1: OK!");
}

fn solve_part2(boxes: &HashMap<usize, (i64, i64, i64)>) -> i64 {
    let nb_boxes = boxes.len();

    // Compute all shortest distances
    let distances = compute_all_distances(boxes);

    // Create hashmap where each box id is alone in its own junction
    let mut junctions = HashMap::new();
    for id in 0..nb_boxes {
        junctions.insert(id, id);
    }

    // Apply junctions until ont group
    'main_loop: for (_, pair) in distances {
        // Get group of the first id, to keep
        let group_to_keep = *junctions.get(&pair.0).unwrap();

        // Get group of second id, to change
        let group_to_change = *junctions.get(&pair.1).unwrap();

        // Then change all groups that need to be changed
        let mut ids_to_change = vec![];
        for id in 0..nb_boxes {
            if *junctions.get(&id).unwrap() == group_to_change {
                ids_to_change.push(id);
            }
        }
        for id in ids_to_change {
            junctions.insert(id, group_to_keep);
        }

        // Check if there is only one group
        let ref_group = *junctions.get(&0).unwrap();
        for id in 1..nb_boxes {
            if *junctions.get(&id).unwrap() != ref_group {
                continue 'main_loop;
            }
        }
        // If we are here, there is only one group!
        let box1 = *boxes.get(&pair.0).unwrap();
        let box2 = *boxes.get(&pair.1).unwrap();
        return box1.0 * box2.0;
    }
    unreachable!();
}

fn day08_part2(example: &HashMap<usize, (i64, i64, i64)>, input: &HashMap<usize, (i64, i64, i64)>) {
    // Exemple tests
    assert_eq!(solve_part2(example), 25272);

    // Solve puzzle
    let res = solve_part2(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 6844224);
    println!("> DAY08 - part 2: OK!");
}
