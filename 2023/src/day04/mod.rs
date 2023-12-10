use std::fs::File;
use std::io::{BufRead, BufReader};
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

    for line in r.lines() {
        let mut card_score = 0;
        let line = line.unwrap();
        let colon = line.find(":").unwrap();
        let pipe = line.find("|").unwrap();
        let winning = read_ints(&line[colon + 1..pipe]).unwrap();
        let mine = read_ints(&line[pipe + 1..]).unwrap();
        for num in mine.iter() {
            if winning.contains(&num) {
                card_score = if card_score == 0 {1} else {card_score * 2};
            }
        }
        score += card_score;
    }
    println!("Part 1: {}", score);
}
