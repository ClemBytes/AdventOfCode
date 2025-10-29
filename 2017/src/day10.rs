use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY10 -------");
    let example_parsed = vec![3, 4, 1, 5];
    let input = fs::read_to_string("inputs/input_day10").expect("Unable to read input!");
    let input_parsed = parse(&input);

    day10_part1(&example_parsed, &input_parsed);
    day10_part2(input);
}

fn parse(raw_input: &str) -> Vec<usize> {
    raw_input
        .trim()
        .split(",")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn apply_one_turn(list: Vec<usize>, length: usize, current_position: usize) -> Vec<usize> {
    let list_len = list.len();
    let mut res = list.clone();
    let position_end_section = current_position + length;
    for i in current_position..position_end_section {
        let mirror_position = (position_end_section - (i - current_position) - 1) % list_len;
        // let mirror_position = (position_end_section - i - 1) % list_len;
        // println!("i: {i} | mirror_position: {mirror_position}");
        res[i % list_len] = list[mirror_position];
        res[mirror_position] = list[i % list_len];
    }
    // println!("current_position: {current_position} | position_end_section: {position_end_section}");
    res
}

fn hash(size_list: usize, lengths: &[usize]) -> usize {
    let mut list: Vec<usize> = (0..size_list).collect();
    let mut current_position = 0;
    for (skip_size, &length) in lengths.iter().enumerate() {
        list = apply_one_turn(list, length, current_position);
        current_position += (length + skip_size) % size_list;
    }
    list[0] * list[1]
}

fn day10_part1(example: &[usize], input: &[usize]) {
    // Exemple tests
    let e = vec![0, 1, 2, 3, 4];
    // println!("{e:?}");
    let e1 = apply_one_turn(e, 3, 0);
    assert_eq!(e1, vec![2, 1, 0, 3, 4]);
    let e2 = apply_one_turn(e1, 4, 3);
    assert_eq!(e2, vec![4, 3, 0, 1, 2]);
    let e3 = apply_one_turn(e2, 1, 3);
    assert_eq!(e3, vec![4, 3, 0, 1, 2]);
    let e4 = apply_one_turn(e3, 5, 1);
    assert_eq!(e4, vec![3, 4, 2, 1, 0]);
    assert_eq!(hash(5, example), 12);

    // Solve puzzle
    let res = hash(256, input);
    println!("Result part 1: {res}");
    assert_eq!(res, 11413);
    println!("> DAY10 - part 1: OK!");
}

fn hash_n_times(n: usize, size_list: usize, lengths: &[usize]) -> Vec<usize> {
    let mut list: Vec<usize> = (0..size_list).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..n {
        for &length in lengths {
            list = apply_one_turn(list, length, current_position);
            current_position += (length + skip_size) % size_list;
            skip_size += 1;
        }
    }
    list
}

fn complete_knot_hash(input: String) -> String {
    // 1. Convert from ASCII
    let mut lengths: Vec<usize> = Vec::from(input.clone())
        .iter()
        .map(|&x| x as usize)
        .collect();

    // 2. Add final sequence
    let mut final_sequence: Vec<usize> = vec![17, 31, 73, 47, 23];
    lengths.append(&mut final_sequence);
    // println!("lengths: {lengths:?}");

    // 3. Run 64 times
    let n = 64;
    let hashed_list = hash_n_times(n, 256, &lengths);
    // println!("hashed_list: {hashed_list:?}");

    // 4. Bitwise XOR on blocks of 16:
    let mut pos = 0;
    let mut dense_hash = vec![];
    let mut x = 0;
    while pos < hashed_list.len() {
        match pos % 16 {
            0 => {
                dense_hash.push(x);
                x = hashed_list[pos];
            }
            _ => x ^= hashed_list[pos],
        }
        pos += 1;
    }
    dense_hash.push(x);
    // println!("dense_hash: {dense_hash:?}");

    let mut final_hash = "".to_string();
    for n in dense_hash {
        // println!("n: {n} | hex: {n:02x}");
        final_hash.push_str(&format!("{:02x}", n));
    }
    // println!("final_hash: {final_hash:?}");
    final_hash.remove(0);
    final_hash.remove(0);
    // println!("final_hash: {final_hash:?}");

    final_hash
}

fn day10_part2(input: String) {
    // Exemple tests
    assert_eq!(
        complete_knot_hash("".to_string()),
        "a2582a3a0e66e6e86e3812dcb672a272".to_string()
    );
    assert_eq!(
        complete_knot_hash("AoC 2017".to_string()),
        "33efeb34ea91902bb2f59c9920caa6cd".to_string()
    );
    assert_eq!(
        complete_knot_hash("1,2,3".to_string()),
        "3efbe78a8d82f29979031a4aa0b16a9d".to_string()
    );
    assert_eq!(
        complete_knot_hash("1,2,4".to_string()),
        "63960835bcdc130f0b66d7ff4f6a5a8e".to_string()
    );

    // Solve puzzle
    let res = complete_knot_hash(input.trim().to_string());
    println!("Result part 2: {res}");
    assert_eq!(res, "7adfd64c2a03a4968cf708d1b7fd418d".to_string());
    println!("> DAY10 - part 2: OK!");
}
