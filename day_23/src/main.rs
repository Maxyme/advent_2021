#![feature(is_sorted)]

extern crate core;

use std::cmp::min;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

lazy_static! {
    static ref PATHS: HashMap<(usize, usize), (usize, Vec<usize>)> = {
        let mut m = HashMap::new();
        m.insert((0, 0), (3, vec![1]));
        m.insert((0, 1), (5, vec![1, 2]));
        m.insert((0, 2), (7, vec![1, 2, 3]));
        m.insert((0, 3), (9, vec![1, 2, 3, 4]));

        m.insert((1, 0), (2, vec![]));
        m.insert((1, 1), (4, vec![2]));
        m.insert((1, 2), (6, vec![2, 3]));
        m.insert((1, 3), (8, vec![2, 4]));

        m.insert((2, 0), (2, vec![]));
        m.insert((2, 1), (2, vec![]));
        m.insert((2, 2), (4, vec![3]));
        m.insert((2, 3), (6, vec![3, 4]));

        m.insert((3, 0), (4, vec![2]));
        m.insert((3, 1), (2, vec![]));
        m.insert((3, 2), (2, vec![]));
        m.insert((3, 3), (4, vec![4]));

        m.insert((4, 0), (6, vec![2, 3]));
        m.insert((4, 1), (4, vec![3]));
        m.insert((4, 2), (2, vec![]));
        m.insert((4, 3), (2, vec![]));

        m.insert((5, 0), (8, vec![2, 3, 4]));
        m.insert((5, 1), (6, vec![3, 4]));
        m.insert((5, 2), (4, vec![4]));
        m.insert((5, 3), (2, vec![]));

        m.insert((6, 0), (9, vec![2, 3, 4, 5]));
        m.insert((6, 1), (7, vec![3, 4, 5]));
        m.insert((6, 2), (5, vec![4, 5]));
        m.insert((6, 3), (3, vec![5]));

        m
    };
    static ref COSTS: HashMap<usize, usize> = {
        let mut m = HashMap::new();
        m.insert(0, 1);
        m.insert(1, 10);
        m.insert(2, 100);
        m.insert(3, 1000);
        m
    };
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

/// Parse input in list of silos
fn parse(lines: &Vec<&str>) -> Vec<Vec<usize>> {
    // Skip first 2 lines
    // lines.next();
    // lines.next();
    let mut rows: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let mut letters = Vec::new();
        for char in line.chars() {
            if char.is_alphabetic() {
                let num = match char {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    'D' => 4,
                    _ => panic!(""),
                };
                letters.push(num)
            }
        }
        if !letters.is_empty() {
            rows.push(letters)
        }
    }
    // Transpose rows in silos
    transpose2(rows)
}

#[derive(Debug, Hash, Clone, PartialEq)]
struct State {
    silos: Vec<Vec<usize>>,
    corridor: [usize; 7],
    cost: usize,
}

impl State {
    fn done(&self) -> bool {
        // Todo: remove transpose
        let rows = transpose2(self.silos.clone());
        for row in &rows {
            if row != &vec![1_usize, 2, 3, 4] {
                return false;
            }
        }
        true
    }
}

fn move_state(
    state: &State,
    corr_index: usize,
    corr_val: usize,
    room_index: usize,
    room_pos: usize,
    room_val: usize,
    score: usize,
) -> State {
    let mut room = state.silos.clone();
    room[room_index][room_pos] = room_val;
    let mut corridor = state.corridor;
    corridor[corr_index] = corr_val;
    State {
        silos: room,
        corridor,
        cost: state.cost + score,
    }
}

/// Get all possible new states following all logical moves
fn get_new_states(state: &State) -> Vec<State> {
    let mut valid_states = Vec::new();
    for (corr_index, amphipod) in state.corridor.iter().enumerate() {
        if amphipod != &0 {
            // Amphipod in corridor space
            let room_index = amphipod - 1;
            if state.silos[room_index]
                .iter()
                .all(|x| HashSet::<usize>::from_iter([0, *amphipod]).contains(x))
            {
                let (distance, cells) = &PATHS[&(corr_index, room_index)];
                if !cells.iter().any(|x| state.corridor[*x] != 0) {
                    let room_pos = state.silos[room_index].iter().filter(|&x| *x == 0).count() - 1;
                    let cost = (distance + room_pos) * COSTS[&(amphipod - 1)];
                    let new_state =
                        move_state(state, corr_index, 0, room_index, room_pos, *amphipod, cost);
                    valid_states.push(new_state);
                }
            }
        }
    }

    for silo_index in 0..4 {
        let room = &state.silos[silo_index];
        if !room
            .iter()
            .all(|x| HashSet::<usize>::from_iter([silo_index + 1, 0]).contains(x))
        {
            let room_pos = room.iter().filter(|&x| *x == 0).count();
            let mover = room[room_pos];
            for (corr_index, occupant) in state.corridor.iter().enumerate() {
                if occupant == &0 {
                    // corr index is empty
                    let (distance, cells) = PATHS.get(&(corr_index, silo_index)).unwrap();
                    if cells.iter().all(|x| state.corridor[*x] == 0) {
                        let cost = (distance + room_pos) * COSTS[&(mover - 1)];
                        let new_state =
                            move_state(state, corr_index, mover, silo_index, room_pos, 0, cost);
                        valid_states.push(new_state);
                    }
                }
            }
        }
    }
    valid_states
}

fn prune_states(states: Vec<State>) -> Vec<State> {
    let mut scores: HashMap<(Vec<Vec<usize>>, [usize; 7]), State> = HashMap::new();
    for state in states {
        let key = (state.silos.clone(), state.corridor);
        match scores.get_mut(&key)  {
            Some(x) => {
                x.cost = min(x.cost, state.cost)
            },
            None => {
                scores.insert(key, state.clone());
            }
        }
    }
    scores.values().into_iter().map(|x| x.to_owned()).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut lines: Vec<_>  = input.lines().collect();
    let silos = parse(&lines);
    let min_cost = get_min_cost(silos);
    // Part 1: What is the least energy required to organize the amphipods
    println!("Part 1 {}", min_cost);

    // Part 2: What is the least energy required to organize the amphipods with the added lines
    //   #D#C#B#A#
    //   #D#B#A#C#
    lines.insert(3, " #D#C#B#A#");
    lines.insert(4, " #D#B#A#C#");
    let silos = parse(&lines);

    let min_cost = get_min_cost(silos);
    println!("Part 2: {}", min_cost);
}

fn get_min_cost(silos: Vec<Vec<usize>>) -> usize {
    let corridor = [0, 0, 0, 0, 0, 0, 0];
    let initial_state = State {
        silos,
        corridor,
        cost: 0,
    };

    let mut states: Vec<State> = vec![initial_state];

    let mut completed_steps: Vec<State> = Vec::new();

    while !states.is_empty() {
        // todo: Pop instead queue
        let mut possible_states = Vec::new();
        for state in &states {
            let new_possible_states = get_new_states(state);
            for new_state in new_possible_states {
                if new_state.done() {
                    completed_steps.push(new_state.clone());
                } else {
                    possible_states.push(new_state.clone())
                }
            }
        }

        states = prune_states(possible_states.clone());
    }

    // Get min energy
    let mut min_cost = usize::MAX;
    for state in completed_steps {
        if state.cost < min_cost {
            min_cost = state.cost;
        }
    }
    min_cost
}
