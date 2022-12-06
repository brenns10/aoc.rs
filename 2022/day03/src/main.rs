use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::vec::Vec;
//use std::str;

type MyResult<T> = Result<T, Box<dyn Error>>;
type Rucksack = [u8; 52];

fn priority(item: u8) -> MyResult<u32> {
    if b'a' <= item && item <= b'z' {
        Ok((item - b'a' + 1) as u32)
    } else if b'A' <= item && item <= b'Z' {
        Ok((item - b'A' + 27) as u32)
    } else {
        Err("Invalid priority".into())
    }
}

fn read_rucksacks() -> MyResult<Vec<(Rucksack, Rucksack)>> {
    let mut rucksacks: Vec<(Rucksack, Rucksack)> = Vec::new();
    for line in io::BufReader::new(File::open("input.txt")?).split(b'\n') {
        let line = line?;
        assert_eq!(line.len() % 2, 0);
        let halfway = line.len() / 2;
        let mut lhs: Rucksack = [0; 52];
        let mut rhs: Rucksack = [0; 52];
        for (i, code) in line.into_iter().enumerate() {
            let idx = (priority(code)? - 1) as usize;
            if i < halfway {
                lhs[idx] += 1;
            } else {
                rhs[idx] += 1;
            }
        }
        rucksacks.push((lhs, rhs))
    }
    Ok(rucksacks)
}

/* Given two compartments, find the item present in both */
fn common_piece(args: &[&Rucksack]) -> Option<u32> {
    for idx in 0..52 {
        if args.iter().all(|v| v[idx] > 0) {
            return Some((idx + 1) as u32);
        }
    }
    None
}

fn combine(rhs: &Rucksack, lhs: &Rucksack) -> Rucksack {
    let mut new: Rucksack = [0; 52];
    for idx in 0..52 {
        new[idx] += rhs[idx] + lhs[idx];
    }
    new
}

fn main() {
    let rucksacks = read_rucksacks().unwrap();
    let total: u32 = rucksacks.iter().map(|r| common_piece(&[&r.0, &r.1]).unwrap()).sum();
    println!("Sum of common item priorities: {}", total);

    assert_eq!(0, rucksacks.len() % 3);
    let mut total_prio = 0;
    for i in 0..rucksacks.len() / 3 {
        total_prio += common_piece(&[
            &combine(&rucksacks[i * 3].0, &rucksacks[i * 3].1),
            &combine(&rucksacks[i * 3 + 1].0, &rucksacks[i * 3 + 1].1),
            &combine(&rucksacks[i * 3 + 2].0, &rucksacks[i * 3 + 2].1),
        ]).unwrap();
    }
    println!("Sum of group priorities: {}", total_prio);
}
