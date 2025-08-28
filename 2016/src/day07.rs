use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY07 -------");
    let example1 = fs::read_to_string("inputs/example_day07_1").expect("Unable to read input!");
    let example1 = parse(&example1);
    let example2 = fs::read_to_string("inputs/example_day07_2").expect("Unable to read input!");
    let example2 = parse(&example2);
    let input = fs::read_to_string("inputs/input_day07").expect("Unable to read input!");
    let input = parse(&input);

    day07_part1(&example1, &input);
    day07_part2(&example2, &input);
}

#[derive(Debug, Clone)]
struct Block {
    within_brackets: bool,
    sequence: String,
}

impl Block {
    fn contains_abba(&self) -> bool {
        let mut first_char = self.sequence.chars().nth(0);
        let mut second_char = self.sequence.chars().nth(1);
        for i in 2..self.sequence.len() - 1 {
            if first_char != second_char
                && self.sequence.chars().nth(i) == second_char
                && self.sequence.chars().nth(i + 1) == first_char
            {
                return true;
            }
            first_char = second_char;
            second_char = self.sequence.chars().nth(i);
        }
        false
    }

    fn get_aba(&self) -> Option<Vec<(char, char)>> {
        let mut abas = vec![];
        let mut first_char = self.sequence.chars().nth(0).unwrap();
        let mut second_char = self.sequence.chars().nth(1).unwrap();
        for i in 2..self.sequence.len() {
            if first_char != second_char && self.sequence.chars().nth(i).unwrap() == first_char {
                abas.push((first_char, second_char));
            }
            first_char = second_char;
            second_char = self.sequence.chars().nth(i).unwrap();
        }
        if !abas.is_empty() { Some(abas) } else { None }
    }

    fn find_bab(&self, abas: Vec<(char, char)>) -> bool {
        for aba in abas {
            let (a, b) = aba;
            for i in 2..self.sequence.len() {
                if self.sequence.chars().nth(i - 2).unwrap() == b
                    && self.sequence.chars().nth(i - 1).unwrap() == a
                    && self.sequence.chars().nth(i).unwrap() == b
                {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Address {
    nb_blocks: u32,
    blocks: Vec<Block>,
}

impl Address {
    fn supports_tls(&self) -> bool {
        let mut res = false;
        for block in self.blocks.iter() {
            if block.contains_abba() && !block.within_brackets {
                res = true;
            } else if block.contains_abba() && block.within_brackets {
                return false;
            }
        }
        res
    }

    fn supports_ssl(&self) -> bool {
        // First search an aba
        for outside_brackets_block in self.blocks.iter() {
            if !outside_brackets_block.within_brackets
                && let Some(abas) = outside_brackets_block.get_aba()
            {
                // There are abas, so we look for a bab for each:
                for inside_brackets_block in self.blocks.iter() {
                    if inside_brackets_block.within_brackets
                        && inside_brackets_block.find_bab(abas.clone())
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn parse(raw_input: &str) -> Vec<Address> {
    let mut addresses: Vec<Address> = vec![];
    for line in raw_input.lines() {
        let mut address = Address {
            nb_blocks: 0,
            blocks: vec![],
        };
        let mut block = Block {
            within_brackets: false,
            sequence: String::new(),
        };
        for ch in line.chars() {
            if ch == '[' {
                address.nb_blocks += 1;
                address.blocks.push(block.clone());
                block.sequence.clear();
                block.within_brackets = true;
            } else if ch == ']' {
                address.nb_blocks += 1;
                address.blocks.push(block.clone());
                block.sequence.clear();
                block.within_brackets = false;
            } else {
                block.sequence.push(ch);
            }
        }
        address.nb_blocks += 1;
        address.blocks.push(block.clone());
        addresses.push(address);
    }
    addresses
}

fn day07_part1(example: &Vec<Address>, input: &Vec<Address>) {
    // Exemple tests
    let mut res = 0;
    for address in example {
        if address.supports_tls() {
            res += 1;
        }
    }
    assert_eq!(res, 2);

    // Solve puzzle
    let mut res = 0;
    for address in input {
        if address.supports_tls() {
            res += 1;
        }
    }
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY07 - part 1: OK!");
}

fn day07_part2(example: &Vec<Address>, input: &Vec<Address>) {
    // Exemple tests
    let mut res = 0;
    for address in example {
        if address.supports_ssl() {
            res += 1;
        }
    }
    assert_eq!(res, 3);

    // Solve puzzle
    let mut res = 0;
    for address in input {
        if address.supports_ssl() {
            res += 1;
        }
    }
    println!("Result part 2: {res}");
    assert_eq!(res, 242);
    println!("> DAY07 - part 2: OK!");
}
