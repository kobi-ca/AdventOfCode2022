use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut count_part1 = 0;
    let mut count_part2 = 0;
    for line in lines {
        let l = line.unwrap();
        let splt = l.split(',').collect::<Vec<&str>>();
        assert_eq!(splt.len(), 2);
        // left pair
        let left = splt[0].split('-').collect::<Vec<&str>>();
        let left_start = left[0].parse::<u64>().unwrap();
        let left_end = left[1].parse::<u64>().unwrap();
        // right pair
        let right = splt[1].split('-').collect::<Vec<&str>>();
        let right_start = right[0].parse::<u64>().unwrap();
        let right_end = right[1].parse::<u64>().unwrap();
        let left_rng = left_start..=left_end;
        let right_rng = right_start..=right_end;
        if left_start >= right_start && left_end <= right_end ||
            right_start >= left_start && right_end <= left_end {
                count_part1 += 1;
                //println!("{:?} {:?}", left_rng, right_rng);
        }
        if left_rng.contains(&right_start) || left_rng.contains(&right_start) ||
            right_rng.contains(&left_start) || right_rng.contains(&left_end)
        {
            count_part2 += 1;
            //println!("{:?} {:?}", left_rng, right_rng);
        }
    }
    println!("part1 {}", count_part1);
    println!("part1 {}", count_part2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
