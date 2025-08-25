use std::{collections::HashSet, vec};

#[test]
fn test() {
    run();
}

pub fn run() {
    /*
    0 H
    1 L
    2 P
    3 T
    4 p
    5 R
    6 C
     */
    println!("------- DAY11 -------");
    let mut example = Area {
        elevator: 0,
        floors: [
            vec![Object::Microchip(0), Object::Microchip(1)],
            vec![Object::Generator(0)],
            vec![Object::Generator(1)],
            vec![],
        ],
    };
    example.floors[0].sort();
    example.floors[1].sort();
    example.floors[2].sort();
    example.floors[3].sort();
    let mut input = Area {
        elevator: 0,
        floors: [
            vec![
                Object::Generator(2),
                Object::Generator(3),
                Object::Microchip(3),
                Object::Generator(4),
                Object::Generator(5),
                Object::Microchip(5),
                Object::Generator(6),
                Object::Microchip(6),
            ],
            vec![Object::Microchip(2), Object::Microchip(4)],
            vec![],
            vec![],
        ],
    };
    input.floors[0].sort();
    input.floors[1].sort();
    input.floors[2].sort();
    input.floors[3].sort();

    day11_part1(example.clone(), input.clone());
    day11_part2(input);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Object {
    Microchip(u8),
    Generator(u8),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Area {
    elevator: usize,
    floors: [Vec<Object>; 4],
}

fn check_floor(floor: &Vec<Object>) -> bool {
    let mut microchips: u32 = 0;
    let mut generators: u32 = 0;
    for object in floor {
        match object {
            Object::Microchip(element) => {
                microchips |= 2_u32.pow((*element).into());
            }
            Object::Generator(element) => {
                generators |= 2_u32.pow((*element).into());
            }
        }
    }
    generators == 0 || (microchips & !generators) == 0
}

/*
fn temp() {
    let m = HashSet::from(['a', 'b']);
    let g = HashSet::from(['a', 'c']);
    let cond = m.difference(&g).next().is_none() || g.difference(&m).next().is_none();
    println!("m: {:?}\ng: {:?}\nm.diff(g): {:?}\ng.diff(m): {:?}\ncondition: {}\n", m, g, m.difference(&g), g.difference(&m), cond);
    let m = HashSet::from(['a', 'b']);
    let g = HashSet::from(['a', 'b', 'c']);
    let cond = m.difference(&g).next().is_none() || g.difference(&m).next().is_none();
    println!("m: {:?}\ng: {:?}\nm.diff(g): {:?}\ng.diff(m): {:?}\ncondition: {}\n", m, g, m.difference(&g), g.difference(&m), cond);
    let m = HashSet::from(['a', 'b', 'c']);
    let g = HashSet::from(['a', 'b']);
    let cond = m.difference(&g).next().is_none() || g.difference(&m).next().is_none();
    println!("m: {:?}\ng: {:?}\nm.diff(g): {:?}\ng.diff(m): {:?}\ncondition: {}\n", m, g, m.difference(&g), g.difference(&m), cond);
}
*/

fn move_floor(current_area: &Area, q: &mut Vec<Area>, from: usize, to: usize, visited: &mut HashSet<Area>) {
    // Max elevator capacity: 2 objects
    // I can move :
    // - 1 object
    // - 2 microchips
    // - 2 generator
    // - 1 microchip + 1 generator ONLY if they are compatible

    // Try moving only one object:
    for (i, &object) in current_area.floors[from].iter().enumerate() {
        let mut next_area = current_area.clone();
        next_area.floors[from].remove(i);
        next_area.floors[to].push(object);
        next_area.floors[to].sort();

        // For each microchip at a given floor, I need to check if there is no generator that can fry it
        if !check_floor(&next_area.floors[0])
            || !check_floor(&next_area.floors[1])
            || !check_floor(&next_area.floors[2])
            || !check_floor(&next_area.floors[3])
        {
            continue;
        }

        if !visited.contains(&next_area) {
            next_area.elevator = to;
            q.push(next_area.clone());
            visited.insert(next_area);
        }
    }

    // Try moving 2 objects:
    for i in 0..current_area.floors[from].len() {
        for j in i + 1..current_area.floors[from].len() {
            let mut next_area = current_area.clone();
            let second_object = next_area.floors[from].remove(j);
            let first_object = next_area.floors[from].remove(i);
            next_area.floors[to].push(first_object);
            next_area.floors[to].push(second_object);
            next_area.floors[to].sort();

            // For each microchip at a given floor, I need to check if there is no generator that can fry it
            if !check_floor(&next_area.floors[0])
                || !check_floor(&next_area.floors[1])
                || !check_floor(&next_area.floors[2])
                || !check_floor(&next_area.floors[3])
            {
                continue;
            }

            if !visited.contains(&next_area) {
                next_area.elevator = to;
                q.push(next_area.clone());
                visited.insert(next_area);
            }
        }
    }
}

fn expand_front(front: &mut Vec<Area>, visited_same_front: &mut HashSet<Area>) -> Vec<Area> {
    let mut next_front = vec![];
    while let Some(current) = front.pop() {
        // println!("front: {}", front.len());
        // If no object on elevator floor => impossible, continue
        for f in 0..4 {
            assert!(!(current.elevator == f && current.floors[f].is_empty()));
        }

        // Create next front
        match current.elevator {
            0 => {
                // Move from for floor 0 to 1:
                move_floor(&current, &mut next_front, 0, 1, visited_same_front);
            }
            1 => {
                // Move from for floor 1 to 0:
                move_floor(&current, &mut next_front, 1, 0, visited_same_front);
                // Move from for floor 1 to 2:
                move_floor(&current, &mut next_front, 1, 2, visited_same_front);
            }
            2 => {
                // Move from for floor 2 to 1:
                move_floor(&current, &mut next_front, 2, 1, visited_same_front);
                // Move from for floor 2 to 3:
                move_floor(&current, &mut next_front, 2, 3, visited_same_front);
            }
            3 => {
                // Move from for floor 3 to 2:
                move_floor(&current, &mut next_front, 3, 2, visited_same_front);
            }
            other => panic!("Unknown elevator floor: {other}"),
        }
    }
    next_front
}

fn find_minimum_steps(input: Area) -> u32 {
    // Init
    let mut visited_from_start: HashSet<Area> = HashSet::new();
    let mut front_from_start = Vec::new();
    let mut end_area = Area {
        elevator: 3,
        floors: [vec![], vec![], vec![], vec![]],
    };
    for f in &input.floors {
        for object in f {
            end_area.floors[3].push(*object);
        }
    }
    let mut visited_from_end: HashSet<Area> = HashSet::new();
    let mut front_from_end = Vec::new();

    // Start
    front_from_start.push(input.clone());
    front_from_end.push(end_area.clone());
    let mut nb_steps_from_start = 0;
    let mut nb_steps_from_end = 0;
    loop {
        // Go one step further from start
        front_from_start = expand_front(&mut front_from_start, &mut visited_from_start);
        // Check if current area from start has been seen from end
        if visited_from_end
            .intersection(&visited_from_start)
            .next()
            .is_some()
        {
            return nb_steps_from_start + nb_steps_from_end - 1;
        }
        nb_steps_from_start += 1;

        // Go one step before from end
        front_from_end = expand_front(&mut front_from_end, &mut visited_from_end);
        // Check if current area from end has been seen from start
        if visited_from_end
            .intersection(&visited_from_start)
            .next()
            .is_some()
        {
            return nb_steps_from_start + nb_steps_from_end - 1;
        }
        nb_steps_from_end += 1;
    }
}

fn day11_part1(example: Area, input: Area) {
    // Exemple tests
    // temp();
    let res = find_minimum_steps(example);
    assert_eq!(res, 11);

    // Solve puzzle
    let res = find_minimum_steps(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 47);
    println!("> DAY11 - part 1: OK!");
}

fn day11_part2(mut input: Area) {
    // Add additionnal elements: 7 D & 8 E
    input.floors[0].push(Object::Generator(7));
    input.floors[0].push(Object::Microchip(7));
    input.floors[0].push(Object::Generator(8));
    input.floors[0].push(Object::Microchip(8));
    input.floors[0].sort();
    // Solve puzzle
    let res = find_minimum_steps(input);
    println!("Result part 2: {res}");
    // assert_eq!(res, ); // 75 not OK
    // println!("> DAY11 - part 2: OK!");
}
