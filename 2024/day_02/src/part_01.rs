use std::{error::Error, fs};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SafeDirectionDeltas {
    Increasing,
    Decreasing,
}

pub fn filter_unsafe(reports: String) {
    let mut safe_num = 0;

    for report in reports.lines() {
        let report_line: Vec<i32> = report.split_whitespace().flat_map(|s| s.parse()).collect();

        if is_safe_report(report_line.clone()) == true {
            safe_num += 1;
        }
    }

    println!("{safe_num}");
}

pub fn is_safe_report(report: Vec<i32>) -> bool {
    // safe:
    //   - decreasing (consistently) by 1 or 2
    //   - increasing (consistently) by 1, 2, or 3
    // unsafe:
    //   - increasing then decreasing (and vice-versa)
    //   - no change between level

    let mut curr_direction: Option<SafeDirectionDeltas> = None; // default: unsafe

    for i in 1..report.len() {
        let curr_level = report[i];
        let prev_level = report[i - 1];
        let prev_direction = curr_direction.clone();

        let delta = curr_level.abs_diff(prev_level);

        if (delta < 1) || (delta > 3) {
            // unsafe (beyond safe limits)
            curr_direction = None;
            break;
        }

        if curr_level > prev_level {
            // safe (increase)
            curr_direction = Some(SafeDirectionDeltas::Increasing);
        } else if curr_level < prev_level {
            // safe (decrease)
            curr_direction = Some(SafeDirectionDeltas::Decreasing);
        } else {
            // unsafe (no change)
            curr_direction = None;
            break;
        }

        if i > 1 && curr_direction != prev_direction {
            // unsafe (inconsistent)
            curr_direction = None;
            break;
        }
    }

    if (curr_direction == Some(SafeDirectionDeltas::Increasing))
        || (curr_direction == Some(SafeDirectionDeltas::Decreasing))
    {
        true
    } else {
        false
    }
}

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");

    filter_unsafe(contents);

    Ok(())
}
