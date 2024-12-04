use core::str;
use std::{error::Error, fs, usize};

use regex::Regex;

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
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wordsearch {
    pub content: String,
    pub info: GameInfo,
}

impl Wordsearch {
    pub fn new(wordsearch: &String) -> Self {
        let mut cols = 0;
        let mut rows = 0;

        for line in wordsearch.lines() {
            rows += 1;
            if cols == 0 {
                cols = line.len();
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
        let rev_word = reverse_string(word);
        
        let formatted_forward_re = format!(r"");
        let reg_word = Regex::new(r"");
        

        
     
        
    }
}

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    //println!("{contents}");

    let game = Wordsearch::new(&contents);

    println!("info: {:?}", game.info);

    let search = "xmas";

    game.word_counter(search);

    Ok(())
}
