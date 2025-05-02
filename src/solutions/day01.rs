use std::{fs,io};
use std::io::BufRead;
use std::io::Error;

pub fn run() {
    let (loc_data1, loc_data2) = iterate_over_file("inputs/day01.txt").unwrap();

    let sumofdiffdata = compare_diff_and_sum(&loc_data1,&loc_data2);
    // println!("loc data {:?}",loc_data1);
    // println!("loc data {:?}",loc_data2);
    println!("get abs diff of each location index from smallest to largest and sum total \
        ends up as -> : {}", sumofdiffdata)
}

fn compare_diff_and_sum(loc1: &Vec<i32>, loc2: &Vec<i32>) -> i32 {
    let mut sumtotal =0;
    for i in 0..loc1.len() {
        sumtotal += (loc1[i]-loc2[i]).abs();
    }
    sumtotal
}
fn iterate_over_file(filename: &str) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let f = fs::File::open(filename)?;
    let reader = io::BufReader::new(f);
    let mut loc_group_one = Vec::new();
    let mut loc_group_two = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        loc_group_one.push(numbers[0]);
        loc_group_two.push(numbers[1]);
    }
    loc_group_one.sort();
    loc_group_two.sort();
    Ok((loc_group_one,loc_group_two))
}

/*
    To find out, pair up the numbers and measure how far apart they are.
    Pair up the smallest number in the left list with the smallest number
    in the right list, then the second-smallest left number with
    the second-smallest right number, and so on.

    Within each pair, figure out how far apart the two numbers are; you'll
    need to add up all of those distances. For example, if you pair up a 3
    from the left list with a 7 from the right list, the distance apart is 4;
    if you pair up a 9 with a 3, the distance apart is 6.

    To find the total distance between the left list and the right list,
    add up the distances between all of the pairs you found. In the example
    above, this is 2 + 1 + 0 + 1 + 2 + 5, a total distance of 11!

    Your actual left and right lists contain many location IDs. What is the
    total distance between your lists?
     */