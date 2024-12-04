use std::{error::Error, fs};
use regex::Regex;

pub fn mul_validator(corrupted_string: &str) -> i32 {
    // invalid sequence examples:
    //   - mul(1*
    //   - mul(2,3!
    //   - ?(11,22)
    //   - mul ( 3 , 1 )
    // valid sequence examples: 
    //   - mul(1,2)
    //   - mul(3,3)
    //   - mul(11,22)mul(4,2)
    let re = Regex::new(r"(mul\((?<m1>[0-9]{1,3}),(?<m2>[0-9]{1,3})\))").unwrap();
    let instructions: Vec<(&str, &str)> = re.captures_iter(corrupted_string).map(|caps| {
        let m1 = caps.name("m1").unwrap().as_str();
        let m2 = caps.name("m2").unwrap().as_str();
        (m1, m2)
    }).collect();
    
    let mut total_sum = 0;

    for mul in instructions.iter() {
        let (m1_raw, m2_raw) = mul;
        let m1 = m1_raw.parse::<i32>();
        let m2 = m2_raw.parse::<i32>();
        let res = m1.unwrap() * m2.unwrap();
        total_sum += res;
    }

    println!("{total_sum}");
    total_sum
}

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");
    
    mul_validator(&contents);
    
    Ok(())
}
