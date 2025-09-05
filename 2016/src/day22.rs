use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY22 -------");
    let example = fs::read_to_string("inputs/example_day22").expect("Unable to read input!");
    let example = Node::from_str(&example);
    let input = fs::read_to_string("inputs/input_day22").expect("Unable to read input!");
    let input = Node::from_str(&input);

    day22_part1(&input);
    day22_part2(&example, &input);
}

#[derive(Debug, Clone, Copy)]
struct Node {
    size: u32,
    used: u32,
}

impl Node {
    fn from_str(raw_input: &str) -> Vec<Vec<Self>> {
        // Important note: the data in input is arranged in order…
        // x0-y0, x0-y1, x0-y2 …, x1-y0, x1-y1, x1-y2…
        let mut nodes = vec![];
        let mut new_line = vec![];
        let re = Regex::new(
            r"^\/dev\/grid\/node-x([0-9]+)-y([0-9]+) +([0-9]+)T +([0-9]+)T +[0-9]+T +[0-9]+%$",
        )
        .unwrap();
        let lines = raw_input.lines();
        for line in lines {
            if let Some(matches) = re.captures(line) {
                let x: u32 = matches[1].parse().unwrap();
                let y: u32 = matches[2].parse().unwrap();
                if y == 0 && x != 0 {
                    nodes.push(new_line.clone());
                    new_line.clear();
                }
                new_line.push(Node {
                    size: matches[3].parse().unwrap(),
                    used: matches[4].parse().unwrap(),
                });
            } else {
                println!("Ignore line: {line}");
            }
        }
        nodes.push(new_line);
        nodes
    }

    fn is_empty(&self) -> bool {
        self.used == 0
    }

    fn fits_in(&self, b: Node) -> bool {
        self.used <= (b.size - b.used)
    }

    fn viable_with(&self, b: Node) -> bool {
        (!self.is_empty()) && self.fits_in(b)
    }
}

fn nb_viable_pairs(input: &[Vec<Node>]) -> u32 {
    let mut nb = 0;
    let input_flat: Vec<Node> = input.iter().flatten().cloned().collect();
    let nb_nodes = input_flat.len();
    for (i, node) in input_flat.iter().enumerate() {
        for &b in input_flat.iter().take(nb_nodes).skip(i + 1) {
            if node.viable_with(b) {
                nb += 1;
            }
            if b.viable_with(*node) {
                nb += 1;
            }
        }
    }
    nb
}

fn find_shortest_path(grid: &[Vec<Node>]) -> usize {
    // Check that exactly one node can contain data to move and get its coordinates
    let x_max = grid.len() - 1;
    let y_max = grid[0].len() - 1;
    let data_to_move_size = grid[x_max][0].used;
    let mut disponible_cell = vec![];
    for x in 0..=x_max {
        for y in 0..=y_max {
            if data_to_move_size <= (grid[x][y].size - grid[x][y].used) {
                disponible_cell.push((x, y));
            }
        }
    }
    assert_eq!(disponible_cell.len(), 1);

    // BFS
    let 


    0
}

fn day22_part1(input: &[Vec<Node>]) {
    // Solve puzzle
    let res = nb_viable_pairs(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 993);
    println!("> DAY22 - part 1: OK!");
}

fn day22_part2(example: &[Vec<Node>], input: &[Vec<Node>]) {
    // Exemple tests
    let res = find_shortest_path(example);
    // assert_eq!(res, 7);
    // println!("Example OK");

    // Solve puzzle
    println!();
    let res = find_shortest_path(input);
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY22 - part 2: OK!");
}
