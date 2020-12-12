use std::collections::HashSet;
use std::fs::read_to_string;

fn to_int(value: &str) -> u32 {
    match value.parse::<u32>() {
        Ok(i) => i,
        Err(_e) => 0,
    }
}

fn split_lines(raw_input: &str) -> Vec<&str> {
    raw_input.split('\n').collect()
}

fn main() {
    let raw_input = read_to_string("joltages.txt");
    match raw_input {
        Ok(raw_input) => {
            // joltage_set: a HashSet of all joltages in the bag
            // max_joltage: the max joltage in the bag
            let (joltage_bag, max_joltage) = split_lines(&raw_input)
                .iter()
                .map(|line| to_int(line))
                .fold(
                    (HashSet::new(), 0),
                    |(mut joltage_set, max_joltage), joltage| {
                        joltage_set.insert(joltage);
                        (
                            joltage_set,
                            if joltage > max_joltage {
                                joltage
                            } else {
                                max_joltage
                            },
                        )
                    },
                );

            let (num_one_jumps, num_three_jumps, _) = (0..max_joltage + 1).fold(
                // we start with 0 jumps in joltage by 1
                // and 1 jump in joltage by 3 (since we're guaranteed a jump at the end)
                // lastly, we assume our starting joltage is 0
                (0, 1, 0),
                |(num_one_jumps, num_three_jumps, prev_joltage), joltage_to_check| {
                    // if we have this joltage in our bag, we can use it!
                    if joltage_bag.contains(&joltage_to_check) {
                        match joltage_to_check - prev_joltage {
                            1 => (num_one_jumps + 1, num_three_jumps, joltage_to_check),
                            3 => (num_one_jumps, num_three_jumps + 1, joltage_to_check),
                            _ => (num_one_jumps, num_three_jumps, joltage_to_check),
                        }
                    } else {
                        (num_one_jumps, num_three_jumps, prev_joltage)
                    }
                },
            );

            println!(
                "Looks like there's \n\
            - {} differences of 1 jolt \n\
            - {} differences of 3 jolts \n\
            Multiplied together, that's {}",
                num_one_jumps,
                num_three_jumps,
                num_one_jumps * num_three_jumps
            );
        }
        Err(_) => println!("Something's wrong with the input file!"),
    }
}
