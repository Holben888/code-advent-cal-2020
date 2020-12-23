use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

fn to_int(value: &str) -> u32 {
    match value.parse::<u32>() {
        Ok(i) => i,
        Err(_e) => 0,
    }
}

fn sum_hand(hand: &VecDeque<u32>) -> u32 {
    let hand_len = hand.len();
    hand.iter().enumerate().fold(0, |sum, (index, &card)| {
        sum + (hand_len - index) as u32 * card
    })
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn part_1_simple_game(p1_hand: &VecDeque<u32>, p2_hand: &VecDeque<u32>) -> u32 {
    let mut p1_hand = p1_hand.clone();
    let mut p2_hand = p2_hand.clone();
    let player_1_wins = loop {
        if p2_hand.len() == 0 {
            break true;
        }
        if p1_hand.len() == 0 {
            break false;
        }
        let p1_card = p1_hand.pop_front().unwrap();
        let p2_card = p2_hand.pop_front().unwrap();
        if p1_card > p2_card {
            p1_hand.push_back(p1_card);
            p1_hand.push_back(p2_card);
        } else {
            p2_hand.push_back(p2_card);
            p2_hand.push_back(p1_card);
        }
    };
    if player_1_wins {
        sum_hand(&p1_hand)
    } else {
        sum_hand(&p2_hand)
    }
}

fn part_2_rec_game(p1_hand: &VecDeque<u32>, p2_hand: &VecDeque<u32>) -> (bool, u32) {
    let mut p1_hand = p1_hand.clone();
    let mut p2_hand = p2_hand.clone();
    let mut prev_hands = HashSet::new();
    let player_1_wins = loop {
        let p1_hand_len = p1_hand.len() as u32;
        let p2_hand_len = p2_hand.len() as u32;
        let p1_hash = calculate_hash(&p1_hand);
        let p2_hash = calculate_hash(&p2_hand);
        // efficiently check whether a given hand was already played this game
        // if it was, let's just assume p1 wins
        if prev_hands.contains(&p1_hash) || prev_hands.contains(&p2_hash) {
            break true;
        }
        prev_hands.insert(p1_hash);
        prev_hands.insert(p2_hash);
        if p2_hand_len == 0 {
            break true;
        }
        if p1_hand_len == 0 {
            break false;
        }
        let p1_card = p1_hand.pop_front().unwrap();
        let p2_card = p2_hand.pop_front().unwrap();
        if p1_card >= p1_hand_len || p2_card >= p2_hand_len {
            if p1_card > p2_card {
                p1_hand.push_back(p1_card);
                p1_hand.push_back(p2_card);
            } else {
                p2_hand.push_back(p2_card);
                p2_hand.push_back(p1_card);
            }
        } else {
            // this is just horrible
            // it's my lazy attempt at getting a "slice" of a queue
            // deserves some love someday!
            let p1_starting_hand: Vec<u32> = Vec::from(p1_hand.clone())[0..p1_card as usize]
                .iter()
                .cloned()
                .collect();
            let p2_starting_hand: Vec<u32> = Vec::from(p2_hand.clone())[0..p2_card as usize]
                .iter()
                .cloned()
                .collect();
            let (p1_wins, _) = part_2_rec_game(
                &VecDeque::from(p1_starting_hand),
                &VecDeque::from(p2_starting_hand),
            );
            if p1_wins {
                p1_hand.push_back(p1_card);
                p1_hand.push_back(p2_card);
            } else {
                p2_hand.push_back(p2_card);
                p2_hand.push_back(p1_card);
            }
        }
    };
    let winning_hand = if player_1_wins {
        sum_hand(&p1_hand)
    } else {
        sum_hand(&p2_hand)
    };
    (player_1_wins, winning_hand)
}

fn main() {
    let raw_file = match read_to_string("puzzle.txt") {
        Ok(raw_file) => raw_file,
        Err(err) => panic!("Something's wrong with the input file! \n{}", err),
    };

    // regex to grab the card values in each player's hand,
    // separated by different matching groups
    let r_player_hands = Regex::new(r"Player [1|2]:(([ |\n]*[0-9]+)*)").unwrap();
    let player_hands = r_player_hands.captures_iter(&raw_file).enumerate().fold(
        (VecDeque::new(), VecDeque::new()),
        |(player_1_hand, player_2_hand), (index, captures)| {
            let hand_values: VecDeque<u32> =
                captures[1].trim().split('\n').map(|s| to_int(s)).collect();
            match index {
                0 => (hand_values, player_2_hand),
                1 => (player_1_hand, hand_values),
                _ => panic!("Now that's what I call a bad input file"),
            }
        },
    );

    let (p1_hand, p2_hand) = player_hands;
    let (p1_wins, winning_hand) = part_2_rec_game(&p1_hand, &p2_hand);

    println!("{} {:?}", p1_wins, winning_hand);
}
