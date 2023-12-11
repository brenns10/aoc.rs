use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{Iterator, self};

use crate::util::read_ints;
use crate::util::return_part1and2;
use crate::util::RunResult;

pub fn run(fln: &str) -> RunResult {
    let r = BufReader::new(File::open(fln)?);
    let mut score = 0;

    let mut counts: Vec<u32> = Vec::new();

    for line in r.lines() {
        let mut card_score = 0;
        let mut count = 0;
        let line = line?;
        let colon = line.find(":").ok_or("missing colon")?;
        let pipe = line.find("|").ok_or("missing pipe")?;
        let winning: Vec<u32> = read_ints(&line[colon + 1..pipe])?;
        let mine: Vec<u32> = read_ints(&line[pipe + 1..])?;
        for num in mine.iter() {
            if winning.contains(&num) {
                count += 1;
                card_score = if card_score == 0 {1} else {card_score * 2};
            }
        }
        score += card_score;
        counts.push(count);
    }
    println!("Part 1: {}", score);

    let mut copies: Vec<u32> = iter::repeat(1).take(counts.len()).collect();
    let mut total = 0;
    for i in 0..copies.len() {
        total += copies[i];
        for j in i + 1 .. std::cmp::min(i + 1 + counts[i] as usize, copies.len()) {
            copies[j] += copies[i];
        }
    }
    println!("Part 2: {}", total);

    return_part1and2(score as isize, total as isize)
}
