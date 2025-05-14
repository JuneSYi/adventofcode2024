use std::fs::File;
use std::io::BufRead;

pub fn run() {
    let filepath = "inputs/day03.txt";
    let count = decode_mul(filepath);
    println!("total decoded value: {}", count);
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
