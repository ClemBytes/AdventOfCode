use std::{
    collections::VecDeque,
    fs,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY24 -------");
    let example: Vec<u64> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
    let raw_input = fs::read_to_string("inputs/input_day24").expect("Unable to read input!");
    let mut input: Vec<u64> = vec![];
    for line in raw_input.lines() {
        input.push(line.parse::<u64>().unwrap());
    }

    day24_part1(&example, &input);
    day24_part2(&example, &input);
}

// #[test]
// fn testlkjlkja() {
//     let bla = can_be_balanced_in_3_groups(&[1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 41, 43, 47, 53, 59, 107, 101, 97, 89, 83, 79, 73, 71, 67]);
//     println!("{bla}");
// // }

// fn nb_packages_group1(state: Vec<u8>) -> u8 {
//     state.iter().sum()
// }

fn quantum_entanglement(state: &[u64], list_of_packages: &[u64]) -> u64 {
    let mut qe = 1;
    for (i, is_in_group1) in state.iter().enumerate() {
        if *is_in_group1 == 1 {
            qe *= list_of_packages[i];
        }
    }
    qe
}

fn can_be_balanced(list_of_packages: &[u64], used: &mut Vec<u64>, target_weight: u64, used_weight: u64, nb_groups: usize) -> bool {
    // Only one group or no packages to balance: trivially OK
    if nb_groups == 1 {
        return true;
    }

    if used_weight == target_weight {
        can_be_balanced(list_of_packages, used, target_weight, used_weight, nb_groups - 1)
    } else {
        // Find last package added
        let mut next_package_index = 0;
        for (i, package) in used.iter().enumerate() {
            if *package == 1 {
                next_package_index = i + 1;
            }
        }
        // Try to add each packages after that
        for i in next_package_index..list_of_packages.len() {
            used[i] = 1;
            if can_be_balanced(list_of_packages, used, target_weight, used_weight, nb_groups) {
                return true;
            }
            used[i] = 0;
        }
        false
    }
}

/*
#[test]
fn test_find_subset() {
    // Test possible subset
    assert!(find_subset(&[1, 5, 2, 6, 3, 4, 8], &mut vec![0, 1, 0, 1, 0, 0, 1], 5));
    assert!(find_subset(&[1, 5, 2, 6, 3, 4, 8], &mut vec![0, 1, 0, 1, 0, 1, 1], 3));
    assert!(find_subset(&[1, 2, 3, 3, 4, 5, 9, 12], &mut vec![0, 1, 0, 0, 1, 0, 1, 1], 6));
    // Test impossible subset
    assert!(!find_subset(&[3, 7], &mut vec![0, 0], 5));
    assert!(!find_subset(&[3, 3, 4], &mut vec![0, 0, 0], 5));
    assert!(!find_subset(&[1, 2, 3, 3, 4, 5, 9, 12], &mut vec![1, 0, 1, 1, 1, 1, 1, 0], 7));
}
*/

#[test]
fn test_can_be_balanced() {
    // Balance 3 groups
    /*
    assert!(!can_be_balanced(&[1, 2, 3, 4, 5], &mut vec![0, 0, 0, 0, 0], 5, 3)); // [] is used, can't do 0 whith the others
    assert!(can_be_balanced(&[1, 2, 3, 4, 5], &mut vec![0, 0, 0, 0, 1], 5, 3)); // [5] is used, can do [1, 4] = 5 and [2, 3] = 5
    assert!(can_be_balanced(&[5, 5, 5], &mut vec![0, 0, 1], 5, 3)); // [5] is used, can do [5] = 5 and [5] = 5
    assert!(!can_be_balanced(&[1, 2, 3], &mut vec![1, 0, 0], 3)); // [1] is used and no possibility to do 1 with 2 and 3
    assert!(can_be_balanced(&[2, 4, 2, 1, 3], &mut vec![0, 1, 0, 0, 0], 3)); // [4] is used and can do [2, 2] = 4 and [1, 3] = 4
    assert!(!can_be_balanced(&[1, 2, 4, 2, 1, 3], &mut vec![1, 0, 1, 0, 0, 0], 3)); // [1, 4] = 5 is used and can do [1, 2, 2] = 5 but then [3] != 5
    assert!(!can_be_balanced(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1], &mut vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 3)); // [1, 1, 1, 1, 1] = 5 is used and then we can do [1, 1, 1, 1, 1] = 5 but then [] != 5
    assert!(can_be_balanced(&[1, 1, 1, 1, 1, 1, 1, 1, 1], &mut vec![0, 1, 0, 0, 0, 0, 0, 1, 1], 3)); // [1, 1, 1] = 3 is used; can do twice [1, 1, 1] = 3
    */
}

fn find_ideal_configuration(list_of_packages: &[u64], nb_groups: usize) -> u64 {
    let total_weight: u64 = list_of_packages.iter().sum();
    assert_eq!(total_weight % (nb_groups as u64), 0);
    let target_weight: u64 = total_weight / (nb_groups as u64);

    let nb_packages_total = list_of_packages.len();
    let initial_state: Vec<u64> = vec![0; nb_packages_total];
    let mut q: VecDeque<(Vec<u64>, u64)> = VecDeque::new();
    q.push_back((initial_state, 0));
    while let Some((mut used, used_weight)) = q.pop_front() {
        if used_weight == target_weight && can_be_balanced(list_of_packages, &mut used, target_weight, used_weight, nb_groups - 1) {
            return quantum_entanglement(&used, list_of_packages);
        }

        // Else: add neighbors to heap
        // Find last package added
        let mut next_package_index = 0;
        for (i, is_used) in used.iter().enumerate() {
            if *is_used == 1 {
                next_package_index = i + 1;
            }
        }
        // Try to add each packages after that
        for i in next_package_index..nb_packages_total {
            used[i] = 1;
            q.push_back((used.clone(), used_weight + list_of_packages[i]));
            used[i] = 0;
        }
    }
    unreachable!()
}

fn day24_part1(example: &[u64], input: &[u64]) {
    // Exemple tests
    let res = find_ideal_configuration(example, 3);
    println!("Result example part 1: {res}");
    assert_eq!(res, 99);

    // Solve puzzle
    let res = find_ideal_configuration(input, 3);
    println!("Result part 1: {res}");
    assert_eq!(res, 11266889531); // 22554787 is too low, 326440069283 is too high
    println!("> DAY24 - part 1: OK!");
}

fn day24_part2(example: &[u64], input: &[u64]) {
    // Exemple tests
    let res = find_ideal_configuration(example, 4);
    println!("Result example part 2: {res}");
    assert_eq!(res, 44);

    // Solve puzzle
    let res = find_ideal_configuration(input, 4);
    println!("Result part 2: {res}");
    assert_eq!(res, 77387711);
    println!("> DAY24 - part 2: OK!");
}
