use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY07 -------");
    let example = fs::read_to_string("inputs/example_day07").expect("Unable to read input!");
    let mut example = parse(&example);
    let input = fs::read_to_string("inputs/input_day07").expect("Unable to read input!");
    let mut input = parse(&input);

    day07_part1(&example, &input);
    day07_part2(&mut example, &mut input);
}

#[derive(Debug, Clone)]
struct Node {
    weight: u32,
    stack_weight: u32,
    parent: Option<String>,
    children: Vec<String>,
}

fn parse(raw_input: &str) -> HashMap<String, Node> {
    // Create list of nodes with only children
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let re_no_children = Regex::new(r"^([a-z]+) \(([0-9]+)\)$").unwrap();
    for line in raw_input.lines() {
        let name: String;
        let weight: u32;
        let mut children = Vec::new();
        if let Some(matches) = re_no_children.captures(line) {
            name = matches[1].to_string();
            weight = matches[2].parse().unwrap();
        } else {
            let parts: Vec<&str> = line.split(" -> ").collect();
            if let Some(matches) = re_no_children.captures(parts[0]) {
                name = matches[1].to_string();
                weight = matches[2].parse().unwrap();
                children = parts[1].split(", ").map(|s| s.to_string()).collect();
            } else {
                unreachable!("Unknown start: {}", parts[0]);
            }
        }
        nodes.insert(
            name,
            Node {
                weight,
                stack_weight: 0,
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

fn get_stack_weight(nodes: &mut HashMap<String, Node>) {
    fn rec_stack_weight(name: String, node: Node, nodes: &mut HashMap<String, Node>) -> u32 {
        let w = node.weight;
        if node.children.is_empty() {
            nodes.get_mut(&name).unwrap().stack_weight = w;
            return w;
        }

        let mut sw = w;
        for child in node.children {
            let child_node = nodes.get(&child).unwrap().clone();
            sw += rec_stack_weight(child, child_node, nodes);
        }

        nodes.get_mut(&name).unwrap().stack_weight = sw;
        sw
    }

    let bottom = find_bottom(nodes);
    let bottom_node = nodes.get(&bottom).unwrap().clone();
    rec_stack_weight(bottom, bottom_node, nodes);
}

fn balance_tower(nodes: &HashMap<String, Node>) -> u32 {
    fn bad_node(children: Vec<String>, nodes: &HashMap<String, Node>) -> Option<String> {
        let mut stack_weights = vec![];
        for child in children.clone() {
            let child_node = nodes.get(&child).unwrap();
            stack_weights.push(child_node.stack_weight);
        }
        if stack_weights.iter().all(|&sw| sw == stack_weights[0]) {
            // All values are identical, no bad node
            return None;
        }
        // Else, find bad value (the one with count = to 1)
        let mut bad_stack_weight = 0;
        for sw in &stack_weights {
            if stack_weights.iter().filter(|&sw2| *sw2 == *sw).count() == 1 {
                bad_stack_weight = *sw;
                break;
            }
        }
        // Then find name of the bad node
        for child in children {
            let child_node = nodes.get(&child).unwrap();
            if child_node.stack_weight == bad_stack_weight {
                return Some(child.clone());
            }
        }
        unreachable!();
    }

    fn rec_balance_tower(name: String, node: Node, nodes: &HashMap<String, Node>) -> u32 {
        let bad_node_name = bad_node(node.children, nodes);
        match bad_node_name {
            None => {
                // Found last bad node !
                // Get good_stack_weight by finding a brother node
                let parent_node = nodes.get(&node.parent.clone().unwrap()).unwrap();
                let brotherhood = parent_node.children.clone();
                let mut good_stack_weight = 0;
                for brother in brotherhood {
                    if brother == name {
                        continue;
                    }
                    let brother_node = nodes.get(&brother).unwrap();
                    good_stack_weight = brother_node.stack_weight;
                    break;
                }
                // Then compute needed_weight
                good_stack_weight - (node.stack_weight - node.weight)
            }
            Some(child_name) => {
                let child_node = nodes.get(&child_name).unwrap();
                rec_balance_tower(child_name, child_node.clone(), nodes)
            }
        }
    }

    let bottom = find_bottom(nodes);
    let bottom_node = nodes.get(&bottom).unwrap().clone();
    rec_balance_tower(bottom, bottom_node, nodes)
}

fn day07_part2(example: &mut HashMap<String, Node>, input: &mut HashMap<String, Node>) {
    // Exemple tests
    get_stack_weight(example);
    assert_eq!(example.get("ugml").unwrap().stack_weight, 251);
    assert_eq!(example.get("padx").unwrap().stack_weight, 243);
    assert_eq!(example.get("fwft").unwrap().stack_weight, 243);
    assert_eq!(example.get("gyxo").unwrap().stack_weight, 61);
    assert_eq!(example.get("ebii").unwrap().stack_weight, 61);
    assert_eq!(example.get("jptl").unwrap().stack_weight, 61);
    assert_eq!(example.get("pbga").unwrap().stack_weight, 66);
    assert_eq!(example.get("havc").unwrap().stack_weight, 66);
    assert_eq!(example.get("qoyq").unwrap().stack_weight, 66);
    assert_eq!(example.get("ktlj").unwrap().stack_weight, 57);
    assert_eq!(example.get("cntj").unwrap().stack_weight, 57);
    assert_eq!(example.get("xhth").unwrap().stack_weight, 57);
    assert_eq!(example.get("tknk").unwrap().stack_weight, 778);
    assert_eq!(balance_tower(example), 60);

    // Solve puzzle
    get_stack_weight(input);
    let res = balance_tower(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 1526);
    println!("> DAY07 - part 2: OK!");
}
