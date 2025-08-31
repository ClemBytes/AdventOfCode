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
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    avail: u32,
    percentage: u32,
}

impl Node {
    fn from_str(raw_input: &str) -> Vec<Self> {
        let mut nodes = vec![];
        let re = Regex::new(
            r"^\/dev\/grid\/node-x([0-9]+)-y([0-9]+) +([0-9]+)T +([0-9]+)T +([0-9]+)T +([0-9]+)%$"
        ).unwrap();
        let mut lines = raw_input.lines();
        lines.next();
        lines.next();
        for line in lines {
            let matches = re.captures(line).unwrap();
            nodes.push(Node {
                x: matches[1].parse().unwrap(),
                y: matches[2].parse().unwrap(),
                size: matches[3].parse().unwrap(),
                used: matches[4].parse().unwrap(),
                avail: matches[5].parse().unwrap(),
                percentage: matches[6].parse().unwrap(),
            });
        }
        nodes
    }
}

fn day22_part1(input: &Vec<Node>) {
    println!("{input:#?}");
    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY22 - part 1: OK!");
}

fn day22_part2(_input: &Vec<Node>) {
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
