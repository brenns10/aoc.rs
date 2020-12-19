use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

#[derive(Debug)]
struct PolicyAndPassword {
    character: char,
    min: u32,
    max: u32,
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
                let pp = PolicyAndPassword{
                    character: m.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                    min: m.get(1).unwrap().as_str().parse().unwrap(),
                    max: m.get(2).unwrap().as_str().parse().unwrap(),
                    password: String::from(m.get(4).unwrap().as_str()),
                };
                println!("line: {} pp: {:?}", line, &pp);
                pps.push(pp);
            }
            None => {
                return Err(format!("Badly formatted string: \"{}\"", line));
            }
        }
    }
    Ok(pps)
}

fn count_valid(pps: &Vec<PolicyAndPassword>) -> i32 {
    let mut valid_count = 0;
    for pp in pps {
        let mut count = 0;
        for c in pp.password.chars() {
            if c == pp.character {
                count += 1;
            }
        }
        if pp.min <= count && count <= pp.max {
            valid_count += 1;
        }
    }
    valid_count
}

fn main() {
    let lines = read_lines("input.txt").unwrap();
    println!("Valid passwords: {}", count_valid(&lines));
}
