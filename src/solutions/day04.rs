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
    // let ans: u32 = count_xmas(restructured);
    let ans: u32 = count_mas(restructured);
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

fn count_mas(grid: Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;
    let rows = grid.len();
    if rows < 3 { return 0; }
    let cols = grid[0].len();
    if cols < 3 { return 0; } 

    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if grid[row][col] == 'A' {
                if check_diagonal_patterns(&grid, row, col) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_diagonal_patterns(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // Diagonal 1: Upper-left to lower-right
    let toplefttobotright_forward = check_mas_diagonal(grid, row, col, -1, -1, 1, 1);
    let toplefttobotright_backward = check_sam_diagonal(grid, row, col, -1, -1, 1, 1);
    
    // Diagonal 2: Upper-right to lower-left
    let toprighttobotleft_forward = check_mas_diagonal(grid, row, col, -1, 1, 1, -1);
    let toprighttobotleft_backward = check_sam_diagonal(grid, row, col, -1, 1, 1, -1);
    
    (toplefttobotright_forward || toplefttobotright_backward) && (toprighttobotleft_forward || toprighttobotleft_backward)
}

fn check_mas_diagonal(grid: &Vec<Vec<char>>, row: usize, col: usize, 
                      m_row_offset: i32, m_col_offset: i32, 
                      s_row_offset: i32, s_col_offset: i32) -> bool {
    // check for 'M'
    let m_row = match (row as i32).checked_add(m_row_offset) {
        Some(r) if r >= 0 && (r as usize) < grid.len() => r as usize,
        _ => return false,
    };
    
    let m_col = match (col as i32).checked_add(m_col_offset) {
        Some(c) if c >= 0 && (c as usize) < grid[0].len() => c as usize,
        _ => return false,
    };
    
    if grid[m_row][m_col] != 'M' {
        return false;
    }
    
    // check for 'S'
    let s_row = match (row as i32).checked_add(s_row_offset) {
        Some(r) if r >= 0 && (r as usize) < grid.len() => r as usize,
        _ => return false,
    };
    
    let s_col = match (col as i32).checked_add(s_col_offset) {
        Some(c) if c >= 0 && (c as usize) < grid[0].len() => c as usize,
        _ => return false,
    };
    
    grid[s_row][s_col] == 'S'
}

fn check_sam_diagonal(grid: &Vec<Vec<char>>, row: usize, col: usize, 
                      s_row_offset: i32, s_col_offset: i32, 
                      m_row_offset: i32, m_col_offset: i32) -> bool {
    // check for 'S'
    let s_row = match (row as i32).checked_add(s_row_offset) {
        Some(r) if r >= 0 && (r as usize) < grid.len() => r as usize,
        _ => return false,
    };
    
    let s_col = match (col as i32).checked_add(s_col_offset) {
        Some(c) if c >= 0 && (c as usize) < grid[0].len() => c as usize,
        _ => return false,
    };
    
    if grid[s_row][s_col] != 'S' {
        return false;
    }
    
    // check for 'M'
    let m_row = match (row as i32).checked_add(m_row_offset) {
        Some(r) if r >= 0 && (r as usize) < grid.len() => r as usize,
        _ => return false,
    };
    
    let m_col = match (col as i32).checked_add(m_col_offset) {
        Some(c) if c >= 0 && (c as usize) < grid[0].len() => c as usize,
        _ => return false,
    };
    
    grid[m_row][m_col] == 'M'
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

fn check_word_in_direction(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    cur_idx: usize,
    dir: usize,
) -> u32 {
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

/*** PART 02
find two MAS in the shape of an X. One way to achieve that is like this:
M.S
.A.
M.S
Irrelevant characters have again been replaced with .
in the above diagram. Within the X, each MAS can be written forwards or backwards.
*/

/*** PART 01
help her with her word search (your puzzle input). She only has to find one word: XMAS.

word search allows words to be horizontal, vertical, diagonal,
written backwards, or even overlapping other words.

don't merely need to find one instance of XMAS - you need to find all of them.
*/
