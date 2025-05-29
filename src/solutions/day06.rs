use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::Error;
use std::io::BufReader;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0,1),
            Direction::South => (1,0),
            Direction::West => (0,-1),
        }
    }
    fn move_forward(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0.wrapping_sub(1), pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West => (pos.0, pos.1.wrapping_sub(1)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn try_move(&self, dir: &Direction, grid: &Vec<Vec<char>>) -> Option<Position> {
        let (r_offset, c_offset) = dir.get_offset();
        let next_r = if r_offset < 0 {
            self.row.checked_sub(r_offset.abs() as usize)
        } else {
            self.row.checked_add(r_offset as usize)
        };
        let next_c = if c_offset < 0 {
            self.col.checked_sub(c_offset.abs() as usize)
        } else {
            self.col.checked_add(c_offset as usize)
        };
        if let (Some(r),Some(c)) = (next_r, next_c) {
            if r < grid.len() && c < grid[0].len() {
                return Some(Position { row: r, col: c });
            }
        }
        None
    }
    fn is_obstacle(&self, grid: &Vec<Vec<char>>) -> bool {
        grid[self.row][self.col] == '#'
    }
}

pub fn run() {
    if let Ok((grid,start_pos)) = parse_map("inputs/day06.txt") {
        let cloned_start_pos = start_pos.clone();
        let ans = count_positions(&grid, start_pos);
        println!("total positions covered {}",ans);
        let result = count_loop_positions(&grid, (cloned_start_pos[0],cloned_start_pos[1]), Direction::North);
        println!("Part 2: {}", result);
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

fn has_loop(
    grid: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_dir: Direction,
    obstruction: (usize, usize),
) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut visited = HashSet::new();

    loop {
        if pos.0 >= rows || pos.1 >= cols {
            return false;
        }

        if !visited.insert((pos, dir)) {
            return true;
        }

        let forward_pos = dir.move_forward(pos);
        if forward_pos.0 >= rows || forward_pos.1 >= cols {
            return false;
        } else if grid[forward_pos.0][forward_pos.1] == '#' || forward_pos == obstruction {
            dir = dir.turn_right();
        } else {
            pos = forward_pos;
        }
    }
}

fn count_loop_positions(grid: &Vec<Vec<char>>, start_pos: (usize, usize), start_dir: Direction) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '.' && (i, j) != start_pos {
                if has_loop(grid, start_pos, start_dir, (i, j)) {
                    count += 1;
                }
            }
        }
    }
    count
}

// fn count_infinite_loop_positions_attempt_one(grid: &Vec<Vec<char>>, guard_pos: Vec<usize>) -> u32 {
//     let mut blockers = 0;
//     let mut position = Position { row: guard_pos[0], col: guard_pos[1] };
//     let mut direction = Direction::North;
//     let mut visited = HashSet::new();
//     let mut inf_blockers_pos = HashSet::new();
//     visited.insert((position.row,position.col));

//     loop {
//         if let Some(next_pos) = position.try_move(&direction, &grid) {
//             if next_pos.is_obstacle(&grid) {
//                 direction = direction.turn_right();
//             } else {
//                 if visited.contains(&(next_pos.row,next_pos.col)) {
//                     if let Some((row_block,col_block)) = position.possible_inf(&direction, &grid) {
//                         if !inf_blockers_pos.contains(&(row_block,col_block)) {
//                             blockers +=1;
//                             inf_blockers_pos.insert((row_block,col_block));
//                         }
//                     }
//                 }
//                 position = next_pos;
//                 visited.insert((position.row,position.col));
//             }
//         } else {
//             break;
//         }
//     }

//     blockers
// }

fn count_positions(grid: &Vec<Vec<char>>, guard_pos: Vec<usize>) -> u32 {
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

/*** PART 2
 * place a new obstruction in such a way that guard will get stuck in a loop
 * find all possible positions and return total
 */

/*** PART 1
 * guard, represented by ^ only goes in direction its facing
 * any obstacle, it turns 90 degrees, and moves forward.
 * mark/count all positions it visits before it leaves
 * include starting position
 */