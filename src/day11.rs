use std::str::FromStr;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY11 -------");
    let input = "vzbxkghb";

    day11_part1(input);
    day11_part2(input);
}

const BASE: u64 = 26;
const LEN_PASSWORD: u32 = 8;
const MAX: u64 = BASE.pow(LEN_PASSWORD);
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

struct Password {
    nb: u64,
    s: String,
}

impl Password {
    fn increment(&mut self) {
        self.nb += 1;
        self.nb %= MAX;
        self.nb_to_string();
    }

    fn nb_to_string(&mut self) {
        let mut copy = self.nb;
        let mut new_s = String::new();
        while copy > 0 {
            let rest = (copy % BASE) as usize;
            let letter = ALPHABET.chars().nth(rest).unwrap();
            new_s.push(letter);
            copy /= BASE;
        }
        for _ in new_s.len()..8 {
            new_s.push('a');
        }
        self.s = new_s.chars().rev().collect::<String>();
    }

    fn string_to_nb(&mut self) {
        let mut new_nb: u64 = 0;
        for ch in self.s.chars() {
            let digit = ALPHABET.find(ch).unwrap() as u64;
            new_nb = digit + BASE * new_nb;
        }
        self.nb = new_nb;
    }

    fn from_nb(n: u64) -> Self {
        let mut ret = Password {
            nb: n,
            s: String::new(),
        };
        ret.nb_to_string();
        ret
    }

    fn from_string(st: String) -> Self {
        let mut ret = Password { nb: 0, s: st };
        ret.string_to_nb();
        ret
    }

    fn is_correct(&self) -> bool {
        // Check if no forbidden characters (second requirement)
        if self.s.contains('i') || self.s.contains('o') || self.s.contains('l') {
            return false;
        }

        // Check three following letters (first requirement)
        let mut i: usize = 0;
        let mut found = false;
        let s_vec: Vec<char> = self.s.chars().collect();
        while i < s_vec.len() - 2 {
            let pos0 = ALPHABET.find(s_vec[i]).unwrap();
            let pos1 = ALPHABET.find(s_vec[i + 1]).unwrap();
            if pos1 != pos0 + 1 {
                i += 1;
                continue;
            }
            let pos2 = ALPHABET.find(s_vec[i + 2]).unwrap();
            if pos2 == pos1 + 1 {
                found = true;
                break;
            } else {
                i += 1;
                continue;
            }
        }
        if !found {
            return false;
        }

        // Check doubles (third requirement)
        let mut i: usize = 0;
        while i < s_vec.len() - 3 {
            if s_vec[i] == s_vec[i + 1] {
                let mut j = i + 2;
                while j < s_vec.len() - 1 {
                    if s_vec[j] != s_vec[i] && s_vec[j] == s_vec[j + 1] {
                        return true;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        false
    }

    fn next_correct(&self) -> Self {
        let mut next = Password {
            nb: self.nb,
            s: self.s.clone(),
        };
        next.increment();
        while !next.is_correct() {
            next.increment();
        }
        next
    }
}

fn day11_part1(input: &str) {
    // Exemple tests
    println!("> Check conversions:");
    let ex = Password::from_string(String::from("aaaaaaaa"));
    println!("Password string: {} | password number: {}", ex.s, ex.nb);
    let ex = Password::from_string(String::from("aaaaaaab"));
    println!("Password string: {} | password number: {}", ex.s, ex.nb);
    let ex = Password::from_string(String::from("aaaaaaba"));
    println!("Password string: {} | password number: {}", ex.s, ex.nb);
    let ex = Password::from_nb(0);
    println!("Password number: {} | password string: {}", ex.nb, ex.s);
    let ex = Password::from_nb(1);
    println!("Password number: {} | password string: {}", ex.nb, ex.s);
    let ex = Password::from_nb(28);
    println!("Password number: {} | password string: {}", ex.nb, ex.s);

    println!("\n> Check requirements:");
    let ex = Password::from_string(String::from("hijklmmn"));
    println!("Password string: {} | password number: {}", ex.s, ex.nb);
    assert!(!ex.is_correct());
    let ex = Password::from_string(String::from("abbceffg"));
    println!("Password string: {} | password number: {}", ex.s, ex.nb);
    assert!(!ex.is_correct());
    let ex = Password::from_string(String::from("abbcegjk"));
    println!("Password string: {} | password number: {}", ex.s, ex.nb);
    assert!(!ex.is_correct());

    println!("\n> Check next good password.");
    let ex = Password::from_string(String::from("abcdefgh"));
    assert_eq!(ex.next_correct().s, String::from("abcdffaa"));
    let ex = Password::from_string(String::from("ghijklmn"));
    assert_eq!(ex.next_correct().s, String::from("ghjaabcc"));

    // Solve puzzle
    let input = Password::from_string(String::from_str(input).unwrap());
    let res = input.next_correct().s;
    println!("\n> Result part 1: {res}");
    assert_eq!(res, "vzbxxyzz");
    println!("> DAY11 - part 1: OK!\n");
}

fn day11_part2(_input: &str) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY11 - part 2: OK!");
}
