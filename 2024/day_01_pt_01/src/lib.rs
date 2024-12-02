use std::{error::Error, fs};

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please provide an input!");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub struct IdLists {
    pub left: Vec<String>,
    pub right: Vec<String>,
}

impl IdLists {
    pub fn divider(config: Config) -> IdLists {
        let contents = fs::read_to_string(config.file_path).expect("Should read");

        let combined_ids: Vec<&str> = contents.split_whitespace().collect();
        let mut ids_01: Vec<String> = vec![String::new(); 126];
        let mut ids_02: Vec<String> = vec![String::new(); 126];

        let mut idx = 0;
        for id in combined_ids {
            idx += 1;

            let id_copy = id.to_string();

            // odds == left list
            if idx % 2 == 1 {
                ids_01.push(id_copy);
            } else {
                ids_02.push(id_copy);
            }
        }

        let left: Vec<String> = ids_01.iter().filter(|e| e.len() > 0).cloned().collect();
        let right: Vec<String> = ids_02.iter().filter(|e| e.len() > 0).cloned().collect();

        IdLists { left, right }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let divided_lists = IdLists::divider(config);
    
    let ids_left = divided_lists.left;
    let ids_right = divided_lists.right;
    
    println!("left list: {ids_left:?}");
    println!("right list: {ids_right:?}");
    
    Ok(())
}

