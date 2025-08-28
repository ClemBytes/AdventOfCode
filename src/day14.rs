use std::collections::HashMap;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY14 -------");
    let example = "abc";
    let input = "cuanljph";

    day14_part1(example, input);
    day14_part2(example, input);
}

fn find_repeat3(hash: &[char]) -> Option<char> {
    for i in 2..hash.len() {
        if hash[i - 2] == hash[i - 1] && hash[i - 1] == hash[i] {
            return Some(hash[i]);
        }
    }
    None
}

fn find_repeat5(hash: &[char], val: char) -> bool {
    for i in 4..hash.iter().len() {
        if hash[i - 4] == val
            && hash[i - 3] == val
            && hash[i - 2] == val
            && hash[i - 1] == val
            && hash[i] == val
        {
            return true;
        }
    }
    false
}

/*
#[test]
fn test_find_repeat5() {
    let s = "abc816";
    let digest = format!("{:x}", md5::compute(s));
    println!("{digest}");
    let hash: Vec<char> = digest.chars().collect();
    println!("{hash:?}");
    let res = find_repeat5(&hash, 'e');
    println!("{res}");
    assert!(res);
}
*/

fn find_index_password(salt: &str) -> u32 {
    let mut index = 0;
    let mut nb_keys = 0;
    let mut computed_md5 = HashMap::new();
    while nb_keys < 64 {
        computed_md5.entry(index).or_insert_with(|| {
            let s = format!("{salt}{index}");
            let hash: Vec<char> = format!("{:x}", md5::compute(s)).chars().collect();
            hash
        });

        let hash = computed_md5.get(&index).unwrap();

        if let Some(val) = find_repeat3(hash) {
            for j in index + 1..index + 1001 {
                computed_md5.entry(j).or_insert_with(|| {
                    let s = format!("{salt}{j}");
                    let hash: Vec<char> = format!("{:x}", md5::compute(s)).chars().collect();
                    hash
                });

                let new_hash = computed_md5.get(&j).unwrap();

                if find_repeat5(new_hash, val) {
                    nb_keys += 1;
                    break;
                }
            }
        }
        index += 1;
    }
    index - 1
}

fn day14_part1(example: &str, input: &str) {
    // Exemple tests
    let res = find_index_password(example);
    assert_eq!(res, 22728);
    println!("Example OK");

    // Solve puzzle
    let res = find_index_password(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 23769);
    println!("> DAY14 - part 1: OK!");
}

fn day14_part2(_example: &str, _input: &str) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY14 - part 2: OK!");
}
