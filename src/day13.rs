use std::collections::{HashSet, VecDeque};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY13 -------");
    let example = 10;
    let input = 1350;

    day13_part1(example, input);
    day13_part2(input);
}

#[derive(Debug, PartialEq)]
enum LocationType {
    Wall,
    OpenSpace,
}

fn get_location_kind(x: i32, y: i32, designer_fav: i32) -> LocationType {
    assert!(!(x < 0 || y < 0));
    let mut code = x * x + 3 * x + 2 * x * y + y + y * y;
    code += designer_fav;
    match code.count_ones() % 2 {
        0 => LocationType::OpenSpace,
        1 => LocationType::Wall,
        _ => unreachable!(),
    }
}

fn reach(start: (i32, i32), end: (i32, i32), designer_fav: i32) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((0, start));
    let mut visited = HashSet::new();
    while let Some((nb_steps, current)) = q.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        if current == end {
            return nb_steps;
        }

        for (x, y) in [
            (current.0 - 1, current.1),
            (current.0 + 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
        ] {
            if x >= 0 && y >= 0 && get_location_kind(x, y, designer_fav) == LocationType::OpenSpace
            {
                q.push_back((nb_steps + 1, (x, y)));
            }
        }
    }
    unreachable!()
}

fn get_nb_locations(start: (i32, i32), nb_steps: i32, designer_fav: i32) -> usize {
    let mut q = VecDeque::new();
    q.push_back((0, start));
    let mut visited = HashSet::new();
    while let Some((nb_steps_current, current)) = q.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        if nb_steps_current == nb_steps {
            return visited.len();
        }

        for (x, y) in [
            (current.0 - 1, current.1),
            (current.0 + 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
        ] {
            if x >= 0 && y >= 0 && get_location_kind(x, y, designer_fav) == LocationType::OpenSpace
            {
                q.push_back((nb_steps_current + 1, (x, y)));
            }
        }
    }
    unreachable!()
}

fn day13_part1(example: i32, input: i32) {
    // Exemple tests
    let res = reach((1, 1), (7, 4), example);
    assert_eq!(res, 11);

    // Solve puzzle
    let res = reach((1, 1), (31, 39), input);
    println!("Result part 1: {res}");
    assert_eq!(res, 92);
    println!("> DAY13 - part 1: OK!");
}

fn day13_part2(input: i32) {
    // Solve puzzle
    let res = get_nb_locations((1, 1), 50, input);
    println!("Result part 2: {res}");
    assert_eq!(res, 124);
    println!("> DAY13 - part 2: OK!");
}
