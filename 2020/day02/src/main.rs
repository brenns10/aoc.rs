use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

#[derive(Debug)]
struct PolicyAndPassword {
    character: char,
    min: usize,
    max: usize,
    password: String,
}

fn read_lines(filename: &str) -> Result<Vec<PolicyAndPassword>, String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut pps: Vec<PolicyAndPassword> = Vec::new();
    let expr = Regex::new("\\A(\\d+)-(\\d+) ([a-z]): ([a-z]+)").unwrap();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        match expr.captures(&line) {
            Some(m) => {
                pps.push(PolicyAndPassword{
                    character: m.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                    min: m.get(1).unwrap().as_str().parse().unwrap(),
                    max: m.get(2).unwrap().as_str().parse().unwrap(),
                    password: String::from(m.get(4).unwrap().as_str()),
                });
            }
            None => {
                return Err(format!("Badly formatted string: \"{}\"", line));
            }
        }
    }
    Ok(pps)
}

fn valid_part1(pp: &PolicyAndPassword) -> bool {
    let mut count = 0;
    for c in pp.password.chars() {
        if c == pp.character {
            count += 1;
        }
    }
    return pp.min <= count && count <= pp.max
}

fn valid_part2(pp: &PolicyAndPassword) -> bool {
    let first = pp.password.chars()
                           .nth(pp.min - 1)
                           .map_or(false, |c| c == pp.character);
    let second = pp.password.chars()
                            .nth(pp.max - 1)
                            .map_or(false, |c| c == pp.character);
    (first || second) && !(first && second)
}

fn count_valid(pps: &Vec<PolicyAndPassword>, validator: &dyn Fn(&PolicyAndPassword) -> bool) -> i32 {
    let mut valid_count = 0;
    for pp in pps {
        if validator(pp) {
            valid_count += 1;
        }
    }
    valid_count
}

fn main() {
    let lines = read_lines("input.txt").unwrap();
    println!("Valid passwords for part 1: {}", count_valid(&lines, &valid_part1));
    println!("Valid passwords for part 2: {}", count_valid(&lines, &valid_part2));
}
