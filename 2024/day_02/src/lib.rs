pub mod part_01;
pub mod part_02;

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
