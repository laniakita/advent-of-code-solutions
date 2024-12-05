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
