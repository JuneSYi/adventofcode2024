use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::Lines;

pub fn run() {
    let part1ans = parse_and_calculate("inputs/day07.txt");
    println!("part 1 answer: {}", part1ans);
}

fn parse_and_calculate(fp: &str) -> u64 {
    let mut ans = 0;
    let file = File::open(fp).unwrap();
    let br = BufReader::new(file);
    for line in br.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(':').collect();
        let test_val = parts[0].parse::<u64>().unwrap();
        let operator_vals: Vec<u64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u64>()
            .unwrap())
            .collect();
        if can_calibrate(test_val, &operator_vals, 0, 0, 0) {
            ans += test_val;
        }
    }

    ans
}

fn can_calibrate(
    test_value: u64, 
    operators: &Vec<u64>, 
    cur_idx: usize, 
    mut cur_total: u64, 
    add_or_mult: u32,
) -> bool {
    if add_or_mult == 0 {
        cur_total += operators[cur_idx];
    } else {
        cur_total *= operators[cur_idx];
    }
    
    if cur_total > test_value {
        return false;
    } else if cur_total == test_value {
        return true;
    }
    
    if cur_idx == operators.len()-1 {
        return false;
    }

    return can_calibrate(test_value, operators, cur_idx+1, cur_total, 0) || 
        can_calibrate(test_value, operators, cur_idx+1, cur_total, 1);
}

/*** PART 1
 * test values appear before colon on each line, determine whether 
 * remaining #s can be bombind w/operators to produce the test value
 *  - operators are always evaluated left to right; not according to
 *      precedence rules
 *  - numbers cant be re-arranged
 *  - 2 types of operators + and *
 * - 
 */