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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SafeDirectionDeltas {
    Increasing,
    Decreasing,
}

pub fn filter_unsafe(reports: String) {
    let mut line_num = 0;
    let mut safe_num = 0;
    for report in reports.lines() {
        line_num += 1;
        let report_line: Vec<i32> = report.split_whitespace().flat_map(|s| s.parse()).collect();
        
        let mut prev_level = 0;

        let mut curr_direction: Option<SafeDirectionDeltas> = None;
        let mut prev_direction: Option<SafeDirectionDeltas> = None;
        

        for (i, level) in report_line.iter().enumerate() {
            
            let curr_level = level.clone();

            if i > 0 {
                let delta = level.abs_diff(prev_level);
                
                if (delta < 1) || (delta > 3) {
                    curr_direction = None;
                    break;
                }

                if curr_level > prev_level {
                    curr_direction = Some(SafeDirectionDeltas::Increasing);
                } else if curr_level < prev_level {
                    curr_direction = Some(SafeDirectionDeltas::Decreasing);
                } else {
                    curr_direction = None;
                    break;
                }
            }

            if i > 1 && curr_direction != prev_direction {
                curr_direction = None;
                break;
            }

            prev_level = curr_level;
            prev_direction = curr_direction.clone();
        }

        if (curr_direction == Some(SafeDirectionDeltas::Increasing)) || (curr_direction == Some(SafeDirectionDeltas::Decreasing)) {
            safe_num += 1;
            println!("[{line_num}]: SAFE! {report_line:?}");
        }
        
    }
    println!("total safe: {safe_num}");
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    filter_unsafe(contents);

    Ok(())
}

