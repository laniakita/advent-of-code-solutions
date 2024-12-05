use core::str;
use grid::*;
use std::{collections::HashSet, error::Error, fs, usize};

pub fn reverse_string(string: &str) -> String {
    let mut string_rev_vec_u8: Vec<u8> = Vec::new();

    for i in (0..string.len()).rev() {
        let u8_char = string.as_bytes()[i];
        string_rev_vec_u8.push(u8_char);
    }

    let sliced: &[u8] = &string_rev_vec_u8;

    let word_rev = String::from_utf8_lossy(sliced);
    word_rev.to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameInfo {
    pub rows: i32,
    pub cols: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wordsearch {
    pub content: String,
    pub info: GameInfo,
}

impl Wordsearch {
    pub fn new(wordsearch: &String) -> Self {
        let mut cols: i32 = 0;
        let mut rows: i32 = 0;

        for line in wordsearch.lines() {
            rows += 1;
            if cols == 0 {
                cols = line.len() as i32;
            }
        }

        Self {
            content: wordsearch.clone(),
            info: GameInfo { rows, cols },
        }
    }
    
    pub fn valid_coordinate(x: &i32, y: &i32, rows: usize, cols: usize) -> bool {
        (x.clone() >= 0) && (x.clone() < rows as i32) && (y.clone() >= 0) && (y.clone() < cols as i32)
    }

    pub fn find_word(
        index: &usize,
        word: &str,
        grid: Grid<char>,
        x: &i32,
        y: &i32,
        x_dir: &i32,
        y_dir: &i32,
    ) -> bool {

        let word_char = word.chars().nth(*index).unwrap();
        
        let new_index = index.clone();
        let new_word = word;
        let new_x = x + x_dir;
        let new_y = y + y_dir;
        let new_x_dir = x_dir.clone();
        let new_y_dir = y_dir.clone();
        

        if index == &word.len() {
            true
        } else if (Wordsearch::valid_coordinate(x, y, 140, 140)) && &word_char == grid.get(*x, *y).unwrap() {
            Wordsearch::find_word(&new_index, &new_word, grid, &new_x, &new_y, &new_x_dir, &new_y_dir)
        } else {
            false
        }
    }

    pub fn word_counter(&self, word: &str) {
        let cols_max = self.info.cols;
        let rows_max = self.info.rows;
        //let rev_word: HashSet<char> = word.chars().rev().collect();
        let word_chars: HashSet<char> = word.chars().collect();

        let mut grid_vec: Vec<char> = Vec::new();

        for line in self.content.lines() {
            for char in line.chars() {
                grid_vec.push(char);
            }
        }

        let grid = Grid::from_vec(grid_vec.clone(), cols_max as usize);

        // directions
        let x_dirs: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
        let y_dirs: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

        let mut count = 0;
        
        let index: usize = 0;
        
        

        for i in 0..rows_max {
            for j in 0..cols_max {
                for k in 0..x_dirs.len() {
                    if Wordsearch::find_word(&index, word, grid.clone(), &i, &j, &x_dirs[k], &y_dirs[k]) {
                        count +=1;
                        break;
                    }
                }
            }
        }

        /*
         *
         *
        for (i, row) in self.content.lines().enumerate() {
            for (j, col) in row.chars().enumerate() {

                if !word_chars.contains(&col) {
                    continue;
                }

                for (d_i, d_j) in directions.iter() {
                    let mut x: i32 = i as i32;
                    let mut y: i32 = j as i32;
                    let mut z = 0;

                    while (x >= 0  && x < rows_max ) && (y >= 0 && y < cols_max) && (z < word.len()) {
                        if grid[x as usize][y as usize] != word.chars().nth(z).unwrap() && grid[x as usize][y as usize] != word.chars().rev().nth(z).unwrap() {
                            break;
                        }
                        z +=1;
                        x += d_i;
                        y += d_j;

                    }

                    if z == word.len() {
                        count += 1;
                    }
                }



            }
        }


        */

        println!("{count}");
    }
}

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    //println!("{contents}");

    let game = Wordsearch::new(&contents);

    println!("info: {:?}", game.info);

    let search = "XMAS";

    game.word_counter(search);

    Ok(())
}
