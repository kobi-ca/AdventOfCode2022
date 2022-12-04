use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn part1() {
    let mut max = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let mut count: u64 = 0;
        for line in lines {
            let ip = line.unwrap();
            if ip.is_empty() {
                max = std::cmp::max(count, max);
                count = 0;
            } else {
                count += ip.parse::<u64>().unwrap();
            }
        }
    }
    println!("{}", max);
}

fn part2() {
    let mut v = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        let mut count: u64 = 0;
        for line in lines {
            let ip = line.unwrap();
            if ip.is_empty() {
                v.push(count);
                count = 0;
            } else {
                count += ip.parse::<u64>().unwrap();
            }
        }
    }
    v.sort_by(|a, b| b.cmp(a));
    let result: u64 = v.iter().take(3).sum();
    println!("{}", result);
}

fn main() {
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
