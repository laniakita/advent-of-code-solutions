use std::{env, process};
use day_03::Config;

fn main() {
    // ex: args = "./day_02/input.txt"

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });
    
    println!("using {} as input", config.file_path);

    if let Err(e) = day_03::part_01::run(config) {
        eprint!("Application error: {e}");
        process::exit(1);
    }
}
