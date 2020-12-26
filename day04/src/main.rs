use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

fn count_valid_passports(filename: &str, reqd: &Vec<String>) -> Result<usize, String> {
    let mut valid: usize = 0;
    let mut count: usize = 0;
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut reqd_cp = reqd.to_vec();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;

        if line.len() == 0 {
            count += 1;
            if reqd_cp.len() == 0 {
                valid += 1;
            } else {
                // invalid, I guess do nothing?
            }
            reqd_cp = reqd.to_vec();
        }
        for token in line.split_whitespace() {
            let mut iter = token.splitn(2, ':');
            let key = iter.next().ok_or("some error")?;
            iter.next().ok_or("missing : in token")?;
            reqd_cp.iter().position(|item| item == key).map(|i| reqd_cp.remove(i));
        }
    }
    if reqd_cp.len() == 0 {
        valid += 1;
    }
    count += 1;

    println!("Found {} passports, {} valid", count, valid);
    Ok(valid)
}

fn main() {
    println!("Hello, world!");
    let mv: Vec<String> = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"
    ].iter().map(|s| String::from(*s)).collect();
    count_valid_passports("input.txt", &mv).unwrap();
}
