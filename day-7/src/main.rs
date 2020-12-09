use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

// match: ([0-9]) ([a-z]* [a-z]*) bags?]*
fn split_lines(raw: &str) -> Vec<&str> {
    raw.split('\n').collect()
}

fn traverse(
    curr: &str,
    visited_bags: &HashSet<String>,
    bag_to_parents_map: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    match bag_to_parents_map.get(curr) {
        Some(parent_bags) => {
            let mut visited_bags = parent_bags.difference(&visited_bags).fold(
                HashSet::new(),
                |visited_bags, parent_bag| {
                    traverse(parent_bag, &visited_bags, bag_to_parents_map)
                        .union(&visited_bags)
                        .cloned()
                        .collect()
                },
            );
            visited_bags.insert(curr.to_string());
            visited_bags
        }
        None => [curr.to_string()].iter().cloned().collect(),
    }
}

fn main() {
    let get_bag_info = Regex::new(r"([0-9]) (\w* \w*) bags?").unwrap();
    // assumes the color will be stated first, with 2 words
    let get_first_bag_color = Regex::new(r"^\w+ \w+").unwrap();

    let raw_input = read_to_string("bags.txt");
    match raw_input {
        Ok(raw_input) => {
            let lines = split_lines(&raw_input);
            let mut bag_to_parents_map: HashMap<String, HashSet<String>> = HashMap::new();
            for line in lines {
                match get_first_bag_color.captures(line) {
                    Some(first_bag_color_captures) => {
                        let first_bag_color = &first_bag_color_captures[0];
                        for bag_info_capture in get_bag_info.captures_iter(line) {
                            let number = &bag_info_capture[1];
                            let color = &bag_info_capture[2];

                            bag_to_parents_map
                                // find the right bag in our Map
                                .entry(color.to_string())
                                // if we can't find it,
                                // create the key with an empty Set as our value
                                .or_insert(HashSet::new())
                                // otherwise, append to the existing Set at that key
                                .insert(first_bag_color.to_string());
                        }
                    }
                    None => println!("This line is poorly formatted \n {}", line),
                }
            }
            println!("{:?}", bag_to_parents_map);
            println!(
                "{}",
                traverse("shiny gold", &HashSet::new(), &bag_to_parents_map).len() - 1
            );
        }
        Err(_) => println!("Something's wrong with the input file!"),
    }
}
