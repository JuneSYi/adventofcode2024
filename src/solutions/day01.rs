use std::{fs,io};
use std::io::BufRead;
use std::io::Error;
use std::collections::HashMap;
use std::time::Instant;

pub fn run() {
    let start_time = Instant::now();

    // let loc_data = iterate_over_file("inputs/day01.txt").unwrap();
    let loc_data = reuse_buffer_over_file("inputs/day01.txt").unwrap();
    let sumofdiffdata = compare_diff_and_sum(&loc_data.0,&loc_data.1);
    println!("loc data {:?}",&loc_data.0);
    println!("loc data {:?}",&loc_data.1);
    println!("get abs diff of each location index from smallest to largest and sum total \
        ends up as -> : {}", sumofdiffdata);

    // create hashmap of loc_data.0 and loc_data.1
    let hm1 = create_hashmap_of_data(&loc_data.0);
    let hm2 = create_hashmap_of_data(&loc_data.1);

    // iterate through hm1 and check if key exists in hm2
    let hmsum = compare_and_find_new_sum(&hm1,&hm2);
    println!("total sum for problem set 2 is {}", hmsum);
    
    let duration = start_time.elapsed();
    println!("Execution time: {:?}", duration);
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

fn create_hashmap_of_data(vec_of_data: &Vec<i32>) -> HashMap<i32,i32> {
    let mut hmap: HashMap<i32,i32> = HashMap::new();
    for locdata in vec_of_data.into_iter() {
        // if !hmap.contains_key(locdata) {
        //     hmap.insert(locdata,1);
        // } else {
        //     let vals = *hmap.get(locdata).unwrap();
        //     hmap.insert(locdata,vals);
        // }
        if let Some(count) = hmap.get_mut(&locdata) {
            *count +=1;
        } else {
            hmap.insert(*locdata,1);
        }
        // hmap.entry(locdata).and_modify(|counter| *counter +=1).or_insert(1);
    }
    hmap
}

fn compare_and_find_new_sum(hm1: &HashMap<i32,i32>, hm2: &HashMap<i32,i32>) -> i32 {
    let mut res: i32 = 0;
    // iterate through hm1 and check if key exists in hm2
    for hm1key in hm1.keys() {
        if hm2.contains_key(hm1key) {
            // if true, multiply key by the value in hm2
            let num: i32 = {
                hm1key * hm2.get(hm1key).unwrap()
            };
            // then multiply that total to value in hm1
            let num_with_freq: i32 = num * hm1.get(hm1key).unwrap();
            // then append that to overall_total
            res += num_with_freq;
        }
    }
    res
}

fn reuse_buffer_over_file(filename: &str) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let f = fs::File::open(filename)?;
    let mut reader = io::BufReader::new(f);
    let mut loc_group_one = Vec::new();
    let mut loc_group_two = Vec::new();
    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        {
            let line = buf.trim_end();
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            loc_group_one.push(numbers[0]);
            loc_group_two.push(numbers[1]);
        }
        buf.clear();
    }
    loc_group_one.sort();
    loc_group_two.sort();
    Ok((loc_group_one,loc_group_two))
}

/*** PART 02
 * figure out exactly how often each number from the left list appears in the 
 * right list. Calculate a total similarity score by adding up each number in 
 * the left list after multiplying it by the number of times that number 
 * appears in the right list.
 */

/*** PART 01
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