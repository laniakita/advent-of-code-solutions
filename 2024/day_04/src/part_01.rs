use std::{error::Error, fs, usize};

pub fn valid_coord(grid: &Vec<Vec<char>>, u: &i32, v: &i32) -> bool {
    //println!("bounds check ({u}, {v})");
    *u >= 0 && *u < grid[0].len() as i32 && *v >= 0 && *v < grid.len() as i32
}

pub fn check_word(
    index: i32,
    grid: &Vec<Vec<char>>,
    word: &str,
    u: &i32,
    v: &i32,
    dir_u: &i32,
    dir_v: &i32,
) -> bool {
    if index as usize == word.len() {
        println!("found!");
        return true;
    }

    // all valid coords should be okay as usize
    if valid_coord(grid, u, v) {

        if word.chars().nth(index as usize).unwrap() == grid[*u as usize][*v as usize] {
            println!(
                "yes: ({u}, {v}) is valid with char: ({})",
                grid[*u as usize][*v as usize]
            );

            let search_u = u + dir_u;
            let search_v = v + dir_v;

            println!("trying next at ({search_u}, {search_v})");
            return check_word(index + 1, grid, word, &search_u, &search_v, dir_u, dir_v);
        }
    }

    false
}

fn word_count(input: &str, word: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|m| m.trim().chars().collect()).collect();

    let mut count = 0;

    let mut directions: Vec<(i32, i32)> = Vec::new();

    for l in -1..2 {
        for r in -1..2 {
            directions.push((l, r));
        }
    }

    println!("{directions:?}");

    for u in 0..grid.len() {
        for v in 0..grid[0].len() {
            let char = grid[u][v];
            println!("checking: {char} at ({u}, {v})");
            for (dir_v, dir_u) in directions.iter() {
                let u_i: i32 = u.try_into().unwrap();
                let v_i: i32 = v.try_into().unwrap();

                if check_word(0, &grid, word, &u_i, &v_i, &dir_u, &dir_v) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count)
}

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    word_count(&contents, "XMAS");

    Ok(())
}
