use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn run() {
    let day_two_data = count_number_of_safe_reports("inputs/day02.txt");
    match day_two_data {
        Ok(ans) => println!("number of safe reports: {}", ans),
        Err(e) => println!("fail... {}", e),
    }
}

fn count_number_of_safe_reports(filename: &str) -> Result<i32, Error> {
    let mut safe_reports: i32 = 0;
    let file = File::open(filename)?;
    let bufreader = BufReader::new(file);
    for line in bufreader.lines() {
        let line = line.unwrap();
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        // determine if report is safe by checking if values only ascend or only descend
        if is_safe(&report) {
            // if report is safe, increment++
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

fn is_safe(report_check: &[i32]) -> bool {
    let value_diff: Vec<i32> = report_check
        .windows(2)
        .map(|compare_window| compare_window[1] - compare_window[0])
        .collect();
    if value_diff.iter().all(|x| x.abs() >= 1 && x.abs() <= 3) {
        return value_diff.iter().all(|&x| x > 0) || value_diff.iter().all(|x| *x < 0);
    }
    false
}

/*** PART 01
   figure out which reports are safe.
   a report only counts as safe if both of the following are true:
       1. The levels are either all increasing or all decreasing.
       2. Any two adjacent levels differ by at least one and at most three.

*/
