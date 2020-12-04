use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Enumerate;
use std::path::Path;

fn check_tree_at_index(index: usize, tree_row: &str) -> bool {
    match tree_row.chars().nth(index) {
        Some(ascii) => ascii == '#',
        None => false,
    }
}

fn count_trees(tree_rows: Enumerate<Lines<BufReader<File>>>) -> u32 {
    struct Slope {
        x: u32,
        y: u32,
    }
    #[derive(Clone)]
    struct Loc {
        // our current index in a given row of the forest
        curr_x: u32,
        // our current row of the forest
        curr_y: u32,
        // how many trees we've found for our give slope
        tree_count: u32,
    }
    let starting_loc = Loc {
        curr_x: 0,
        curr_y: 0,
        tree_count: 0,
    };

    // pairs of slopes and associated locations
    let mut slopes: [(Slope, Loc); 5] = [
        (Slope { x: 1, y: 1 }, starting_loc.clone()),
        (Slope { x: 3, y: 1 }, starting_loc.clone()),
        (Slope { x: 5, y: 1 }, starting_loc.clone()),
        (Slope { x: 7, y: 1 }, starting_loc.clone()),
        (Slope { x: 1, y: 2 }, starting_loc.clone()),
    ];

    // 🎄 --- My solution for day 3 --- 🎄
    for (reader_index, tree_row) in tree_rows {
        match tree_row {
            Ok(tree_row) => {
                let tree_row = String::from(tree_row);
                // iterate over all slopes for a given row
                // (means we don't have to reread the file
                // or reallocate memory for *every* slope)
                for (slope, loc) in slopes.iter_mut() {
                    // if we're reading a row that matches our
                    // current row in the forest for a given slope:
                    if loc.curr_y == reader_index as u32 {
                        let is_tree = check_tree_at_index(loc.curr_x as usize, &tree_row);
                        if is_tree {
                            loc.tree_count += 1;
                        }
                        // make sure to wrap to the beginning as we move to the right
                        loc.curr_x = (loc.curr_x + slope.x) % tree_row.len() as u32;
                        loc.curr_y += slope.y;
                    }
                }
            }
            Err(_) => println!("Something's wrong with row {}!", reader_index),
        }
    }

    let mut multiplier = 1;
    for (_, loc) in slopes.iter() {
        multiplier *= loc.tree_count;
    }
    multiplier
}

fn main() {
    let tree_rows = read_lines("treees.txt");
    let multiplier = match tree_rows {
        Ok(tree_rows) => count_trees(tree_rows.enumerate()),
        Err(_) => {
            println!("Something's wrong with this input file!");
            0
        }
    };

    println!(
        "Looks like there's a product of {} trees in this forest",
        multiplier
    )
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
