use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut count = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let (half1, half2) = ip.split_at(ip.len()/2);
                let set1: HashSet<char> = HashSet::from_iter(half1.chars());
                let set2: HashSet<char> = HashSet::from_iter(half2.chars());
                let mut intersection = set1.intersection(&set2);
                let c = intersection.nth(0).unwrap();
                match c {
                    'a'..='z' => count += *c as u32 - 'a' as u32 + 1,
                    'A'..='Z' => count += *c as u32 - 'A' as u32 + 27,
                    _ => panic!("..."),
                }
            } else {
                panic!("...");
            }
        }
    }
    println!("{}", count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
