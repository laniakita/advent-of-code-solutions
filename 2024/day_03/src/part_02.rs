use regex::Regex;
use std::{error::Error, fs};

use crate::part_01::mul_validator;

pub fn mul_validator_complex(corrupted_string: &str) {

    let re = Regex::new(r"(do\(\)|don't\(\))|(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let instructions: Vec<&str> = re.find_iter(corrupted_string).map(|m| m.as_str()).collect();

    let mut is_enabled: bool = true;

    let mut total_sum_2: i32 = 0;
    for ins in instructions.iter() {
        // if do() => multiply
        // if don't() => don't multiply
        if ins == &"do()" {
            is_enabled = true;
        } else if ins == &"don't()" {
            is_enabled = false;
        } else {
            if is_enabled {
                let res = mul_validator(ins);
                total_sum_2 += res;
            }
        }
    }
    println!("{total_sum_2}");
}

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    mul_validator_complex(&contents);

    Ok(())
}
