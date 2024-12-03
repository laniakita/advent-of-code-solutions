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

pub fn vec_str_to_ints(string_vec: Vec<String>) -> Vec<i32> {
    let mut int_vec = Vec::new();
    for s in string_vec {
        match s.parse::<i32>() {
            Ok(num) => int_vec.push(num),
            Err(_) => println!("Error parsing: {}", s),
        }
    }
    int_vec
}

pub struct IdLists {
    pub left: Vec<i32>,
    pub right: Vec<i32>,
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

        let left_strings: Vec<String> = ids_01.iter().filter(|e| e.len() > 0).cloned().collect();
        let right_strings: Vec<String> = ids_02.iter().filter(|e| e.len() > 0).cloned().collect();
        
        let left = vec_str_to_ints(left_strings);
        let right = vec_str_to_ints(right_strings);

        IdLists { left, right }
    }
}

pub fn find_distance(left_vec: Vec<i32>, right_vec: Vec<i32>) -> Result<Vec<i32>, &'static str> {
    if left_vec.len() != right_vec.len() {
        return Err("Lists are unequal!")
    }

    let mut distance_res: Vec<i32> = Vec::new();
    
    for n in 0..left_vec.len() {
        let res = left_vec[n] - right_vec[n];
        distance_res.push(res.abs());
    }

    Ok(distance_res)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let divided_lists = IdLists::divider(config);
    
    let mut sorted_left = divided_lists.left;
    let mut sorted_right = divided_lists.right;   
    sorted_left.sort();
    sorted_right.sort();

    let distance_res = find_distance(sorted_left, sorted_right);
    let distances = distance_res.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        vec![]
    });

    let mut total_distance: i32 = 0;
    
    for d in 0..distances.len() {
        total_distance += distances[d]
    }
    
    // part 01
    println!("total distance: {total_distance}");
    

    Ok(())
}

