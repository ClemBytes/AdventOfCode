#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY20 -------");
    let input: u32 = 29000000;

    day20_part1(input);
    day20_part2(input);
}

fn find_nb_presents_house_n(house_number: u32) -> u32 {
    let mut divisors: Vec<u32> = vec![];
    let sqrt_n = (house_number as f64).sqrt() as u32;
    for i in 1..=sqrt_n {
        if house_number % i == 0 {
            divisors.push(i);
            if i != house_number / i {
                divisors.push(house_number / i);
            }
        }
    }
    10 * divisors.iter().sum::<u32>()
}

fn find_lowest_house_number(n: u32) -> u32 {
    let mut house_number = 1;
    loop {
        let nb_presents = find_nb_presents_house_n(house_number);
        if nb_presents >= n {
            return house_number;
        }
        house_number += 1;
    }
}

fn day20_part1(input: u32) {
    // Exemple tests
    assert_eq!(find_lowest_house_number(10), 1);
    assert_eq!(find_lowest_house_number(30), 2);
    assert_eq!(find_lowest_house_number(40), 3);
    assert_eq!(find_lowest_house_number(70), 4);
    assert_eq!(find_lowest_house_number(60), 4);
    assert_eq!(find_lowest_house_number(120), 6);
    assert_eq!(find_lowest_house_number(80), 6);
    assert_eq!(find_lowest_house_number(150), 8);
    assert_eq!(find_lowest_house_number(130), 8);

    // Solve puzzle
    let res = find_lowest_house_number(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 665280);
    println!("> DAY20 - part 1: OK!");
}

fn day20_part2(_input: u32) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY20 - part 2: OK!");
}
