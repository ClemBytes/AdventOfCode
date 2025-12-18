use std::{collections::HashSet, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY24 -------");
    let example = fs::read_to_string("inputs/example_day24").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day24").expect("Unable to read input!");
    let input = parse(&input);

    day24_part1(&example, &input);
    day24_part2(&example, &input);
}

fn parse(raw_input: &str) -> HashSet<(usize, usize)> {
    let mut components = HashSet::new();
    for line in raw_input.lines() {
        let (a, b) = line.split_once("/").unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        if components.contains(&(a, b)) {
            println!("({a}, {b}) already in set!");
        }
        components.insert((a, b));
    }
    components
}

fn next_possible_components(
    bridge: &[(usize, usize)],
    components: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut possible_components = vec![];
    let current = bridge[bridge.len() - 1];
    // Always use identity if possible:
    let identity0 = (current.0, current.0);
    if components.contains(&identity0) && !bridge.contains(&identity0) {
        possible_components.push(identity0);
        return possible_components;
    }
    let identity1 = (current.1, current.1);
    if components.contains(&identity1) && !bridge.contains(&identity1) {
        possible_components.push(identity1);
        return possible_components;
    }

    for &next in components {
        if (!bridge.contains(&next))
            && (next.0 != 0 && next.1 != 0)
            && (current.0 == next.0
                || current.0 == next.1
                || current.1 == next.0
                || current.1 == next.1)
        {
            possible_components.push(next);
        }
    }
    possible_components
}

fn compute_strength(bridge: &[(usize, usize)]) -> usize {
    let mut strength = 0;
    for &component in bridge {
        strength += component.0 + component.1;
    }
    strength
}

// Theorically OK, but too long…
fn find_strongest_bridge(components: &HashSet<(usize, usize)>) -> usize {
    let mut max_strength = 0;
    let mut pile = vec![];
    for &comp in components {
        if comp.0 == 0 {
            pile.push(vec![comp]);
        }
    }
    // println!("Start pile: {pile:?}");

    while let Some(current_bridge) = pile.pop() {
        // println!("{current_bridge:?}");
        let possible_components = next_possible_components(&current_bridge, components);
        // println!("possible_components: {possible_components:?}");
        if possible_components.is_empty() {
            let strength = compute_strength(&current_bridge);
            // println!("Bridge: {current_bridge:?}");
            // println!("Strength: {strength}\n");
            if strength > max_strength {
                max_strength = strength;
            }
            continue;
        }

        // Else (we can add other components)
        for next_component in possible_components {
            let mut next_bridge = current_bridge.clone();
            next_bridge.push(next_component);
            pile.push(next_bridge);
        }
    }

    max_strength
}

fn day24_part1(example: &HashSet<(usize, usize)>, input: &HashSet<(usize, usize)>) {
    // Exemple tests
    assert_eq!(find_strongest_bridge(example), 31);
    println!("Example OK");

    // Solve puzzle
    let res = find_strongest_bridge(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY24 - part 1: OK!");
}

fn day24_part2(_example: &HashSet<(usize, usize)>, _input: &HashSet<(usize, usize)>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY24 - part 2: OK!");
}
