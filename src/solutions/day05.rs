use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Error;
use std::io::BufReader;
use std::io::BufRead;

type Graph = HashMap<u32, HashSet<u32>>;

pub fn run() {
    // let ans = parse_input("inputs/day05.txt");
    if let Ok((graph,updates)) = parse_input("inputs/day05.txt") {
        let ans = solve(graph,updates);
        println!("answer is {}",ans);
    } else {
        println!("error parsing");
    }
}

fn parse_input(fp: &str) -> Result<(Graph,Vec<Vec<u32>>), Error> {
    let file = File::open(fp)?;
    let br = BufReader::new(file);
    let mut lines = br.lines();
    let mut make_graph = true;
    let mut graph: Graph = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.trim().is_empty() {
            make_graph = false;
            continue;
        }
        if make_graph {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let a = parts[0].parse::<u32>().unwrap();
                let b = parts[1].parse::<u32>().unwrap();
                graph.entry(a).or_insert_with(HashSet::new).insert(b);
            }
        } else {
            let update: Result<Vec<u32>,_> = line.split(',')
                .map(|num| num.parse::<u32>())
                .collect();
            updates.push(update.unwrap());
        }
    }
    Ok((graph,updates))
}

fn is_valid_update(graph: &Graph, update: &[u32]) -> bool {
    for i in 0..update.len() {
        for j in i+1..update.len() {
            let a = update[i];
            let b = update[j];
            // if let Some(after_set) = graph.get(&a) {
            //     if after_set.contains(&b) {
            //         continue;
            //     }
            // }
            if let Some(after_set) = graph.get(&b) {
                if after_set.contains(&a) {
                    return false;
                }
            }
        }
    }
    true
}

fn solve(graph: Graph, updates: Vec<Vec<u32>>) -> u32 {
    let mut ans = 0;
    for update in &updates {
        if is_valid_update(&graph, update) {
            ans += update[update.len()/2];
        }
    }
    ans
}

/*** PART 01
 * X|Y notation means if both X and Y are to be produced as part of an update
 * X must be printed at some point b4 Y 
 * day05.txt input file has both page ordering
 * as well as the pages to produce in each update
 * e.g. 61|98 means if an update includes both 61 and 98, then 61 must
 * be printed at some point b4 98
 * directed graph?
 * grab the middle # of each update being printed
 *  only the correctly-ordered updates
 * going forward...checking for incorrect. so 88|92, 34|88 and 34|92....that gets too big
 * going backward...checking for incorrect. 25|47, then would have to check every one....still big
 * hashmap with set of i32 as values
 * going forward...checking for correct 92|88,
 */