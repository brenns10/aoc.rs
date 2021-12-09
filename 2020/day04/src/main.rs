use regex::Captures;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

struct Policy {
    expr: Regex,
    valid: Option<Box<dyn Fn(&Captures) -> Result<bool, String>>>,
}

fn policy_match(value: &str, policy: &Policy) -> Result<bool, String> {
    match policy.expr.captures(value) {
        None => Ok(false),
        Some(c) => match &policy.valid {
            None => Ok(true),
            Some(f) => f.as_ref()(&c),
        }
    }
}

fn minmax(min: isize, max: isize) -> Box<dyn Fn(&Captures) -> Result<bool, String>> {
    Box::new(move |c: &Captures| {
        let m = c.get(1).ok_or("missing match!")?;
        let res: isize = m.as_str().parse().map_err(|_| String::from("couldnt parse"))?;
        Ok(res >= min && res <= max)
    })
}

fn minmax_hgt(c: &Captures) -> Result<bool, String> {
    let m = c.get(1).ok_or("missing match!")?;
    let u = c.get(2).ok_or("missing unit!")?;
    let res: isize = m.as_str().parse().map_err(|_| String::from("couldnt parse"))?;
    if u.as_str() == "cm" {
        Ok(res >= 150 && res <= 193)
    } else {
        Ok(res >= 59 && res <= 76)
    }
}

fn count_valid_passports(filename: &str, reqd: &Vec<Policy>) -> Result<usize, String> {
    let mut valid: usize = 0;
    let mut count: usize = 0;
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut seen: Vec<bool> = vec![false; reqd.len()];
    let mut counts = vec![0; reqd.len()];
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;

        if line.len() == 0 {
            count += 1;
            if seen.iter().all(|x| *x) {
                valid += 1;
            } else {
                // invalid, I guess do nothing?
            }
            seen = vec![false; reqd.len()];
        }
        for token in line.split_whitespace() {
            for (i, candidate) in reqd.iter().enumerate() {
                if policy_match(token, candidate)? {
                    seen[i] = true;
                    counts[i] += 1;
                    break;
                }
            }
        }
    }
    if seen.iter().all(|x| *x) {
        valid += 1;
    }
    count += 1;

    println!("Found {} passports, {} valid", count, valid);
    println!("Rule matches: {:?}", &counts);
    Ok(valid)
}

fn main() {
    println!("Hello, world!");
    let mv: Vec<Policy> = vec![
        Policy{expr: Regex::new("^byr:\\S+$").unwrap(), valid: None},
        Policy{expr: Regex::new("^iyr:\\S+$").unwrap(), valid: None},
        Policy{expr: Regex::new("^eyr:\\S+$").unwrap(), valid: None},
        Policy{expr: Regex::new("^hgt:\\S+$").unwrap(), valid: None},
        Policy{expr: Regex::new("^hcl:\\S+$").unwrap(), valid: None},
        Policy{expr: Regex::new("^ecl:\\S+$").unwrap(), valid: None},
        Policy{expr: Regex::new("^pid:\\S+$").unwrap(), valid: None}
    ];
    count_valid_passports("input.txt", &mv).unwrap();
    let part2policies: Vec<Policy> = vec![
        Policy{expr: Regex::new("^byr:(\\d{4})$").unwrap(), valid: Some(minmax(1920, 2002))},
        Policy{expr: Regex::new("^iyr:(\\d{4})$").unwrap(), valid: Some(minmax(2010, 2020))},
        Policy{expr: Regex::new("^eyr:(\\d{4})$").unwrap(), valid: Some(minmax(2020, 2030))},
        Policy{expr: Regex::new("^hgt:(\\d+)(cm|in)$").unwrap(), valid: Some(Box::new(minmax_hgt))},
        Policy{expr: Regex::new("^hcl:#[0-9a-f]{6}$").unwrap(), valid: None},
        Policy{expr: Regex::new("^ecl:(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(), valid: None},
        Policy{expr: Regex::new("^pid:\\d{9}$").unwrap(), valid: None},
    ];
    count_valid_passports("input.txt", &part2policies).unwrap();
}
