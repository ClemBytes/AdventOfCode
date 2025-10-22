use std::{collections::{HashMap, HashSet, VecDeque}, fs};

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

fn balance_tower(nodes: &mut HashMap<String, Node>) -> u32 {
    // First, fill stack_weights, starting from leaves
    // Then, for each leave, the stack_weight is equal to its own weight + its chidren' stack_weights
    // And I push back it's parent, keeping track of already added nodes
    let mut nodes_to_compute = VecDeque::new();
    let mut added_nodes = HashSet::new();
    for (name, node) in &mut *nodes {
        if node.children.is_empty() {
            nodes_to_compute.push_back(name.clone());
            added_nodes.insert(name.clone());
        }
    }
    while let Some(name) = nodes_to_compute.pop_front() {
        // Get children' stack_weights
        let mut children_stack_weight = 0;
        for child in &nodes.get(&name).unwrap().children {
            children_stack_weight += &nodes.get(child).unwrap().stack_weight;
        }
        // Update current stack_weight
        let current_weight = nodes.get(&name).unwrap().weight;
        nodes.get_mut(&name).unwrap().stack_weight = current_weight + children_stack_weight;
        // Add parent to VecDeque if not already added
        let parent = &nodes.get(&name).unwrap().parent;
        if let Some(parent_name) = parent && !added_nodes.contains(parent_name) {
            nodes_to_compute.push_back(parent_name.clone());
            added_nodes.insert(parent_name.clone());
        }
    }

    // println!("{nodes:#?}"); // Seems OK here
    // Now, search for wrong weight. TODO: RESTART THIS PART
    let nodes_clone = nodes.clone();
    for (name, node) in nodes {
        if node.children.is_empty() {
            continue;
        }
        // Get children stack_weights
        let mut children_stack_weights = HashMap::new();
        for child in &node.children {
            let w = nodes_clone.get(child).unwrap().weight;
            let sw = nodes_clone.get(child).unwrap().stack_weight;
            // children_stack_weights.insert(sw);
            let count = children_stack_weights.entry(sw).or_insert((w, 0));
            count.1 += 1;
        }
        // Check if all equals
        if children_stack_weights.len() == 1 {
            continue;
        }
        // If not, find good value
        let mut stack_weight_should_be = 0;
        if let Some((&sw, &(_w, _count))) = children_stack_weights.iter().max_by_key(|(_sw, (_w, count))| count) {
            stack_weight_should_be = sw;
        }
        if let Some((&sw, &(w, _count))) = children_stack_weights.iter().min_by_key(|(_sw, (_w, count))| count) {
            println!("children_stack_weights: {children_stack_weights:?}");
            println!("parent name: {name} | weight: {w} | stack_weight: {sw} | stack_weight_should_be: {stack_weight_should_be}");
            return stack_weight_should_be - (sw - w);
        }
    } // Ouh là. J'ai pas le même résultat à chaque fois...
    unreachable!("No wrong weight found");
}

fn day07_part2(example: &mut HashMap<String, Node>, input: &mut HashMap<String, Node>) {
    // En gros, il faut que je calcule les poids de chaque sous-stack (donc le noeud + ceux qu'il porte récursivement)
    // Une fois que c'est fait, je parcours mon arbre en regardant pour chaque noeud lequel n'est pas égal à ses voisins.

    // Exemple tests
    assert_eq!(balance_tower(example), 60);
    println!("Example OK");

    // Solve puzzle
    let res = balance_tower(input);
    println!("Result part 2: {res}"); // 365 too low, 1104 also, 9182 too high, 2012 not good
    // assert_eq!(res, );
    // println!("> DAY07 - part 2: OK!");
}
