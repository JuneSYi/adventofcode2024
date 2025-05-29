use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::Lines;

pub fn run() {
    let part1ans = parse_and_calculate("inputs/day07.txt");
    println!("part 1 answer: {}", part1ans);
    let part2ans = parse_and_calculate_part2("inputs/day07.txt");
    println!("part 2 answer: {}", part2ans);
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
        } //else {
            // lines that dont work we try with concatenation for part 2
        //     if concatentation_works(test_val, &operator_vals) {
        //             ans += test_val;
        //         }
        // }
    }

    ans
}

fn parse_and_calculate_part2(fp: &str) -> u64 {
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
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        if can_reach_target(operator_vals[0], 1, &operator_vals, test_val) {
            ans += test_val;
        }
    }
    ans
}

fn can_reach_target(acc: u64, idx: usize, numbers: &Vec<u64>, target: u64) -> bool {
    if idx == numbers.len() {
        return acc == target;
    }

    let next_num = numbers[idx];

    if can_reach_target(acc + next_num, idx + 1, numbers, target) {
        return true;
    }

    if can_reach_target(acc * next_num, idx + 1, numbers, target) {
        return true;
    }

    let concat_str = format!("{}{}", acc, next_num);
    if let Ok(concat_val) = concat_str.parse::<u64>() {
        if can_reach_target(concat_val, idx + 1, numbers, target) {
            return true;
        }
    }

    false
}

fn concatentation_works(
    test_value: u64,
    operators: &Vec<u64>,
) -> bool {
    for i in 1..operators.len() {
        let left_vec: Vec<u64> = operators[..i].to_vec();
        let mut left: Vec<String> = Vec::new();
        get_possibles(&left_vec, test_value, &mut left, 0, 0, 0);
        let right_vec: Vec<u64> = operators[i..].to_vec();
        let mut right: Vec<String> = Vec::new();
        get_possibles(&right_vec, test_value, &mut right, 0, 0, 0);
        for left_pos in &left {
            for right_pos in &right {
                let concat_match = left_pos.to_string() + right_pos;
                if concat_match == test_value.to_string() {
                    return true;
                }
            }
        }

    }
    
    false
}

fn get_possibles(
    operators: &Vec<u64>, 
    test_value: u64,
    possibles: &mut Vec<String>,
    cur_idx: usize,
    mut cur_total: u64,
    add_or_mult: u32,
) {
    if add_or_mult == 0 {
        cur_total += operators[cur_idx];
    } else {
        cur_total *= operators[cur_idx];
    }

    if cur_total > test_value {
        return;
    } else if cur_idx == operators.len()-1 {
        let tvstring = test_value.to_string();
        let cur_string = cur_total.to_string();
        if tvstring.ends_with(&cur_string) || tvstring.starts_with(&cur_string) {
            possibles.push(cur_string);
        }
        return;
    }
    // let mut res1 = get_possibles(&operators, test_value, possibles, cur_idx+1, cur_total, 0);
    // let mut res2 = get_possibles(&operators, test_value, possibles, cur_idx+1, cur_total, 1);
    get_possibles(&operators, test_value, possibles, cur_idx+1, cur_total, 0);
    get_possibles(&operators, test_value, possibles, cur_idx+1, cur_total, 1);
    // res1.extend(res2);
    // return res1
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