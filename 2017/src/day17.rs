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

fn complete_circuler_buffer(steps: usize, last_value: usize) -> usize {
    let mut buffer: Vec<usize> = vec![0];
    let mut current_position = 0;
    for i in 1..=last_value {
        let new_position = (current_position + steps) % (i);
        buffer.insert(new_position, i);
        current_position = new_position;
    }
    println!("{:?}", buffer);
    buffer[current_position + 1]
}

fn day17_part1(input: usize) {
    // Exemple tests
    assert_eq!(complete_circuler_buffer(3, 1), 638);
    println!("Example OK");

    // Solve puzzle
    let res = complete_circuler_buffer(input, 2017);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY17 - part 1: OK!");
}

fn day17_part2(_input: usize) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY17 - part 2: OK!");
}
