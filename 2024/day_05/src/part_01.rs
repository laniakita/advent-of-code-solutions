use std::{collections::HashMap, error::Error, fs, usize, vec};

struct PrintOrder {
    vec_01: Vec<String>,
    order_map: PrintOrderMap,
    vec_02: Vec<Vec<usize>>,
}

struct PrintOrderMap {
    map: HashMap<usize, Vec<usize>>,
    middle_digit: usize,
}

impl PrintOrder {
    fn preprocess(input: &str) -> Self {
        let mut section_01: Vec<String> = Vec::new();
        let mut section_02: Vec<String> = Vec::new();
        let mut should_switch: bool = false;

        for line in input.lines() {
            if line == "" {
                should_switch = true;
                continue;
            }

            if !should_switch {
                section_01.push(line.to_string());
            } else {
                section_02.push(line.to_string());
            }
        }

        // pair map
        let mut pair_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for pair in section_01.iter() {
            let pairs: Vec<String> = pair.split("|").map(|m| m.to_string()).collect();
            let p1: &String = &pairs[0];
            let p2: &String = &pairs[1];

            let p1: usize = p1.parse().unwrap();
            let p2: usize = p2.parse().unwrap();
            let default_vec: Vec<usize> = Vec::new();
            let current: &Vec<usize> = pair_map.get(&p1).unwrap_or_else(|| &default_vec);

            let mut before: Vec<usize> = current.to_vec();
            before.push(p2);
            pair_map.insert(p1, before.to_vec());
        }

        // section_02 vecs
        let mut section_02_processed: Vec<Vec<usize>> = Vec::new();
        for line in section_02.iter() {
            let tmp_vec: Vec<usize> = line
                .split(",")
                .map(|m| m.parse::<usize>().unwrap())
                .collect();
            section_02_processed.push(tmp_vec);
        }

        Self {
            vec_01: section_01,
            order_map: PrintOrderMap {
                map: pair_map,
                middle_digit: 0,
            },
            vec_02: section_02_processed,
        }
    }
}
/*
fn search(index: usize, test_vec: &Vec<usize>, key_map_vec: &Vec<usize>, prev_key: &usize, curr_key: &usize) -> bool {
    if index == key_map_vec.len() {
        return true;
    }


    false
}
*/
fn part_01(input: &str) {
    let print_order: PrintOrder = PrintOrder::preprocess(input);
    let order_map = print_order.order_map.map;

    //println!("{:?}", print_order.order_map.map);
    
    let mut print_count = 0;
    let mut good_orders: Vec<Vec<usize>> = Vec::new();

    for i in 0..print_order.vec_02.len() {
        let line = print_order.vec_02[i].to_vec();

        for j in 1..line.len() {
           

            let prev_key = line[j - 1];
            let curr_key = line[j];

            let map_res = order_map.get(&prev_key);

            if map_res.is_some() {
                println!("key: {prev_key}: {map_res:?}");
                let ok_res = map_res.unwrap();
                if ok_res.contains(&curr_key) {
                    println!("{prev_key} is before {curr_key}");
                    if j == line.len() - 1 {
                        println!("CAN PRINT!");
                        good_orders.push(line.to_vec());
                        print_count += 1;
                    }
                }
            }
        }
    }

    println!("{print_count}");
    
    let mut mid_count = 0;
    for order in good_orders {
        
        let middle_digit = ((order.len() / 2) as f32).floor();
        let middle_digit: usize = middle_digit as usize;
        
        
        let middle_page = order[middle_digit];

        println!("{} {:?} {}", order.len(), order, middle_page);

        mid_count += middle_page;

    }

    println!("{mid_count}");

}

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    part_01(&contents);

    Ok(())
}
