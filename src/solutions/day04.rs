use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let filepath = "inputs/day04.txt";
    let ans = xmas_count(filepath);
    // let ans = count_xmas(xmas_array);
    println!("total xmas found: {}", ans);
}

fn xmas_count(fp: &str) -> u32 {
    let restructured: Vec<Vec<char>> = match create_2d_array(fp) {
        Ok(grid) => grid,
        Err(_) => panic!("coudn't process file"),
    };
    let ans: u32 = count_xmas(restructured);
    ans
}

fn create_2d_array(fp: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let f = File::open(fp)?;
    let br = BufReader::new(f);
    // let mut ans: Vec<Vec<char>> = Vec::new();
    // for l in br.lines() {
    //     let l = l?;
    //     let vecarr: Vec<char> = l.chars().collect();
    //     ans.push(vecarr);
    // }
    // Ok(ans)
    Ok(br
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>())
}

fn count_xmas(grid: Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 'X' {
                for chosen_dir in 0..8 {
                    let xmas_found = check_word_in_direction(&grid, row, col, 0, chosen_dir);
                    if xmas_found > 0 {
                        count += xmas_found;
                    }
                }
            }
        }
    }
    count
}

fn check_word_in_direction(grid: &Vec<Vec<char>>, x: usize, y: usize, cur_idx: usize, dir: usize) -> u32 {
    let target = ['X', 'M', 'A', 'S'];
    let directions = [
        (0, 1),
        (1, 0),
        (0, usize::MAX),
        (usize::MAX, 0),
        (1, 1),
        (1, usize::MAX),
        (usize::MAX, 1),
        (usize::MAX, usize::MAX),
    ];
    if grid[x][y] == target[cur_idx] && cur_idx == 3 {
        return 1;
    }
    if grid[x][y] != target[cur_idx] || cur_idx > 2 {
        return 0;
    }
    let mut found = 0;
    let (r, c) = directions[dir];
    let a;
    let b;
    if r == usize::MAX {
        a = x.checked_sub(1);
    } else {
        a = Some(x + r);
    }
    if c == usize::MAX {
        b = y.checked_sub(1);
    } else {
        b = Some(y + c);
    }
    if let (Some(new_a), Some(new_b)) = (a, b) {
        if new_a >= grid.len() || new_b >= grid[0].len() {
            return 0;
        }
        found += check_word_in_direction(grid, new_a, new_b, cur_idx + 1, dir);
    }
    found
}

/***
*/

/*** PART 01
help her with her word search (your puzzle input). She only has to find one word: XMAS.

word search allows words to be horizontal, vertical, diagonal,
written backwards, or even overlapping other words.

don't merely need to find one instance of XMAS - you need to find all of them.
*/
