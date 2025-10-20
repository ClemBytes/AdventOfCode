use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY07 -------");
    let example = fs::read_to_string("inputs/example_day07").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day07").expect("Unable to read input!");
    let input = parse(&input);

    day07_part1(&example, &input);
    day07_part2(&example, &input);
}

#[derive(Debug, Clone)]
struct Node {
    size: u32,
    parent: Option<String>,
    children: Vec<String>,
}

fn parse(raw_input: &str) -> HashMap<String, Node> {
    // Create list of nodes with only children
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let re_no_children = Regex::new(r"^([a-z]+) \(([0-9]+)\)$").unwrap();
    for line in raw_input.lines() {
        let name: String;
        let size: u32;
        let mut children = Vec::new();
        if let Some(matches) = re_no_children.captures(line) {
            name = matches[1].to_string();
            size = matches[2].parse().unwrap();
        } else {
            let parts: Vec<&str> = line.split(" -> ").collect();
            if let Some(matches) = re_no_children.captures(parts[0]) {
                name = matches[1].to_string();
                size = matches[2].parse().unwrap();
                children = parts[1].split(", ").map(|s| s.to_string()).collect();
            } else {
                unreachable!("Unknown start: {}", parts[0]);
            }
        }
        nodes.insert(
            name,
            Node {
                size,
                parent: None,
                children,
            },
        );
    }
    // Now add parents
    for (name, node) in nodes.clone() {
        for child in node.children {
            if let Some(child_node) = nodes.get_mut(&child) {
                child_node.parent = Some(name.clone());
            }
        }
    }
    nodes
}

fn find_bottom(nodes: &HashMap<String, Node>) -> String {
    for (name, child) in nodes {
        if child.parent.is_none() {
            return name.clone();
        }
    }
    unreachable!("Bottom program not found!");
}

fn day07_part1(example: &HashMap<String, Node>, input: &HashMap<String, Node>) {
    // Exemple tests
    assert_eq!(find_bottom(example), "tknk");

    // Solve puzzle
    let res = find_bottom(input);
    println!("Result part 1: {res}");
    assert_eq!(res, "gynfwly");
    println!("> DAY07 - part 1: OK!");
}

fn day07_part2(_example: &HashMap<String, Node>, _input: &HashMap<String, Node>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY07 - part 2: OK!");
}
