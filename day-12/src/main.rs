use regex::Regex;
use std::fs::read_to_string;

fn to_int(value: &str) -> u32 {
    match value.parse::<u32>() {
        Ok(i) => i,
        Err(_e) => 0,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Nav {
    N(u32),
    S(u32),
    E(u32),
    W(u32),
    L(u32),
    R(u32),
    F(u32),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

struct Position {
    location: (i64, i64),
    direction: Direction,
}

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

fn rotate_by_degrees(degrees: u32, curr_direction: Direction, counterclockwise: bool) -> Direction {
    let degrees = if counterclockwise {
        (degrees as i64 * -1).into()
    } else {
        degrees as i64
    };
    match (degrees + 90 * curr_direction as i64) % 360 {
        90 | -270 => Direction::E,
        180 | -180 => Direction::S,
        270 | -90 => Direction::W,
        _ => Direction::N,
    }
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

            let position = nav_instructions.iter().fold(
                Position {
                    direction: Direction::E,
                    location: (0, 0),
                },
                |curr_position, nav_instruction| {
                    let (horiz, vert) = curr_position.location;
                    Position {
                        direction: match nav_instruction {
                            Nav::L(degrees) => {
                                rotate_by_degrees(*degrees, curr_position.direction, true)
                            }
                            Nav::R(degrees) => {
                                rotate_by_degrees(*degrees, curr_position.direction, false)
                            }
                            _ => curr_position.direction,
                        },
                        location: match (nav_instruction, curr_position.direction) {
                            (Nav::N(distance), _) | (Nav::F(distance), Direction::N) => {
                                (horiz, vert + *distance as i64)
                            }
                            (Nav::S(distance), _) | (Nav::F(distance), Direction::S) => {
                                (horiz, vert - *distance as i64)
                            }
                            (Nav::E(distance), _) | (Nav::F(distance), Direction::E) => {
                                (horiz + *distance as i64, vert)
                            }
                            (Nav::W(distance), _) | (Nav::F(distance), Direction::W) => {
                                (horiz - *distance as i64, vert)
                            }
                            _ => (horiz, vert),
                        },
                    }
                },
            );

            println!(
                "{:?} {:?}",
                position.location.0.abs() + position.location.1.abs(),
                position.direction
            );
        }
        Err(_) => println!("There's something wrong with the input file!"),
    }
}
