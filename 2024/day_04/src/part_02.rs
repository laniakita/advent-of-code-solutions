use std::{error::Error, fs};

use crate::part_01::valid_coord;

pub fn mas_x_count(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|m| m.trim().chars().collect()).collect();
    let mut count = 0;

    for u in 0..grid.len() {
        for v in 0..grid[0].len() {
            // look for A, because it forms the center core of the MAS cross
            if grid[u][v] == 'A' {
                // create the diagonal points as i32, from where we found A
                let test_ul: i32 = (u as i32) - 1;
                let test_ur: i32 = (u as i32) + 1;
                let test_vl: i32 = (v as i32) - 1;
                let test_vr: i32 = (v as i32) + 1;
                
                // test the diagonal vertices to see if they're valid coords
                if valid_coord(&grid, &test_ul, &test_vr)
                    && valid_coord(&grid, &test_ur, &test_vl)
                    && valid_coord(&grid, &test_ul, &test_vl)
                    && valid_coord(&grid, &test_ur, &test_vr)
                {
                    // since they're all valid, we can construct the diagonal lines MAS || SAM
                    let left_diag =
                        format!("{}{}{}", grid[u - 1][v + 1], grid[u][v], grid[u + 1][v - 1]);
                    let right_diag =
                        format!("{}{}{}", grid[u - 1][v - 1], grid[u][v], grid[u + 1][v + 1]);
                    if left_diag == "MAS" || left_diag == "SAM" {
                        if right_diag == "MAS" || right_diag == "SAM" {
                            // since both are valid diagonals, we can call this a MAS cross!
                            println!("found cross! left: {}, right: {}", left_diag, right_diag);
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{count}");
}

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    mas_x_count(&contents);

    Ok(())
}
