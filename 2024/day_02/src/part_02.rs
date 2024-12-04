use crate::part_01::is_safe_report;
use crate::Config;
use std::{error::Error, fs};

pub fn main_filter(reports: String) {
    let mut safe_num = 0;
    let mut safe_num_damp = 0;

    for report in reports.lines() {
        let report_line: Vec<i32> = report.split_whitespace().flat_map(|s| s.parse()).collect();

        if is_safe_report(report_line.clone()) == true {
            safe_num += 1;
        }

        if is_safe_damp(report_line.clone()) == true {
            safe_num_damp += 1;
        }
    }

    println!("strictly safe: {safe_num}");
    println!("dampened safe: {safe_num_damp}");
}

pub fn is_safe_damp(report_line: Vec<i32>) -> bool {
    let mut dampened_reports: Vec<Vec<i32>> = Vec::new();

    // brute force: generate reports, from a single report, with varying i's removed
    for i in 0..report_line.len() {
        let mut damp_report = report_line.clone();
        damp_report.remove(i.into());
        dampened_reports.push(damp_report);
    }

    let mut safety_flag = false;

    // test generated reports, break at first success, otherwise keep iterating
    for report in dampened_reports.iter() {
        let safety = is_safe_report(report.to_vec());
        if safety {
            safety_flag = true;
            break;
        } else {
            safety_flag = false;
        }
    }

    // return safe or unsafe (true or false)
    safety_flag
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Should read");
    main_filter(contents);
    Ok(())
}
