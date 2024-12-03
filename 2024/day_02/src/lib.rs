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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    // todo:
    // 1. split contents string by new line (report)
    // 2. for each report, figure out if it's safe or unsafe:
    //   safe == levels are all descreasing by 1 or 2
    //   unsafe == levels increasing || descreasing > 2 || increasing and decreasing || staying the same
    // 3. find total number of safe reports

    // build array of reports + determine safety
    let mut safety_total = 0;
    let mut total_reports = 0;
    for report in contents.lines() {
        let report_line: Vec<i32> = report.split_whitespace().flat_map(|s| s.parse()).collect();
        let mut prev_val = 0;
        let mut delta_safety: bool = false;
        let mut direction_safety: bool = false;
        let mut prev_direction = 0; // -1 == decr || 0 == neutral || 1 == incr

        for i in 0..report_line.len() {
            let curr_val = report_line[i];
            let mut curr_direction = 0;

            if i > 0 {
                let delta = curr_val - prev_val;

                if delta == 0 {
                    println!("unsafe: encountered neutral delta ({delta})");
                    println!("{report_line:?}");
                    break;
                } else if (delta > 0 && delta <= 3) || (delta < 0 && delta >= -2) {
                    println!("safe: delta within safe operating limits ({delta})");
                    println!("{report_line:?}");
                    delta_safety = true;
                } else {
                    println!("unsafe: delta out of safe operating limits: ({delta})");
                    println!("{report_line:?}");
                    break;
                }

                if delta > 0 {
                    curr_direction = 1;
                } else if delta < 0 {
                    curr_direction = -1;
                } else {
                    curr_direction = 0;
                }

                if i > 1 {
                    if curr_direction == prev_direction {
                        direction_safety = true;
                    }
                }
            }

            // assign current to previous
            prev_val = curr_val;
            prev_direction = curr_direction;
        }
        total_reports += 1;
        if delta_safety == true && direction_safety == true {
            safety_total += 1;
        }
    }

    println!("{total_reports}");
    println!("{safety_total}");

    Ok(())
}

/*
// iterate through report, find curr_variance
 let mut prev_num = 0;
        let mut prev_variance = 0;
        let mut curr_variance = 0;
        let mut curr_safety: bool = false;
        let mut delta_safety: bool = false;

        for i in 0..report.len() {
            let curr_num = report[i];

            if i > 0 {

                curr_variance = prev_num - curr_num;

                // decreasing OK
                if curr_variance == 1 || curr_variance == 2 {
                    curr_safety = true;
                }
                // increasing OK
                else if curr_variance == -1 || curr_variance == -2 || curr_variance == -3 {
                    curr_safety = true;
                }

                // check increasing or decreasing in same direction
                if (prev_variance > 0 && curr_variance > 0)
                    || (prev_variance < 0 && curr_variance < 0)
                {
                    delta_safety = true;
                } else {
                    delta_safety = false;
                }
            }
            prev_variance = curr_variance;
            prev_num = curr_num;
        }

        total_reports += 1;
        if curr_safety == true && delta_safety == true {
            safety_total += 1
        }


for i in 0..report.len() {
    if i >= report.len() - 1 {
        break;
    }

    let prev_variance = curr_variance;

    let curr = report[i];
    let next = report[i + 1];

    curr_variance = curr - next;


}*/
