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
        let ans = solve(&graph,&updates);
        println!("answer is {}",ans);
        let ans_two = solve_part_two(&graph, &updates);
        println!("part two answer is {}",ans_two);
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

fn invalid_reordered(graph: &Graph, update: &[u32]) -> u32 {
    let mut reversed_graph: Graph = HashMap::new();
    for &node in update {
        reversed_graph.entry(node).or_insert_with(HashSet::new);
    }
    for (&before, afters) in graph {
        for &after in afters {
            if update.contains(&before) && update.contains(&after) {
                reversed_graph.entry(after).or_insert_with(HashSet::new).insert(before);
            }
        }
    }
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut temp_visited = HashSet::new();
    for &node in update {
        if !visited.contains(&node) {
            dfs(&reversed_graph, node, &mut visited, &mut temp_visited, &mut result);
        }
    }
    result[result.len() / 2]
}

fn dfs(graph: &Graph, node: u32, visited: &mut HashSet<u32>, temp_visited: &mut HashSet<u32>, ans: &mut Vec<u32>) {
    if visited.contains(&node) {
        return;
    }
    temp_visited.insert(node);
    if let Some(dependencies) = graph.get(&node) {
        for dep in dependencies {
            if !visited.contains(dep) && !temp_visited.contains(dep) {
                dfs(graph, *dep, visited, temp_visited, ans);
            }
        }
    }
    temp_visited.remove(&node);
    visited.insert(node);
    ans.push(node);
}
fn solve_part_two(graph: &Graph, updates: &Vec<Vec<u32>>) -> u32 {
    let mut sum_of_middle_elements = 0;
    
    for update in updates {
        if !is_valid_update(&graph, update) {
            sum_of_middle_elements += invalid_reordered(&graph, update);
        }
    }
    
    sum_of_middle_elements
}


fn solve(graph: &Graph, updates: &Vec<Vec<u32>>) -> u32 {
    let mut ans = 0;
    for update in updates {
        if is_valid_update(&graph, update) {
            ans += update[update.len()/2];
        }
    }
    ans
}

/*** PART 02
 * for the incorrectly ordered updates, use page ordering rules to put page #s in right order
 * what is new total
 */

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