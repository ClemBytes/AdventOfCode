use std::cmp::Ordering;
use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY22 -------");
    let input = fs::read_to_string("inputs/input_day22").expect("Unable to read input!");
    let (boss_hp, boss_damage) = input.split_once("\n").unwrap();
    let boss_hp = boss_hp.trim().split(": ").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();
    let boss_damage = boss_damage.trim().split(": ").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let mut initial_state_input = State {
        mana_used: 0,
        player_hp: 50,
        player_mana: 500,
        boss_hp,
        boss_damage,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        turn: Turn::Player,
        hard: false,
    };

    let initial_state_example1 = State {
        mana_used: 0,
        player_hp: 10,
        player_mana: 250,
        boss_hp: 13,
        boss_damage: 8,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        turn: Turn::Player,
        hard: false,
    };

    let initial_state_example2 = State {
        mana_used: 0,
        player_hp: 10,
        player_mana: 250,
        boss_hp: 14,
        boss_damage: 8,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        turn: Turn::Player,
        hard: false,
    };

    day22_part1(
        initial_state_example1.clone(),
        initial_state_example2.clone(),
        initial_state_input.clone(),
    );

    initial_state_input.hard = true;
    day22_part2(initial_state_input.clone());
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Turn {
    Player,
    Boss,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    // All mana already spent
    mana_used: i32,
    // Player infos
    player_hp: i32,
    player_mana: i32,
    // Boss infos
    boss_hp: i32,
    boss_damage: i32,
    // Effects timers
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    // Whose turn
    turn: Turn,
    // Difficulty
    hard: bool,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Other and self inversed to do a MIN-heap
        other.mana_used.cmp(&self.mana_used)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn apply_effects(&self) -> Self {
        let mut new_state = self.clone();
        if self.shield_timer > 0 {
            new_state.shield_timer -= 1;
        }
        if self.poison_timer > 0 {
            new_state.boss_hp -= 3;
            new_state.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            new_state.player_mana += 101;
            new_state.recharge_timer -= 1;
        }
        new_state
    }

    fn next_states(&self) -> Vec<Self> {
        let mut next_possible_states: Vec<State> = vec![];
        let mut state = self.clone();
        if state.hard && state.turn == Turn::Player {
            state.player_hp -= 1;
        }
        let mut state = state.apply_effects();

        match state.turn {
            Turn::Player => {
                state.turn = Turn::Boss;

                // Magic Missile
                let magic_missile_cost = 53;
                if state.player_mana >= magic_missile_cost {
                    let mut new_state = state.clone();
                    new_state.boss_hp -= 4;
                    new_state.player_mana -= magic_missile_cost;
                    new_state.mana_used += magic_missile_cost;
                    next_possible_states.push(new_state);
                }

                // Drain
                let drain_cost = 73;
                if state.player_mana >= drain_cost {
                    let mut new_state = state.clone();
                    new_state.boss_hp -= 2;
                    new_state.player_hp += 2;
                    new_state.player_mana -= drain_cost;
                    new_state.mana_used += drain_cost;
                    next_possible_states.push(new_state);
                }

                // Shield
                let shield_cost = 113;
                if state.player_mana >= shield_cost && state.shield_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.shield_timer = 6;
                    new_state.player_mana -= shield_cost;
                    new_state.mana_used += shield_cost;
                    next_possible_states.push(new_state);
                }

                // Poison
                let poison_cost = 173;
                if state.player_mana >= poison_cost && state.poison_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.poison_timer = 6;
                    new_state.player_mana -= poison_cost;
                    new_state.mana_used += poison_cost;
                    next_possible_states.push(new_state);
                }

                // Recharge
                let recharge_cost = 229;
                if state.player_mana >= recharge_cost && state.recharge_timer == 0 {
                    let mut new_state = state.clone();
                    new_state.recharge_timer = 5;
                    new_state.player_mana -= recharge_cost;
                    new_state.mana_used += recharge_cost;
                    next_possible_states.push(new_state);
                }
            }

            Turn::Boss => {
                state.turn = Turn::Player;
                let player_armor = if state.shield_timer > 0 { 7 } else { 0 };
                state.player_hp -= (state.boss_damage - player_armor).max(1);
                next_possible_states.push(state);
            }
        }
        next_possible_states
    }
}

fn find_cheapest_mana_win_dijkstra(initial_state: State) -> i32 {
    let mut seen_states: HashSet<State> = HashSet::new();
    let mut min_heap: BinaryHeap<State> = BinaryHeap::new();
    min_heap.push(initial_state);
    while let Some(current_state) = min_heap.pop() {
        // State already visited?
        // TODO: merge contains and insert with Entry
        if seen_states.contains(&current_state) {
            continue;
        }
        seen_states.insert(current_state.clone());

        // Win?
        if current_state.boss_hp <= 0 {
            return current_state.mana_used;
        }

        // Lose?
        if current_state.player_hp <= 0 {
            continue;
        }

        // Else: add neighbors to heap
        for next_state in current_state.next_states() {
            min_heap.push(next_state);
        }
    }
    unreachable!();
}

fn day22_part1(
    initial_state_example1: State,
    initial_state_example2: State,
    initial_state_input: State,
) {
    // Exemple tests
    let res = find_cheapest_mana_win_dijkstra(initial_state_example1);
    println!("Result example 1 part 1: {res}");
    assert_eq!(res, 226);

    let res = find_cheapest_mana_win_dijkstra(initial_state_example2);
    println!("Result example 2 part 1: {res}");
    assert_eq!(res, 641);

    // Solve puzzle
    let res = find_cheapest_mana_win_dijkstra(initial_state_input);
    println!("Result part 1: {res}");
    assert_eq!(res, 1269); // 932 is too low, 1415 is too high, 1093 not Ok, should be 1269
    println!("> DAY22 - part 1: OK!");
}

fn day22_part2(initial_state_input: State) {
    // Solve puzzle
    let res = find_cheapest_mana_win_dijkstra(initial_state_input);
    println!("Result part 2: {res}");
    assert_eq!(res, 1309);
    println!("> DAY22 - part 2: OK!");
}
