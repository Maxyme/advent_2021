use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

use itertools::iproduct;
type State = (usize, usize, usize, usize);

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut lines = input.lines();
    let p1_pos: usize = lines
        .next()
        .unwrap()
        .strip_prefix("Player 1 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    let p2_pos: usize = lines
        .next()
        .unwrap()
        .strip_prefix("Player 2 starting position: ")
        .unwrap()
        .parse()
        .unwrap();

    let (p1_score, p2_score, roll_count) = get_scores_deterministic(p1_pos, p2_pos, 1000);

    let losing_player_score = min(p1_score, p2_score);
    println!("Part 1: {}", roll_count * losing_player_score);

    // Find out player that wins in more universes
    let mut cache = HashMap::new();
    let (p1_wins, p2_wins) = get_scores_quantum(p1_pos, p2_pos, 0, 0, &mut cache);

    let winning_universes_count = max(p1_wins, p2_wins);
    println!("Part 2: {}", winning_universes_count);
}

/// Get dirac scores
fn get_scores_deterministic(
    p1_start_pos: usize,
    p2_start_pos: usize,
    winning_score: usize,
) -> (usize, usize, usize) {
    let mut dice_num = 1;
    let mut roll_count = 0;
    let mut dice_roll = || -> usize {
        roll_count += 1;
        let roll = dice_num;
        if dice_num <= 99 {
            dice_num += 1;
        } else {
            dice_num = 1;
        }
        roll
    };

    let mut p1_pos = p1_start_pos;
    let mut p2_pos = p2_start_pos;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let winning_score = winning_score;
    'outer: loop {
        for (pos, score_ref) in [(&mut p1_pos, &mut p1_score), (&mut p2_pos, &mut p2_score)] {
            // Roll 3 times and add the numbers
            let sum = (0..3).fold(0, |acc, _| acc + dice_roll());
            let new_pos = ((*pos + sum - 1) % 10) + 1;
            *score_ref += new_pos;
            if *score_ref >= winning_score {
                break 'outer;
            }
            *pos = new_pos;
        }
    }
    (p1_score, p2_score, roll_count)
}

fn get_scores_quantum(
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
    cache: &mut HashMap<State, (usize, usize)>,
) -> (usize, usize) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for (roll_1, roll_2, roll_3) in iproduct!(1..=3, 1..=3, 1..=3) {
        let roll_sum = roll_1 + roll_2 + roll_3;
        let new_p1_pos = ((p1_pos + roll_sum - 1) % 10) + 1;
        let new_p1_score = p1_score + new_p1_pos;
        if new_p1_score >= 21 {
            p1_wins += 1;
        } else {
            let (new_p2_wins, new_p1_wins) =
                match cache.get(&(p2_pos, new_p1_pos, p2_score, new_p1_score)) {
                    Some(scores) => *scores,
                    None => {
                        // Switch players when calling again
                        let scores =
                            get_scores_quantum(p2_pos, new_p1_pos, p2_score, new_p1_score, cache);
                        cache.insert((p2_pos, new_p1_pos, p2_score, new_p1_score), scores);
                        scores
                    }
                };
            p1_wins += new_p1_wins;
            p2_wins += new_p2_wins;
        }
    }
    (p1_wins, p2_wins)
}
