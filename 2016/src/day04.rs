use std::{collections::HashMap, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY04 -------");
    let example = fs::read_to_string("inputs/example_day04").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day04").expect("Unable to read input!");
    let input = parse(&input);

    day04_part1(&example, &input);
    day04_part2(&input);
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

impl Room {
    fn from_str(raw_input_line: &str) -> Self {
        let re = Regex::new(r"^([a-z\-]+)-([0-9]+)\[([a-z]+)\]$").unwrap();
        let matches = re.captures(raw_input_line).unwrap();
        Room {
            name: matches[1].to_string(),
            sector_id: matches[2].parse().unwrap(),
            checksum: matches[3].to_string(),
        }
    }

    fn is_real(&self) -> bool {
        let mut count_letters: HashMap<char, u32> = HashMap::new();
        for ch in self.name.chars() {
            if ch != '-' {
                let count = count_letters.entry(ch).or_insert(0);
                *count += 1;
            }
        }
        let mut count_letters_as_tuples: Vec<(u32, char)> = vec![];
        for (letter, count) in count_letters {
            count_letters_as_tuples.push((count, letter));
        }
        count_letters_as_tuples.sort_by(|(c1, l1), (c2, l2)| c2.cmp(c1).then(l1.cmp(l2)));
        if count_letters_as_tuples.len() < 5 {
            return false;
        }
        let mut computed_checksum = String::new();
        for tup in count_letters_as_tuples.iter().take(5) {
            computed_checksum.push(tup.1);
        }
        computed_checksum == self.checksum
    }

    fn sum_real_room_ids(rooms: &Vec<Self>) -> u32 {
        let mut sum = 0;
        for room in rooms {
            if room.is_real() {
                sum += room.sector_id;
            }
        }
        sum
    }

    fn real_name(&self, alphabet: &[char]) -> String {
        let mut real_name_string = String::new();
        for ch in self.name.chars() {
            if ch == '-' {
                real_name_string.push(ch);
            } else {
                real_name_string.push(
                    alphabet[(self.sector_id as usize
                        + alphabet.iter().position(|&x| x == ch).unwrap())
                        % 26],
                );
            }
        }
        real_name_string
    }

    fn get_plausible_rooms(rooms: &Vec<Self>) -> u32 {
        let alphabet: Vec<char> = ALPHABET.chars().collect();
        for room in rooms {
            if !room.is_real() {
                continue;
            } else {
                let real_name_string = room.real_name(&alphabet);
                if real_name_string.contains("north") {
                    println!("{} | {}", real_name_string, room.sector_id);
                    return room.sector_id;
                }
            }
        }
        unreachable!();
    }
}

fn parse(raw_input: &str) -> Vec<Room> {
    let mut rooms: Vec<Room> = vec![];
    for line in raw_input.lines() {
        rooms.push(Room::from_str(line));
    }
    rooms
}

fn day04_part1(example: &Vec<Room>, input: &Vec<Room>) {
    // Exemple tests
    assert_eq!(Room::sum_real_room_ids(example), 1514);

    // Solve puzzle
    let res = Room::sum_real_room_ids(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 361724);
    println!("> DAY04 - part 1: OK!");
}

fn day04_part2(input: &Vec<Room>) {
    // Exemple tests
    let ex = Room {
        name: "qzmt-zixmtkozy-ivhz".to_string(),
        sector_id: 343,
        checksum: "zimth".to_string(),
    };
    assert_eq!(
        ex.real_name(&ALPHABET.chars().collect::<Vec<char>>()),
        "very-encrypted-name"
    );

    // Solve puzzle
    let res = Room::get_plausible_rooms(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 482);
    println!("> DAY04 - part 2: OK!");
}
