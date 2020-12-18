use std::collections::HashMap;

fn generate_starting_map(input: &[u64]) -> HashMap<u64, u64> {
    let mut num_to_turn_last_spoken = HashMap::new();
    for (index, &num) in input.iter().enumerate() {
        // index by one!
        num_to_turn_last_spoken.insert(num, index as u64 + 1);
    }
    num_to_turn_last_spoken
}

fn main() {
    let input = [0, 1, 5, 10, 3, 12, 19];
    let num_rounds = 30000000;

    let input_len = input.len();
    // fill our map of words to "last turn spoken" with our input vector
    // but ignore the last element! This is the first number spoken in our loop,
    // so it can't exist in our mapping yet
    let mut num_to_turn_last_spoken = generate_starting_map(&input[..input_len - 1]);
    let mut last_spoken_num = input[input_len - 1];

    for turn in input_len as u64..num_rounds {
        let num_to_speak = match num_to_turn_last_spoken.get(&last_spoken_num) {
            Some(turn_last_spoken) => turn - turn_last_spoken,
            None => 0,
        };
        num_to_turn_last_spoken.insert(last_spoken_num, turn);
        last_spoken_num = num_to_speak;
    }

    println!(
        "The last spoken number at round {}: {}",
        num_rounds, last_spoken_num
    );
}
