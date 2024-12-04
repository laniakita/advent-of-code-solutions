use std::{error::Error, fs};


pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");
    
    println!("{contents}");

    Ok(())
}
