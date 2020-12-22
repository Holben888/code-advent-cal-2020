use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cube {
    A,
    I,
}

const DIRECTIONS: [(i32, i32, i32); 26] = [
    (-1, -1, -1),
    (0, -1, -1),
    (1, -1, -1),
    (-1, 0, -1),
    (0, 0, -1),
    (1, 0, -1),
    (-1, 1, -1),
    (0, 1, -1),
    (1, 1, -1),
    (-1, -1, 0),
    (0, -1, 0),
    (1, -1, 0),
    (-1, 0, 0),
    // (0, 0, 0),
    (1, 0, 0),
    (-1, 1, 0),
    (0, 1, 0),
    (1, 1, 0),
    (-1, -1, 1),
    (0, -1, 1),
    (1, -1, 1),
    (-1, 0, 1),
    (0, 0, 1),
    (1, 0, 1),
    (-1, 1, 1),
    (0, 1, 1),
    (1, 1, 1),
];

fn create_empty_plane(dimensions: &[usize; 3]) -> Vec<Vec<Vec<Cube>>> {
    vec![vec![vec![Cube::I; dimensions[2]]; dimensions[1]]; dimensions[0]]
}

fn split_lines(raw_file: &str) -> Vec<&str> {
    raw_file.split('\n').collect()
}

fn map_to_counts(cubes: &Vec<Cube>) -> HashMap<Cube, u32> {
    cubes.iter().fold(HashMap::new(), |mut count, cube| {
        *count.entry(*cube).or_insert(0) += 1;
        count
    })
}

fn get_nearest_cube_in_direction(
    plane: &Vec<Vec<Vec<Cube>>>,
    plane_dimensions: &[usize; 3],
    curr_cube: (usize, usize, usize),
    direction: &(i32, i32, i32),
) -> Option<Cube> {
    let adjacent_cube_coords = [
        curr_cube.0 as i32 + direction.0,
        curr_cube.1 as i32 + direction.1,
        curr_cube.2 as i32 + direction.2,
    ];
    let mut i = 0;
    loop {
        // if we hit this point, we've checked all 3 dimensions for invalid values
        // now we're safe to return this adjacement cube!
        if i == 3 {
            break Some(
                plane[adjacent_cube_coords[0] as usize][adjacent_cube_coords[1] as usize]
                    [adjacent_cube_coords[2] as usize],
            );
        }
        // check whether this index is out-of-bounds
        if adjacent_cube_coords[i] < 0 || adjacent_cube_coords[i] >= plane_dimensions[i] as i32 {
            break None;
        }
        i += 1;
    }
}

fn part_1(plane: &Vec<Vec<Vec<Cube>>>, plane_dimensions: &[usize; 3], cycles: usize) {
    let plane = (0..cycles).fold(plane.clone(), |plane, _| {
        let mut new_plane = create_empty_plane(plane_dimensions);
        for (grid_i, grid) in plane.iter().enumerate() {
            for (row_i, row) in grid.iter().enumerate() {
                for (cube_i, &cube) in row.iter().enumerate() {
                    let adjacent_cubes: Vec<Cube> = DIRECTIONS
                        .iter()
                        .map(|direction| {
                            get_nearest_cube_in_direction(
                                &plane,
                                plane_dimensions,
                                (grid_i, row_i, cube_i),
                                direction,
                            )
                        })
                        .filter(|cube| cube.is_some())
                        .map(|cube| cube.unwrap())
                        .collect();

                    let counts = map_to_counts(&adjacent_cubes);
                    println!("{:?}", counts);

                    new_plane[grid_i][row_i][cube_i] = match counts.get(&Cube::A) {
                        Some(&active_cnt) => match cube {
                            Cube::A => {
                                if active_cnt == 2 || active_cnt == 3 {
                                    Cube::A
                                } else {
                                    Cube::I
                                }
                            }
                            Cube::I => {
                                if active_cnt == 3 {
                                    Cube::A
                                } else {
                                    Cube::I
                                }
                            }
                        },
                        _ => cube,
                    };
                }
            }
        }
        for grid in &new_plane {
            for row in grid {
                println!("{:?}", row);
            }
            println!("-----------------");
        }
        new_plane
    });

    let mut count = 0;
    for grid in plane {
        for row in grid {
            for cube in row {
                if cube == Cube::A {
                    count += 1;
                }
            }
        }
    }

    println!("{:?}", count);
}

fn main() {
    let file = read_to_string("puzzle.txt");
    match file {
        Ok(file) => {
            let cycles = 6;
            let initial_grid_size = split_lines(&file).len();
            let max_grid_size = initial_grid_size + cycles * 2;
            let max_z_height = 1 + cycles * 2;
            let plane_dimensions = [max_z_height, max_grid_size, max_grid_size];
            let mut plane: Vec<Vec<Vec<Cube>>> = create_empty_plane(&plane_dimensions);

            for (line_i, line) in split_lines(&file).iter().enumerate() {
                for (c_i, c) in line.chars().enumerate() {
                    plane[cycles][line_i + cycles][c_i + cycles] = match c {
                        '#' => Cube::A,
                        _ => Cube::I,
                    }
                }
            }

            part_1(&plane, &plane_dimensions, cycles);

            // for row in &plane[0] {
            //     println!("{:?}", row);
            // }
            // println!("-----------------");
            // println!("{}", max_grid_size);
        }
        Err(_) => println!("There's something wrong with the input file!"),
    }
}
