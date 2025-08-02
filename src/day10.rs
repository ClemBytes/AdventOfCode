#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY10 -------");
    let input = "1113222113";

    day10_part1(input);
    day10_part2(input);
}

fn look_and_say(s: String) -> String {
    let mut i: usize = 0;
    let s_col: Vec<char> = s.chars().collect();
    let mut ret: String = String::new();
    while i < s_col.len() {
        if i == s_col.len() - 1 {
            ret.push('1');
            ret.push(s_col[i]);
            return ret;
        }
        let mut nb_repeat = 1;
        let mut j = i + 1;
        while j < s.len() && s_col[j] == s_col[i] {
            nb_repeat += 1;
            j += 1;
        }
        ret.push_str(&nb_repeat.to_string());
        ret.push(s_col[i]);
        i = j;
    }
    ret
}

fn day10_part1(input: &str) {
    // Exemple tests
    assert_eq!(look_and_say(String::from("1")), String::from("11"));
    assert_eq!(look_and_say(String::from("11")), String::from("21"));
    assert_eq!(look_and_say(String::from("21")), String::from("1211"));
    assert_eq!(look_and_say(String::from("1211")), String::from("111221"));
    assert_eq!(look_and_say(String::from("111221")), String::from("312211"));

    // Solve puzzle
    let mut result = String::from(input);
    for _ in 0..40 {
        result = look_and_say(result);
    }
    println!("Result part 1: {}", result.len());
    assert_eq!(result.len(), 252594);
    println!("> DAY10 - part 1: OK!");
}

fn day10_part2(_input: &str) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY10 - part 2: OK!");
}
