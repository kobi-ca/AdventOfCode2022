use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum State {
    ListingDirectory,
    CMD,
}

fn parse_cmd(l: &str, state: &mut State) {
    *state = State::CMD;
    if l.starts_with("$ cd /") {
        println!("moving to the top");
    } else if l.starts_with("$ cd ..") {
        println!("moving one up");
    } else if l.starts_with("cd ") {
        println!("cd into dir");
    } else if l.contains("$ ls") {
        println!("listing dir");
        *state = State::ListingDirectory;
    }
}

fn listing_directory(l: &str, state: &mut State) {
    if l.starts_with('$') {
        println!("done listing dir");
        parse_cmd(l, state);
    } else if l.starts_with("dir") {
            let dir = &l[4..];
            println!("it contains dir {}", dir);
        } else {
            let mut res = l.split_whitespace();
            let sz = res.next().unwrap().parse::<usize>().unwrap();
            let fname = res.next().unwrap();
            println!("it contains file size {} name {}", sz, fname);
        }
}

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut state = State::CMD;
    for line in lines {
        let l = line.unwrap();
        match state {
            State::CMD => {
                parse_cmd(&l, &mut state);
            },
            State::ListingDirectory => {
                listing_directory(&l, &mut state);
            },
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
