use std::collections::{HashSet, VecDeque};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY17 -------");
    let input = "pgflpeqp";

    day17_part1(input);
    day17_part2(input);
}

#[derive(Debug, Clone)]
struct State {
    position: (usize, usize),
    path: String,
    hash: String,
}

fn is_open(ch: char) -> bool {
    ['b', 'c', 'd', 'e', 'f'].contains(&ch)
}

fn find_shortest_path(password: String, nb_lines: usize, nb_cols: usize) -> String {
    let init = State {
        position: (0, 0),
        path: "".to_string(),
        hash: format!("{:x}", md5::compute(&password)),
    };
    let mut q = VecDeque::new();
    q.push_back(init);
    let mut visited = HashSet::new();
    while let Some(current_state) = q.pop_front() {
        // println!("current: {current_state:?} | q: {q:?}");
        let current_hash = current_state.hash;
        if visited.contains(&current_hash) {
            continue;
        }
        visited.insert(current_hash.clone());

        let (x, y) = current_state.position;
        if x == 3 && y == 3 {
            return current_state.path;
        }

        let current_hash_chars: Vec<char> = current_hash.chars().collect();
        let up = current_hash_chars[0];
        let down = current_hash_chars[1];
        let left = current_hash_chars[2];
        let right = current_hash_chars[3];

        if x > 0 && is_open(up) {
            let mut new_path = current_state.path.clone();
            new_path.push('U');
            let mut to_hash = password.clone();
            to_hash.push_str(&new_path);
            q.push_back(State {
                position: (x - 1, y),
                hash: format!("{:x}", md5::compute(to_hash)),
                path: new_path,
            });
        }

        if x + 1 < nb_lines && is_open(down) {
            let mut new_path = current_state.path.clone();
            new_path.push('D');
            let mut to_hash = password.clone();
            to_hash.push_str(&new_path);
            q.push_back(State {
                position: (x + 1, y),
                hash: format!("{:x}", md5::compute(to_hash)),
                path: new_path,
            });
        }

        if y > 0 && is_open(left) {
            let mut new_path = current_state.path.clone();
            new_path.push('L');
            let mut to_hash = password.clone();
            to_hash.push_str(&new_path);
            q.push_back(State {
                position: (x, y - 1),
                hash: format!("{:x}", md5::compute(to_hash)),
                path: new_path,
            });
        }

        if y + 1 < nb_cols && is_open(right) {
            let mut new_path = current_state.path.clone();
            new_path.push('R');
            let mut to_hash = password.clone();
            to_hash.push_str(&new_path);
            q.push_back(State {
                position: (x, y + 1),
                hash: format!("{:x}", md5::compute(to_hash)),
                path: new_path,
            });
        }
    }
    unreachable!("Didn't find a way for: {password}");
}

fn find_longest_path(password: String, nb_lines: usize, nb_cols: usize) -> usize {
    let init = State {
        position: (0, 0),
        path: "".to_string(),
        hash: format!("{:x}", md5::compute(&password)),
    };
    let mut q = VecDeque::new();
    q.push_back(init);
    let mut visited = HashSet::new();
    let mut paths_lengthes = Vec::new();
    while let Some(current_state) = q.pop_front() {
        // println!("current: {current_state:?} | q: {q:?}");
        let current_hash = current_state.hash;
        if visited.contains(&current_hash) {
            continue;
        }
        visited.insert(current_hash.clone());

        let (x, y) = current_state.position;
        if x == 3 && y == 3 {
            paths_lengthes.push(current_state.path.len());
            continue;
        }

        let current_hash_chars: Vec<char> = current_hash.chars().collect();
        let up = current_hash_chars[0];
        let down = current_hash_chars[1];
        let left = current_hash_chars[2];
        let right = current_hash_chars[3];

        if x > 0 && is_open(up) {
            let mut new_path = current_state.path.clone();
            new_path.push('U');
            let mut to_hash = password.clone();
            to_hash.push_str(&new_path);
            q.push_back(State {
                position: (x - 1, y),
                hash: format!("{:x}", md5::compute(to_hash)),
                path: new_path,
            });
        }

        if x + 1 < nb_lines && is_open(down) {
            let mut new_path = current_state.path.clone();
            new_path.push('D');
            let mut to_hash = password.clone();
            to_hash.push_str(&new_path);
            q.push_back(State {
                position: (x + 1, y),
                hash: format!("{:x}", md5::compute(to_hash)),
                path: new_path,
            });
        }

        if y > 0 && is_open(left) {
            let mut new_path = current_state.path.clone();
            new_path.push('L');
            let mut to_hash = password.clone();
            to_hash.push_str(&new_path);
            q.push_back(State {
                position: (x, y - 1),
                hash: format!("{:x}", md5::compute(to_hash)),
                path: new_path,
            });
        }

        if y + 1 < nb_cols && is_open(right) {
            let mut new_path = current_state.path.clone();
            new_path.push('R');
            let mut to_hash = password.clone();
            to_hash.push_str(&new_path);
            q.push_back(State {
                position: (x, y + 1),
                hash: format!("{:x}", md5::compute(to_hash)),
                path: new_path,
            });
        }
    }
    *paths_lengthes.iter().max().unwrap()
}

fn day17_part1(input: &str) {
    // Exemple tests
    assert_eq!(find_shortest_path("ihgpwlah".to_string(), 4, 4), "DDRRRD");
    assert_eq!(
        find_shortest_path("kglvqrro".to_string(), 4, 4),
        "DDUDRLRRUDRD"
    );
    assert_eq!(
        find_shortest_path("ulqzkmiv".to_string(), 4, 4),
        "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
    );

    // Solve puzzle
    let res = find_shortest_path(input.to_string(), 4, 4);
    println!("Result part 1: {res}");
    assert_eq!(res, "RDRLDRDURD");
    println!("> DAY17 - part 1: OK!");
}

fn day17_part2(input: &str) {
    // Exemple tests
    assert_eq!(find_longest_path("ihgpwlah".to_string(), 4, 4), 370);
    assert_eq!(find_longest_path("kglvqrro".to_string(), 4, 4), 492);
    assert_eq!(find_longest_path("ulqzkmiv".to_string(), 4, 4), 830);

    // Solve puzzle
    let res = find_longest_path(input.to_string(), 4, 4);
    println!("Result part 2: {res}");
    assert_eq!(res, 596);
    println!("> DAY17 - part 2: OK!");
}
