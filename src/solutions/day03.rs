use std::fs::File;
use std::io::BufRead;

pub fn run() {
    let filepath = "inputs/day03.txt";
    let count = decode_mul(filepath);
    println!("total decoded value: {}", count);
    let dodontcount = do_dont_mul(filepath);
    println!("do don't mul decoded value: {}", dodontcount);
}

fn do_dont_mul(fp: &str) -> u32 {
    let f = File::open(fp).unwrap();
    let bufreader = std::io::BufReader::new(f);
    let mut total = 0;
    let mut enabled = true;
    for line in bufreader.lines() {
        let line = line.unwrap();
        let mut i = 0;
        while i < line.len() {
            if i + 7 <= line.len() && &line[i..i + 7] == "don't()" {
                enabled = false;
                i += 7;
                continue;
            } else if i + 4 <= line.len() && &line[i..i + 4] == "do()" {
                enabled = true;
                i += 4;
                continue;
            } else if i + 8 <= line.len() && &line[i..i + 4] == "mul(" && enabled {
                // i+8 because at min, it needs to be mul(x,x)
                i += 4;
                // similar logic below to fn decode_mul()
                if let Some(last_idx) = line[i..].find(")") {
                    let possible = &line[i..i + last_idx];
                    if let Some(comma_idx) = possible.find(",") {
                        let a = &possible[..comma_idx];
                        let b = &possible[comma_idx + 1..];
                        if a.len() >= 1 && a.len() <= 3 && b.len() >= 1 && b.len() <= 3 {
                            if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                                total += a * b;
                                i += 4;
                                continue;
                            }
                        }
                    }
                }
                continue;
            }
            i += 1;
        }
    }
    total
}

fn decode_mul(fp: &str) -> u32 {
    let file = File::open(fp).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut count: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        for l in line.split("mul(") {
            if let Some(last_idx) = l.find(")") {
                let possible = &l[0..last_idx];
                if let Some(comma_idx) = possible.find(",") {
                    let a = &possible[0..comma_idx];
                    let b = &possible[comma_idx + 1..];
                    if a.len() >= 1 && a.len() <= 3 && b.len() >= 1 && b.len() <= 3 {
                        if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                            count += a * b;
                        }
                    }
                }
            }
        }
    }
    count
}

/*** PART 02
2 new instructions
- do() enables future mul instructions
- don't() disables future mul instructions
only the most recent do() or don't() instruction applies
at the beginning of the program, mul instructions are enabled

e.g. xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    - mul(5,5) and mul(11,8) are disabled due to don't() prior
*/

/*** PART 01
instructions in day03.txt are jumbled up
goal of program is to multiply some numbers
it does that with instructions like mul(X,Y) where X and Y are each 1-3 digit numbers
    e.g. mul(44,46) -> 44 x 46 = 2024
day03.txt includes many invalid characters that should be ignored,
    even if they look like part of a mul instruction
    e.g. do nothing in sequences like these -> mul(4*, mul(6,9!, ?(12,34) or mul ( 2 , 4 )
another e.g. xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    the real mul instructions from the above are mul(2,4) + mul(5,5) + mul(11,8) + mul(8,5)
    answer -> 161
*/
