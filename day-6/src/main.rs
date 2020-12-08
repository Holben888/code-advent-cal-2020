use array_tool::vec::Intersect;
use std::collections::HashSet;
use std::fs::read_to_string;

fn format_groups_flattened(input_file: &str) -> Vec<String> {
    input_file
        .split("\n\n") // split up the groups
        .map(|s| s.replace('\n', "").to_string())
        .collect()
}

fn format_groups(input_file: &str) -> Vec<Vec<&str>> {
    input_file
        .split("\n\n") // split up the groups
        .map(|group| {
            group
                .split('\n') // create vec of individual answers
                .collect()
        })
        .collect()
}

fn count_unique_answers(group_answers: &str) -> u32 {
    group_answers
        .chars()
        .fold(HashSet::new(), |mut unique_answers, answer| {
            unique_answers.insert(answer);
            unique_answers
        }) // generate Set of answers to remove duplicates
        .len() as u32
}

fn count_unanimous_answers(group_answers: &Vec<&str>) -> u32 {
    if group_answers.len() == 0 {
        0
    } else {
        // assume the first group's answers are unanimous among all groups
        let starting_answers: Vec<char> = group_answers[0].chars().collect();
        group_answers
            .iter() // then, iterate through the groups to eliminate disagreements
            .fold(starting_answers, |unanimous_answers, person_answers| {
                // the intersection of all unanimous answers and a single group's answers
                // should eliminate answers the group didn't agree with
                unanimous_answers.intersect(person_answers.chars().collect())
            })
            .len() as u32
    }
}

fn main() {
    let input_file = read_to_string("group_answers.txt");

    match input_file {
        Ok(input_file) => {
            let part_1 = format_groups_flattened(&input_file)
                .iter()
                .fold(0, |answer_count, group| {
                    answer_count + count_unique_answers(group)
                });

            let part_2 = format_groups(&input_file)
                .iter()
                .fold(0, |unanimous_count, group| {
                    unanimous_count + count_unanimous_answers(group)
                });

            println!(
                "Among the groups, there's \n\
                - {} total answers \n\
                - {} unanimous answers",
                part_1, part_2
            )
        }
        Err(_) => println!("Something's wrong with this input file!"),
    };
}
