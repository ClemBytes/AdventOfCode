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

fn find_nb_presents_house_n_part1(house_number: u32) -> u32 {
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

fn find_nb_presents_house_n_part2(house_number: u32) -> u32 {
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
    divisors.sort();
    11 * divisors[..50.min(divisors.len())].iter().sum::<u32>()
}

fn find_lowest_house_number(n: u32, part: u32) -> u32 {
    let mut house_number = 1;
    loop {
        let mut nb_presents = 0;
        if part == 1 {
            nb_presents = find_nb_presents_house_n_part1(house_number);
        } else if part ==2 {
            nb_presents = find_nb_presents_house_n_part2(house_number);
        } else {
            panic!("Part should 1 or 2 and not {part}!");
        }
        if nb_presents >= n {
            return house_number;
        }
        house_number += 1;
    }
}

fn day20_part1(input: u32) {
    // Exemple tests
    assert_eq!(find_lowest_house_number(10, 1), 1);
    assert_eq!(find_lowest_house_number(30, 1), 2);
    assert_eq!(find_lowest_house_number(40, 1), 3);
    assert_eq!(find_lowest_house_number(70, 1), 4);
    assert_eq!(find_lowest_house_number(60, 1), 4);
    assert_eq!(find_lowest_house_number(120, 1), 6);
    assert_eq!(find_lowest_house_number(80, 1), 6);
    assert_eq!(find_lowest_house_number(150, 1), 8);
    assert_eq!(find_lowest_house_number(130, 1), 8);

    // Solve puzzle
    let res = find_lowest_house_number(input, 1);
    println!("Result part 1: {res}");
    assert_eq!(res, 665280);
    println!("> DAY20 - part 1: OK!");
}

fn day20_part2(input: u32) {
    // Solve puzzle
    let res = find_lowest_house_number(input, 2);
    println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY20 - part 2: OK!");
}
