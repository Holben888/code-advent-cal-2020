use std::collections::HashMap;
use std::fs::read_to_string;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum SEAT {
    Empty,
    Occ,
    Floor,
}

fn split_lines(raw: &str) -> Vec<&str> {
    raw.split('\n').collect()
}

fn to_seat(c: char) -> SEAT {
    match c {
        '#' => SEAT::Occ,
        'L' => SEAT::Empty,
        _ => SEAT::Floor,
    }
}

fn map_to_counts(seats: &Vec<SEAT>) -> HashMap<SEAT, u32> {
    seats.iter().fold(HashMap::new(), |mut seat_count, seat| {
        *seat_count.entry(*seat).or_insert(0) += 1;
        seat_count
    })
}

fn get_nearest_seat_in_direction(
    seating_chart: &Vec<Vec<SEAT>>,
    row: usize,
    seat: usize,
    direction: (i32, i32),
    take_adjacent: bool,
) -> Option<SEAT> {
    let mut nearest_row = row as i32;
    let mut nearest_seat = seat as i32;
    let row_len = (seating_chart[0].len() - 1) as i32;
    let chart_len = (seating_chart.len() - 1) as i32;
    loop {
        nearest_row += direction.0;
        nearest_seat += direction.1;
        if nearest_row < 0 || nearest_row > chart_len || nearest_seat < 0 || nearest_seat > row_len
        {
            // if we went out-of-bounds, we clearly couldn't find a seat
            break None;
        }
        let seat = seating_chart[nearest_row as usize][nearest_seat as usize];
        if take_adjacent || seat == SEAT::Occ || seat == SEAT::Empty {
            break Some(seat);
        }
    }
}

fn find_directional_seat_counts(
    seating_chart: &Vec<Vec<SEAT>>,
    row: usize,
    seat: usize,
    // 👇 depends on whether you're doing part 1 or part 2!
    // - if part 1, you want to *always* find the adjacent seat, even if it's the floor
    // - if part 2, you want to find the first seat that *isn't* the floor
    // (so not necessarily the adjacent)
    take_adjacent: bool,
) -> HashMap<SEAT, u32> {
    let nearest_seats: Vec<SEAT> = DIRECTIONS
        .iter()
        .map(|direction| {
            get_nearest_seat_in_direction(seating_chart, row, seat, *direction, take_adjacent)
        })
        .filter(|seat| seat.is_some())
        .map(|seat| seat.unwrap())
        .collect();
    map_to_counts(&nearest_seats)
}

fn decide_seating(seating_chart: &Vec<Vec<SEAT>>) -> Vec<Vec<SEAT>> {
    let mut revised_seating: Vec<Vec<SEAT>> = Vec::new();
    let mut something_was_revised = false;
    for (row_index, row) in seating_chart.iter().enumerate() {
        revised_seating.push(Vec::new());
        for (seat_index, seat) in row.iter().enumerate() {
            let revised_seat = match seat {
                SEAT::Occ => {
                    let nearby_seat_counts =
                        find_directional_seat_counts(seating_chart, row_index, seat_index, true);
                    match nearby_seat_counts.get(&SEAT::Occ) {
                        Some(count) if *count >= 4 => {
                            something_was_revised = true;
                            SEAT::Empty
                        }
                        _ => SEAT::Occ,
                    }
                }
                SEAT::Empty => {
                    let nearby_seat_counts =
                        find_directional_seat_counts(seating_chart, row_index, seat_index, true);
                    match nearby_seat_counts.get(&SEAT::Occ) {
                        // if we couldn't find a count for occupied seats... there aren't any!
                        None => {
                            something_was_revised = true;
                            SEAT::Occ
                        }
                        _ => SEAT::Empty,
                    }
                }
                _ => SEAT::Floor,
            };
            revised_seating[row_index].push(revised_seat);
        }
    }
    // if we had to change something this round, we need to process again
    if something_was_revised {
        decide_seating(&revised_seating)
    } else {
        revised_seating
    }
}

fn main() {
    let raw_input = read_to_string("seat_chart.txt");
    match raw_input {
        Ok(raw_input) => {
            let seating_chart: Vec<Vec<SEAT>> = split_lines(&raw_input)
                .iter()
                .map(|row| row.chars().map(|c| to_seat(c)).collect())
                .collect();

            // flatten our seating chart to count all the occupied seats
            let occ_count = decide_seating(&seating_chart).iter().flatten().fold(
                0 as u32,
                |occ_count, seat| match seat {
                    SEAT::Occ => occ_count + 1,
                    _ => occ_count,
                },
            );
            println!("Looks like there's {} occupied seats", occ_count);
        }
        Err(_) => println!("There's something wrong with the input file!"),
    }
}
