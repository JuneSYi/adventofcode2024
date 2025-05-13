use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn run() {
    let day_two_data = count_number_of_safe_reports("inputs/day02.txt");
    match day_two_data {
        Ok((safe_reports, safe_with_dampener)) => {
            println!("number of safe reports: {}", safe_reports);
            println!(
                "safe reports with problem dampener enabled: {}",
                safe_with_dampener
            );
        }
        Err(e) => println!("fail... {}", e),
    }
}

fn count_number_of_safe_reports(filename: &str) -> Result<(i32, i32), Error> {
    let mut safe_reports: i32 = 0;
    let mut safe_reports_with_problem_dampener: i32 = 0;
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
        if is_safe_with_problem_dampener(&report) {
            safe_reports_with_problem_dampener += 1;
        }
    }
    Ok((safe_reports, safe_reports_with_problem_dampener))
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

fn is_safe_with_problem_dampener(check_report: &[i32]) -> bool {
    if is_safe(check_report) {
        return true;
    }
    // let val_diff: Vec<i32> = check_report.windows(2).map(|x| x[1] - x[0]).collect();
    // let unsafe_reports: usize = val_diff
    //     .iter()
    //     .filter(|x| x.abs() < 0 || x.abs() > 3)
    //     .count();
    for i in 0..check_report.len() {
        let dampened_report: Vec<i32> = check_report
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, &x)| x)
            .collect();
        if is_safe(&dampened_report) {
            return true;
        }
    }
    false
}

/*** PART 01
   figure out which reports are safe.
   a report only counts as safe if both of the following are true:
       1. The levels are either all increasing or all decreasing.
       2. Any two adjacent levels differ by at least one and at most three.
*/
/*** PART 02
same rules apply as before, except if removing a single level from
an unsafe report would make it safe, the report instead counts as safe.
*/
