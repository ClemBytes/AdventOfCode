#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY17 -------");
    let input = 344;

    day17_part1(input);
    day17_part2(input);
}

fn complete_circuler_buffer(steps: usize, last_value: usize, value_after: usize) -> usize {
    let mut buffer: Vec<usize> = vec![0];
    let mut current_position = 0;
    for i in 1..=last_value {
        let new_position = (current_position + steps) % (i);
        buffer.insert(new_position + 1, i);
        current_position = new_position + 1;
        // println!("{:?} | current_position: {current_position} | current_position value: {}", buffer, buffer[current_position]);
    }
    let next_value_position = buffer.iter().position(|&r| r == value_after).unwrap();
    buffer[next_value_position + 1]
}

fn day17_part1(input: usize) {
    // Exemple tests
    assert_eq!(complete_circuler_buffer(3, 2017, 2017), 638);

    // Solve puzzle
    let res = complete_circuler_buffer(input, 2017, 2017);
    println!("Result part 1: {res}");
    assert_eq!(res, 996);
    println!("> DAY17 - part 1: OK!");
}

fn day17_part2(input: usize) {
    // Solve puzzle
    let res = complete_circuler_buffer(input, 50_000_000, 0);
    println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY17 - part 2: OK!");
}
