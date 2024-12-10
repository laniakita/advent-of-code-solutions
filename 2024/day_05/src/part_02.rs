use crate::part_01::Prints;

/* fix_update function (declared in part_01.rs)
 *
 * 0. Loop through invalid update,
 * 1. save the current usize into a var
 * 2. save the next usize into a var
 * 3. create a experimental instruction pair (e.g. AA|BB),
 * 4. create an experiment inverse instruction pair (e.g., BB|AA),
 *    if we find the inverse of the experimental pair in the instruction_set,
 *    we can ammend the bad_update vec, by updating the current usize at 
 *    index i, to the next usize (cached in next) and vice-versa, updating the
 *    next usize at index i+1, with the current usize (cached in curr). 
 * 5. Once the loop completes, we can check if our "ammended" new_update passes
 *    validation, if it doesn't we'll run fix_update again, with the new_update
 *    as input. Otherwise, we can return a valid new_update
 *
 * pub fn fix_update(&self, update: &Vec<usize>) -> Vec<usize> {
 *       let mut new_update = update.to_vec();
 *
 *       for i in 0..new_update.len() - 1 {
 *           let curr = new_update[i];
 *           let next = new_update[i + 1];
 *
 *           let curr_ins_pair = format!("{}|{}", curr, next);
 *           let flipped_ins_pair = format!("{}|{}", next, curr);
 *
 *           // find the inverse pairs in the bad_new_update, then fix the update to correct
 *           if self.instruction_set.contains(&flipped_ins_pair) {
 *               println!("Invalid: {curr_ins_pair} has inverse rule {flipped_ins_pair}");
 *               println!("Ammending ... ");
 *               new_update[i] = next;
 *               new_update[i + 1] = curr;
 *           }
 *       }
 *
 *       if !self.is_valid_print(&new_update) {
 *           self.fix_update(&new_update)
 *       } else {
 *           new_update
 *       }
 *   }
 *
 * */

pub fn part_02(input: &str) {
    let prints = Prints::build(input);
    let mut bad_updates: Vec<Vec<usize>> = Vec::new();

    for update in prints.print_updates.iter() {
        // push all invalid updates to the bad_updates vec
        if !prints.is_valid_print(&update) {
            bad_updates.push(update.to_vec());
        }
    }

    let mut bad_count = 0; // debug var
    let mut fixed_count = 0; // debug var
    let mut mid_count = 0; // var of interest

    for update in bad_updates {
        bad_count += 1;
        println!("attempting to fix: {update:?}");
        let fixed = prints.fix_update(&update);
        println!("validating: {fixed:?}");

        if prints.is_valid_print(&fixed) {
            println!("fixed!");
            fixed_count += 1;
            mid_count += Prints::find_middle_page(&fixed);
        } else {
            println!("unable to fix :(")
        }
    }
    println!("{fixed_count}/{bad_count}"); // debug to check success rate
    println!("{mid_count}");
}
