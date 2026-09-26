use chrono::TimeDelta;
use chrono::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY04 -------");
    let example = fs::read_to_string("inputs/example_day04").expect("Unable to read input!");
    let example = parse_and_sort(&example);
    assert!(check_pattern(&example));
    let input = fs::read_to_string("inputs/input_day04").expect("Unable to read input!");
    let input = parse_and_sort(&input);
    assert!(check_pattern(&input));

    day04_part1(&example, &input);
    day04_part2(&example, &input);
}

#[derive(Debug, Clone, PartialEq)]
enum RecordKind {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Clone)]
struct Record {
    timestamp: NaiveDateTime,
    record_kind: RecordKind,
}

fn parse_and_sort(raw_input: &str) -> Vec<Record> {
    let mut records: Vec<Record> = vec![];
    let r = Regex::new(r"^\[([0-9]+-[0-9]+-[0-9]+ [0-9]+:[0-9]+)\] (.+)$").unwrap();
    for line in raw_input.lines() {
        let matches = r.captures(line).unwrap();
        let timestamp = NaiveDateTime::parse_from_str(&matches[1], "%Y-%m-%d %H:%M").unwrap();
        let action = matches[2].split_whitespace().collect::<Vec<&str>>();
        let record_kind = match action[0] {
            "Guard" => {
                let guard_id = action[1].split('#').collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap();
                RecordKind::BeginsShift(guard_id)
            }
            "falls" => RecordKind::FallsAsleep,
            "wakes" => RecordKind::WakesUp,
            _ => unreachable!(),
        };
        records.push(Record {
            timestamp,
            record_kind,
        });
    }
    records.sort_by_key(|r| r.timestamp);
    records
}

fn check_pattern(records: &[Record]) -> bool {
    // Check if we don't change a sleeping guard
    for (i, record) in records.iter().enumerate() {
        if i == 0 {
            if !matches!(record.record_kind, RecordKind::BeginsShift { .. }) {
                return false;
            }
            continue;
        }
        if i == records.len() - 1 {
            continue;
        }
        if matches!(record.record_kind, RecordKind::BeginsShift { .. }) {
            let previous = &records[i - 1];
            let next = &records[i + 1];
            if matches!(previous.record_kind, RecordKind::FallsAsleep) {
                println!("{previous:#?}");
                println!("{record:#?}");
                println!("{next:#?}");
                return false;
            }
            if matches!(next.record_kind, RecordKind::WakesUp) {
                println!("{previous:#?}");
                println!("{record:#?}");
                println!("{next:#?}");
                return false;
            }
        }

        if matches!(record.record_kind, RecordKind::FallsAsleep) {
            let next = &records[i + 1];
            if !matches!(next.record_kind, RecordKind::WakesUp) {
                println!("{record:#?}");
                println!("{next:#?}");
                return false;
            }
        }
    }
    true
}

fn minutes_asleep_by_guard(records: &[Record]) -> HashMap<u32, HashSet<(NaiveDate, NaiveTime)>> {
    let mut guards_to_minutes = HashMap::new();
    let mut current_guard_id = 0; // no guard has id 0
    for (i, record) in records.iter().enumerate() {
        match record.record_kind {
            RecordKind::BeginsShift(id) => current_guard_id = id,
            RecordKind::FallsAsleep => {}
            RecordKind::WakesUp => {
                let previous_record = &records[i - 1];
                assert!(matches!(
                    previous_record.record_kind,
                    RecordKind::FallsAsleep
                ));
                let asleep_timestamp = previous_record.timestamp;
                let mut current_timestamp = asleep_timestamp;
                while current_timestamp < record.timestamp {
                    let guard = guards_to_minutes
                        .entry(current_guard_id)
                        .or_insert(HashSet::new());
                    guard.insert((current_timestamp.date(), current_timestamp.time()));
                    current_timestamp += TimeDelta::try_minutes(1).unwrap();
                }
            }
        };
    }
    guards_to_minutes
}

fn find_guard_most_minutes_asleep(
    guards_to_minutes: &HashMap<u32, HashSet<(NaiveDate, NaiveTime)>>,
) -> u32 {
    let mut winner = 0; // no guard has id 0
    let mut max_minutes = 0;
    for (guard_id, minutes_set) in guards_to_minutes {
        let nb_minutes_asleep = minutes_set.len();
        if nb_minutes_asleep > max_minutes {
            winner = *guard_id;
            max_minutes = nb_minutes_asleep;
        }
    }
    winner
}

fn find_most_slept_minute(day_minutes_asleep: &HashSet<(NaiveDate, NaiveTime)>) -> (u32, u32) {
    let mut count_minutes: HashMap<u32, u32> = HashMap::new();
    for (_, time) in day_minutes_asleep {
        let count = count_minutes.entry(time.minute()).or_insert(0);
        *count += 1;
    }

    let mut winner = 100; // minute can't be over 59
    let mut max_count = 0;
    for (k, v) in count_minutes.iter() {
        if *v > max_count {
            winner = *k;
            max_count = *v;
        }
    }
    (winner, max_count)
}

fn strategy1(records: &[Record]) -> u32 {
    let minutes_asleep = minutes_asleep_by_guard(records);
    let guard_id = find_guard_most_minutes_asleep(&minutes_asleep);
    let most_slept_minute = find_most_slept_minute(minutes_asleep.get(&guard_id).unwrap());
    guard_id * most_slept_minute.0
}

fn day04_part1(example: &[Record], input: &[Record]) {
    // Exemple tests
    let minutes_asleep_by_guard_example = minutes_asleep_by_guard(example);
    let guard_most_asleep_id_example =
        find_guard_most_minutes_asleep(&minutes_asleep_by_guard_example);
    assert_eq!(guard_most_asleep_id_example, 10);
    assert_eq!(
        find_most_slept_minute(minutes_asleep_by_guard_example.get(&10).unwrap()),
        (24, 2)
    );
    assert_eq!(
        find_most_slept_minute(minutes_asleep_by_guard_example.get(&99).unwrap()),
        (45, 3)
    );
    assert_eq!(strategy1(example), 240);
    println!("Example OK");

    // Solve puzzle
    let res = strategy1(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 101262);
    println!("> DAY04 - part 1: OK!");
}

fn strategy2(records: &[Record]) -> u32 {
    // Very questionable variable names yeah…
    let minutes_asleep = minutes_asleep_by_guard(records);
    let mut max_max_count = 0;
    let mut winner_guard_id = 0;
    let mut winner_minute = 100;
    for (guard_id, minutes_set) in minutes_asleep {
        let (minute_winner, max_count) = find_most_slept_minute(&minutes_set);
        if max_count > max_max_count {
            winner_guard_id = guard_id;
            max_max_count = max_count;
            winner_minute = minute_winner;
        }
    }
    winner_minute * winner_guard_id
}

fn day04_part2(example: &[Record], input: &[Record]) {
    // Exemple tests
    assert_eq!(strategy2(example), 4455);
    println!("Example OK");

    // Solve puzzle
    let res = strategy2(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 71976);
    println!("> DAY04 - part 2: OK!");
}
