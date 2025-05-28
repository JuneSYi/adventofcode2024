use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::Error;
use std::io::BufReader;
use std::io::Read;

pub fn run() {
    if let Ok((grid,start_pos)) = parse_map("inputs/day06.txt") {
        let ans = count_positions(grid, start_pos);
        println!("total positions covered {}",ans);
    }
}

fn parse_map(filepath: &str) -> Result<(Vec<Vec<char>>,Vec<usize>),Error> {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start = vec![0,0];
    let file = File::open(filepath)?;
    let br = BufReader::new(file);
    for (i,line) in br.lines().enumerate() {
        let line = line?;
        let mut row = Vec::new();
        for (j,ch) in line.chars().enumerate() {
            row.push(ch);
            if ch == '^' {
                start[0]=i;
                start[1]=j;
            }
        }
        map.push(row);
    }
    Ok((map,start))
}

fn count_positions(grid: Vec<Vec<char>>, guard_pos: Vec<usize>) -> u32 {
    let mut total = 0;
    let dirs = ['N','E','S','W'];
    let mut dir = 0;
    let mut in_bounds = true;
    let mut guard_pos = guard_pos;
    let mut visited: HashSet<(usize,usize)> = HashSet::new();
    while in_bounds {
        let x = guard_pos[0];
        let y = guard_pos[1];
        match dirs[dir] {
            'N' => {
                if x.checked_sub(1).is_none() {
                    in_bounds = false;
                } else if grid[x-1][y]=='#' {
                    dir+=1
                } else {
                    guard_pos[0]-=1;
                }
            },
            'E' => {
                if y+1 == grid[0].len() {
                    in_bounds = false;
                } else if grid[x][y+1]=='#' {
                    dir+=1
                } else {
                    guard_pos[1]+=1;
                }
            },
            'S' => {
                if x+1 == grid.len() {
                    in_bounds = false;
                } else if grid[x+1][y]=='#' {
                    dir+=1
                } else {
                    guard_pos[0]+=1;
                }
            },
            'W' => {
                if y.checked_sub(1).is_none() {
                    in_bounds = false;
                } else if grid[x][y-1]=='#' {
                    dir=0
                } else {
                    guard_pos[1]-=1;
                }
            },
            _ => panic!()
        }
        if !visited.contains(&(x,y)) {
            total+=1
        }
        visited.insert((x,y));
    }
    total
}

/*** PART 1
 * guard, represented by ^ only goes in direction its facing
 * any obstacle, it turns 90 degrees, and moves forward.
 * mark/count all positions it visits before it leaves
 * include starting position
 */