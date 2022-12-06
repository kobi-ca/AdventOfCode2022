use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use iterwindows::IterArrayWindows;
use itertools::Itertools;

fn main() {
    let lines = read_lines("input.txt").unwrap();
    // in this case, it's a single line
    let s = lines.into_iter().next().unwrap().unwrap();
    //let r = s.chars().next_chunk();
    let r = s.chars().array_windows::<4>();
    let mut counter = 4;
    for chunk in r {
        //println!("{:?}", chunk);
        let res = chunk.to_vec().into_iter().unique().collect::<Vec<_>>();
        if res.len() ==4 {
            break;
        }
        counter += 1;
    }
    println!("{}", counter);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
