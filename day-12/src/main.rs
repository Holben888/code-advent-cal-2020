use regex::Regex;
use std::fs::read_to_string;

fn to_int(value: &str) -> u32 {
    match value.parse::<u32>() {
        Ok(i) => i,
        Err(_e) => 0,
    }
}

#[derive(Debug)]
enum Nav {
    N(u32),
    S(u32),
    E(u32),
    W(u32),
    L(u32),
    R(u32),
    F(u32),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

struct Loc(i64, i64);
struct Waypoint(i64, i64);

fn to_nav_instruction(raw_instruction: regex::Captures) -> Nav {
    let instr = &raw_instruction[1];
    let amount = to_int(&raw_instruction[2]);
    match instr {
        "N" => Nav::N(amount),
        "S" => Nav::S(amount),
        "E" => Nav::E(amount),
        "W" => Nav::W(amount),
        "L" => Nav::L(amount),
        "R" => Nav::R(amount),
        _ => Nav::F(amount),
    }
}

fn rotate_direction(degrees: u32, direction: Direction, counterclockwise: bool) -> Direction {
    let degrees = if counterclockwise {
        (degrees as i64 * -1).into()
    } else {
        degrees as i64
    };
    match (degrees + 90 * direction as i64) % 360 {
        90 | -270 => Direction::E,
        180 | -180 => Direction::S,
        270 | -90 => Direction::W,
        _ => Direction::N,
    }
}

fn rotate_waypoint(degrees: u32, waypoint: Waypoint, counterclockwise: bool) -> Waypoint {
    let degrees = if counterclockwise {
        (degrees as i64 * -1).into()
    } else {
        degrees as i64
    };
    match degrees % 360 {
        90 | -270 => Waypoint(waypoint.1, -waypoint.0),
        180 | -180 => Waypoint(-waypoint.0, -waypoint.1),
        270 | -90 => Waypoint(-waypoint.1, waypoint.0),
        _ => waypoint,
    }
}

fn process_part_1(nav_instructions: &Vec<Nav>) -> (Loc, Direction) {
    nav_instructions
        .iter()
        .fold((Loc(0, 0), Direction::E), |(loc, dir), nav_instruction| {
            (
                match (nav_instruction, dir) {
                    (Nav::N(distance), _) | (Nav::F(distance), Direction::N) => {
                        Loc(loc.0, loc.1 + *distance as i64)
                    }
                    (Nav::S(distance), _) | (Nav::F(distance), Direction::S) => {
                        Loc(loc.0, loc.1 - *distance as i64)
                    }
                    (Nav::E(distance), _) | (Nav::F(distance), Direction::E) => {
                        Loc(loc.0 + *distance as i64, loc.1)
                    }
                    (Nav::W(distance), _) | (Nav::F(distance), Direction::W) => {
                        Loc(loc.0 - *distance as i64, loc.1)
                    }
                    _ => loc,
                },
                match nav_instruction {
                    Nav::L(degrees) => rotate_direction(*degrees, dir, true),
                    Nav::R(degrees) => rotate_direction(*degrees, dir, false),
                    _ => dir,
                },
            )
        })
}

fn process_part_2(nav_instructions: &Vec<Nav>) -> (Loc, Waypoint) {
    nav_instructions.iter().fold(
        (Loc(0, 0), Waypoint(10, 1)),
        |(loc, waypoint), nav_instruction| {
            (
                match nav_instruction {
                    Nav::F(multiplier) => Loc(
                        loc.0 + waypoint.0 * *multiplier as i64,
                        loc.1 + waypoint.1 * *multiplier as i64,
                    ),
                    _ => loc,
                },
                match nav_instruction {
                    Nav::L(degrees) => rotate_waypoint(*degrees, waypoint, true),
                    Nav::R(degrees) => rotate_waypoint(*degrees, waypoint, false),
                    Nav::N(distance) => Waypoint(waypoint.0, waypoint.1 + *distance as i64),
                    Nav::S(distance) => Waypoint(waypoint.0, waypoint.1 - *distance as i64),
                    Nav::E(distance) => Waypoint(waypoint.0 + *distance as i64, waypoint.1),
                    Nav::W(distance) => Waypoint(waypoint.0 - *distance as i64, waypoint.1),
                    _ => waypoint,
                },
            )
        },
    )
}

fn calc_manhattan_distance(loc: &Loc) -> u32 {
    (loc.0.abs() + loc.1.abs()) as u32
}

fn main() {
    let raw_input = read_to_string("nav_instructions.txt");
    let capture_instructions = Regex::new(r"([A-Z])([0-9]*)").unwrap();
    match raw_input {
        Ok(raw_input) => {
            let nav_instructions: Vec<Nav> = capture_instructions
                .captures_iter(&raw_input)
                .map(|capture| to_nav_instruction(capture))
                .collect();

            let (part_1_pos, _) = process_part_1(&nav_instructions);
            let (part_2_pos, _) = process_part_2(&nav_instructions);

            println!(
                "Here's the Manhattan distances we found: \n\
                1. Based on directions: {} \n\
                2. Based on waypoints: {}",
                calc_manhattan_distance(&part_1_pos),
                calc_manhattan_distance(&part_2_pos),
            );
        }
        Err(_) => println!("There's something wrong with the input file!"),
    }
}
