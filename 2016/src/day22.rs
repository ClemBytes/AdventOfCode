use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY22 -------");
    let input = fs::read_to_string("inputs/input_day22").expect("Unable to read input!");
    let input = Node::from_str(&input);

    day22_part1(&input);
    day22_part2(&input);
}

#[derive(Debug, Clone, Copy)]
struct Node {
    _size: u32,
    used: u32,
    avail: u32,
    _percentage: u32,
}

impl Node {
    fn from_str(raw_input: &str) -> Vec<Vec<Self>> {
        // Important note: the data in input is arranged in order…
        // x0-y0, x0-y1, x0-y2 …, x1-y0, x1-y1, x1-y2…
        let mut nodes = vec![];
        let mut new_line = vec![];
        let re = Regex::new(
            r"^\/dev\/grid\/node-x([0-9]+)-y([0-9]+) +([0-9]+)T +([0-9]+)T +([0-9]+)T +([0-9]+)%$",
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
                    _size: matches[3].parse().unwrap(),
                    used: matches[4].parse().unwrap(),
                    avail: matches[5].parse().unwrap(),
                    _percentage: matches[6].parse().unwrap(),
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
        self.used <= b.avail
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

fn day22_part1(input: &[Vec<Node>]) {
    // Solve puzzle
    let res = nb_viable_pairs(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 993);
    println!("> DAY22 - part 1: OK!");
}

fn day22_part2(_input: &[Vec<Node>]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY22 - part 2: OK!");
}
