use std::fs::read_to_string;
use std::collections::HashSet;
use regex::Regex;

//  key
//      on: colon -> value
//  value
//      on: space, ONE \n -> key
//      on: TWO \n -> parse_passport
//  parse_passport
//      action: check keys, count up if valid
//          onDone -> key

const VALID_EYE_COLORS: &[&str] = &[
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth"
];

fn to_int(value: &str) -> u32 {
    match value.parse::<u32>() {
        Ok(i) => i,
        Err(_e) => 0
    }
}

fn is_valid_passport_value(key: &str, value: &str) -> bool {
    match key {
        "byr" => to_int(value) >= 1920 && to_int(value) <= 2002,
        "iyr" => to_int(value) >= 2010 && to_int(value) <= 2020,
        "eyr" => to_int(value) >= 2020 && to_int(value) <= 2030,
        "hgt" => {
            let split = value.len() - 2;
            match &value[split..] {
                "cm" => {
                    let measurement = to_int(&value[..split]);
                    measurement >= 150 && measurement <= 193
                },
                "in" => {
                    let measurement = to_int(&value[..split]);
                    measurement >= 59 && measurement <= 76
                },
                _ => false
            }
        },
        "hcl" => Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap().is_match(value),
        "ecl" => VALID_EYE_COLORS.contains(&value),
        "pid" => value.len() == 9,
        _ => false
    }
}

fn read_passports(raw_passports: String) -> u32 {
    enum State {
        ReadKeys,
        ReadValues,
    }

    let required_keys: HashSet<String> = [
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string()
    ].iter().cloned().collect();

    let mut num_valid_passports = 0;
    let mut state = State::ReadKeys;
    let mut keys_unaccounted_for: HashSet<String> = required_keys.clone();
    let mut curr_key = String::from("");
    let mut curr_value = String::from("");

    for c in raw_passports.chars() {
        state = match state {
            State::ReadKeys => {
                match c {
                    // if we find a new-line, that (hopefully) means
                    // we found the first new-line in State::ReadValues
                    // and we bounced over here. 2 new lines = end of passport
                    '\n' => {
                        if keys_unaccounted_for.is_empty() {
                            num_valid_passports += 1;
                        }
                        // reset keys unaccounted for after validating the passport
                        keys_unaccounted_for = required_keys.clone();
                        State::ReadKeys
                    },
                    // on a colon, jump to reading the value
                    ':' => State::ReadValues,
                    c => {
                        // read the character into our running key
                        curr_key.push_str(&c.to_string());
                        State::ReadKeys
                    }
                }
            },
            State::ReadValues => {
                match c {
                    ' ' | '\n' => {
                        // if we're done reading the value,
                        // we're ready to validate the key / value pair
                        if is_valid_passport_value(&curr_key, &curr_value) {
                            keys_unaccounted_for.remove(&curr_key);
                        }
                        curr_key.clear();
                        curr_value.clear();
                        State::ReadKeys
                    },
                    c => {
                        // read the character into our running value
                        curr_value.push_str(&c.to_string());
                        State::ReadValues
                    }
                }
            },
        }
    }
    num_valid_passports
}

fn main() {
    let passports = read_to_string("passports.txt");

    let num_valid_passports = match passports {
        Ok(passports) => read_passports(passports),
        Err(_) => {
            println!("Something's wrong with this input file!");
            0
        }
    };

    println!(
        "Looks like there's {} valid passports here",
        num_valid_passports
    )
}