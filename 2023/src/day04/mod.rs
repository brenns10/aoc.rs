use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{Iterator, self};
use std::num::ParseIntError;

pub fn read_ints(s: &str) -> Result<Vec<u32>, ParseIntError> {
    let mut vec = Vec::new();

    for num in s.split_ascii_whitespace() {
        vec.push(u32::from_str_radix(num, 10)?);
    }

    Ok(vec)
}

pub fn run(fln: &str) {
    let r = BufReader::new(File::open(fln).unwrap());
    let mut score = 0;

    let mut counts: Vec<u32> = Vec::new();

    for line in r.lines() {
        let mut card_score = 0;
        let mut count = 0;
        let line = line.unwrap();
        let colon = line.find(":").unwrap();
        let pipe = line.find("|").unwrap();
        let winning = read_ints(&line[colon + 1..pipe]).unwrap();
        let mine = read_ints(&line[pipe + 1..]).unwrap();
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
}
