use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate regex;

use regex::Regex;

// all kinds of attempt to read input
// but my input is constant:
// So we will hardcode the stacks
// and skip 9 first lines
// [Q] [J]                         [H]
// [G] [S] [Q]     [Z]             [P]
// [P] [F] [M]     [F]     [F]     [S]
// [R] [R] [P] [F] [V]     [D]     [L]
// [L] [W] [W] [D] [W] [S] [V]     [G]
// [C] [H] [H] [T] [D] [L] [M] [B] [B]
// [T] [Q] [B] [S] [L] [C] [B] [J] [N]
// [F] [N] [F] [V] [Q] [Z] [Z] [T] [Q]
//  1   2   3   4   5   6   7   8   9

// let l = line.unwrap();
// let patter = l.chars().collect::<Vec<char>>().chunks(3)
//     .map(|c|);
// let r = line
//     .and_then(|l| { let f = l.chars().collect::<Vec<char>>()
//                                 .chunks(3);

//                               }
//                             );
// let r = line.and_then(|l| l.chars().chunks(3));
// let num_stacks = 9; // hardcoded by input.txt file
// for i in 0..=num_stacks {
//     let stack_id: char = line.iter().skip(1).

fn part1() {
    let mut v: Vec<Vec<char>> = vec![
        vec!['F', 'T', 'C', 'L', 'R', 'P', 'G', 'Q'],
        vec!['N', 'Q', 'H', 'W', 'R', 'F', 'S', 'J'],
        vec!['F', 'B', 'H', 'W', 'P', 'M', 'Q'],
        vec!['V', 'S', 'T', 'D', 'F'],
        vec!['Q', 'L', 'D', 'W', 'V', 'F', 'Z'],
        vec!['Z', 'C', 'L', 'S'],
        vec!['Z', 'B', 'M', 'V', 'D', 'F'],
        vec!['T', 'J', 'B'],
        vec!['Q', 'N', 'B', 'G', 'L', 'S', 'P', 'H'],
    ];

    let lines = read_lines("input.txt").unwrap();
    let mut skip = 0;
    let input_re = Regex::new(
        r#"move (\d*) from (\d*) to (\d*)"#
    ).unwrap();
    for line in lines {
        if skip < 10 {
            skip +=1;
            continue;
        }
        let l = line.unwrap();
        let cap = input_re.captures(l.as_str()).unwrap();
        // println!("{:?}", cap.get(1).map_or("???", |m| m.as_str()));
        let mut num_elems_to_move = cap.get(1)
            .map(|m| m.as_str().parse::<usize>().unwrap() ).unwrap();
        let from = cap.get(2)
            .map(|m| m.as_str().parse::<usize>().unwrap() ).unwrap();
        let to = cap.get(3)
            .map(|m| m.as_str().parse::<usize>().unwrap() ).unwrap();
        // println!("{} {} {}", what_to_move, from, to);
        while num_elems_to_move > 0 {
            let elem = v[from - 1].pop().unwrap();
            v[to - 1].push(elem);
            num_elems_to_move -= 1;
        }
    }
    println!("{}{}{}{}{}{}{}{}{}",
                v[0].last().unwrap(),
                v[1].last().unwrap(),
                v[2].last().unwrap(),
                v[3].last().unwrap(),
                v[4].last().unwrap(),
                v[5].last().unwrap(),
                v[6].last().unwrap(),
                v[7].last().unwrap(),
                v[8].last().unwrap()
            );
}

fn part2() {
    let mut v: Vec<Vec<char>> = vec![
        vec!['F', 'T', 'C', 'L', 'R', 'P', 'G', 'Q'],
        vec!['N', 'Q', 'H', 'W', 'R', 'F', 'S', 'J'],
        vec!['F', 'B', 'H', 'W', 'P', 'M', 'Q'],
        vec!['V', 'S', 'T', 'D', 'F'],
        vec!['Q', 'L', 'D', 'W', 'V', 'F', 'Z'],
        vec!['Z', 'C', 'L', 'S'],
        vec!['Z', 'B', 'M', 'V', 'D', 'F'],
        vec!['T', 'J', 'B'],
        vec!['Q', 'N', 'B', 'G', 'L', 'S', 'P', 'H'],
    ];

    let lines = read_lines("input.txt").unwrap();
    let mut skip = 0;
    let input_re = Regex::new(
        r#"move (\d*) from (\d*) to (\d*)"#
    ).unwrap();
    for line in lines {
        if skip < 10 {
            skip +=1;
            continue;
        }
        let l = line.unwrap();
        let cap = input_re.captures(l.as_str()).unwrap();
        // println!("{:?}", cap.get(1).map_or("???", |m| m.as_str()));
        let mut num_elems_to_move = cap.get(1)
            .map(|m| m.as_str().parse::<usize>().unwrap() ).unwrap();
        let from = cap.get(2)
            .map(|m| m.as_str().parse::<usize>().unwrap() ).unwrap();
        let to = cap.get(3)
            .map(|m| m.as_str().parse::<usize>().unwrap() ).unwrap();

        let mut tmp_vec: Vec<char> = vec![];
        while num_elems_to_move > 0 {
            let elem = v[from - 1].pop().unwrap();
            tmp_vec.push(elem);
            num_elems_to_move -= 1;
        }
        v[to - 1].extend(tmp_vec.iter().rev());
    }
    println!("{}{}{}{}{}{}{}{}{}",
                v[0].last().unwrap(),
                v[1].last().unwrap(),
                v[2].last().unwrap(),
                v[3].last().unwrap(),
                v[4].last().unwrap(),
                v[5].last().unwrap(),
                v[6].last().unwrap(),
                v[7].last().unwrap(),
                v[8].last().unwrap()
            );
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
