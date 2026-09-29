use std::fs;
use regex::Regex;
use std::collections::HashMap;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY06 -------");
    let example = fs::read_to_string("inputs/example_day06").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day06").expect("Unable to read input!");
    let input = parse(&input);

    day06_part1(&example, &input);
    day06_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<(i32, i32)> {
    let mut coordinates = vec![];
    let r = Regex::new(r"^([0-9]+), ([0-9]+)$").unwrap();
    for line in raw_input.lines() {
        let matches = r.captures(line).unwrap();
        let x = matches[1].parse().unwrap();
        let y = matches[2].parse().unwrap();
        coordinates.push((x, y));
    }
    coordinates
}

fn manhattan_dist(c1: (i32, i32), c2: (i32, i32)) -> i32 {
    (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs()
}

fn borders(coordinates_list: &[(i32, i32)]) -> (i32, i32, i32, i32) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    for other in coordinates_list {
        let other_x = other.0;
        let other_y = other.1;
        if other_x < x_min {
            x_min = other_x;
        }
        if other_y < y_min {
            y_min = other_y;
        }
        if other_x > x_max {
            x_max = other_x;
        }
        if other_y > y_max {
            y_max = other_y;
        }
    }
    (x_min, x_max, y_min, y_max)
}

fn exclude_coordinate(c: (i32, i32), borders: (i32, i32, i32, i32)) -> bool {
    let (x_min, x_max, y_min, y_max) = borders;
    if c.0 == x_min || c.0 == x_max || c.1 == y_min || c.1 == y_max {
        return true;
    }
    false
}

fn closest_coordinate(c: (i32, i32), coordinates_list: &[(i32, i32)]) -> (i32, i32) {
    let mut closest = (i32::MAX, i32::MAX);
    let mut closest_dist = i32::MAX;
    let mut twice = false;
    for reference in coordinates_list {
        let d = manhattan_dist(c, *reference);
        if d == closest_dist && *reference != closest {
            twice = true;
        }
        if d < closest_dist {
            closest_dist = d;
            closest = *reference;
            twice = false;
        }
    }
    if twice {
        return (i32::MIN, i32::MIN);
    }
    closest
}

fn largest_area_not_infinite(coordinates_list: &[(i32, i32)]) -> i32 {
    let b = borders(coordinates_list);
    let (x_min, x_max, y_min, y_max) = b;
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    for x in (x_min - 100)..(x_max + 100) {
        for y in (y_min - 100)..(y_max + 100) {
            let closest = closest_coordinate((x, y), coordinates_list);
            let count = counts.entry(closest).or_insert(0);
            *count += 1;
        }
    }

    let mut biggest_area = i32::MIN;
    for (c, nb) in counts {
        if exclude_coordinate(c, b) {
            continue;
        }
        if c == (i32::MIN, i32::MIN) {
            continue;
        }
        if nb > biggest_area {
            biggest_area = nb;
        }
    }
    biggest_area
}

fn day06_part1(example: &[(i32, i32)], input: &[(i32, i32)]) {
    // Exemple tests
    assert_eq!(manhattan_dist((5, 5), (8, 9)), 7);
    let example_borders = borders(example);
    assert!(exclude_coordinate((1, 1), example_borders));
    assert!(exclude_coordinate((1, 6), example_borders));
    assert!(exclude_coordinate((8, 3), example_borders));
    assert!(!exclude_coordinate((3, 4), example_borders));
    assert!(!exclude_coordinate((5, 5), example_borders));
    assert!(exclude_coordinate((8, 9), example_borders));
    assert_eq!(closest_coordinate((1, 1), example), (1, 1));
    assert_eq!(closest_coordinate((1, 6), example), (1, 6));
    assert_eq!(closest_coordinate((8, 3), example), (8, 3));
    assert_eq!(closest_coordinate((3, 4), example), (3, 4));
    assert_eq!(closest_coordinate((5, 5), example), (5, 5));
    assert_eq!(closest_coordinate((8, 9), example), (8, 9));
    assert_eq!(closest_coordinate((4, 4), example), (3, 4));
    assert_eq!(closest_coordinate((4, 4), example), (3, 4));
    assert_eq!(manhattan_dist((0, 4), (3, 4)), 3);
    assert_eq!(manhattan_dist((0, 4), (1, 6)), 3);
    assert_eq!(closest_coordinate((0, 4), example), (i32::MIN, i32::MIN));
    assert_eq!(largest_area_not_infinite(example), 17);
    println!("Example OK");

    // Solve puzzle
    let res = largest_area_not_infinite(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, ); // 3682 too high
    // println!("> DAY06 - part 1: OK!");
}

fn day06_part2(_example: &[(i32, i32)], _input: &[(i32, i32)]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY06 - part 2: OK!");
}
