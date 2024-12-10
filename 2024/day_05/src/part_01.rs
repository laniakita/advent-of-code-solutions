use std::{collections::HashSet, usize};

pub struct Prints {
    pub instruction_set: HashSet<String>,
    pub print_updates: Vec<Vec<usize>>,
}

impl Prints {
    pub fn build(input: &str) -> Self {
        let mut instruction_set: HashSet<String> = HashSet::new();
        // temporay vec to hold each update strings
        let mut section_02: Vec<String> = Vec::new(); 
        let mut should_switch: bool = false;

        for line in input.lines() {
            if line == "" {
                should_switch = true;
                continue;
            }

            if !should_switch {
                instruction_set.insert(line.to_string());
            } else {
                section_02.push(line.to_string());
            }
        }

        let print_updates: Vec<Vec<usize>> = section_02
            .iter()
            .map(|instruction_set| {
                instruction_set
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        Self {
            instruction_set,
            print_updates,
        }
    }
    /*
     * 0. Loop through update, 
     * 1. create a experimental instruction pair (e.g. AA|BB),
     * 2. create an experiment inverse instruction pair (e.g., BB|AA),
     *    if we find the inverse of the experimental pair in the instruction_set,
     *    we can assume the update is invalid, as the experimental instruction pair 
     *    doesn't exist in the instruction_set
     * 3. For edge cases where neither the experimental pair or the inverse pair exists
     *    in the instruction_set, we just assume the update is valid,
     *    assuming a partial match, and continuing the loop
     * 4. if the loop completes without returning "false" (invalid condition),
     *    then we can return "true", as the update has no invalid pairs.
     **/
    pub fn is_valid_print(&self, update: &Vec<usize>) -> bool {
        for i in 0..update.len() - 1 {
            let test_ins_pair = format!("{}|{}", update[i], update[i + 1]);
            let test_flipped_ins_pair = format!("{}|{}", update[i + 1], update[i]);

            if self.instruction_set.contains(&test_flipped_ins_pair) {
                println!("Invalid: {test_ins_pair} has inverse rule {test_flipped_ins_pair}");
                return false;
            }

            if !self.instruction_set.contains(&test_ins_pair) {
                println!("OK: {test_ins_pair} doesn't exist");
                continue;
            }

            println!("OK: {test_ins_pair} exists as {i} in {}", update.len() - 2);
        }

        true
    }

    pub fn fix_update(&self, update: &Vec<usize>) -> Vec<usize> {
        let mut new_update = update.to_vec();

        for i in 0..new_update.len() - 1 {
            let curr = new_update[i];
            let next = new_update[i + 1];

            let curr_ins_pair = format!("{}|{}", curr, next);
            let flipped_ins_pair = format!("{}|{}", next, curr);

            // find the inverse pairs in the bad_new_update, then fix the update to correct
            if self.instruction_set.contains(&flipped_ins_pair) {
                println!("Invalid: {curr_ins_pair} has inverse rule {flipped_ins_pair}");
                println!("Ammending ... ");
                new_update[i] = next;
                new_update[i + 1] = curr;
            }
        }

        if !self.is_valid_print(&new_update) {
            self.fix_update(&new_update)
        } else {
            new_update
        }
    }

    pub fn find_middle_page(update: &Vec<usize>) -> usize {
        // WARN: assumes update is odd in length
        let middle_digit = ((update.len() / 2) as f32).floor();
        let middle_digit: usize = middle_digit as usize;
        let middle_page = update[middle_digit];

        println!("{} {:?} {}", update.len(), update, middle_page);

        let mid_count = middle_page;
        mid_count
    }
}

pub fn part_01(input: &str) {
    let prints = Prints::build(input);

    let mut good_updates: Vec<Vec<usize>> = Vec::new();

    for update in prints.print_updates.iter() {
        if prints.is_valid_print(&update) {
            good_updates.push(update.to_vec());
        }
    }

    // find the middle page number of each update, then add to running sum total: mid_count
    let mut mid_count = 0;
    for update in good_updates {
        mid_count += Prints::find_middle_page(&update);
    }

    println!("{mid_count}");
}


