use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::error::Error;
use std::result::Result;

use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn get_line_number(line: &str) -> Option<u32> {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;
    for c in line.chars() {
        if let Some(val) = c.to_digit(10) {
            if let None = first {
                first = Some(val);
            }
            last = Some(val);
        }
    }
    match (first, last) {
        (Some(f), Some(l)) => Some(10 * f + l),
        _ => None
    }
}

fn digit_val(s: &str) -> Option<u32> {
    if let Some(val) = s.chars().nth(0)?.to_digit(10) {
        return Some(val);
    }
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn get_spelled_line_number(re: &Regex, er: &Regex, line: &str) -> Option<u32> {
    let first = re.find(line)?.as_str();
    let reversed: String = line.chars().rev().collect();
    let last: String = er.find(&reversed)?.as_str().chars().rev().collect();
    Some(digit_val(first)? * 10 + digit_val(&last)?)
}

fn get_line_sum(fln: &str, sum_fn: &dyn Fn (&str) -> Option<u32>) -> MyResult<u32> {
    let reader = BufReader::new(File::open(fln)?);
    let mut sum = 0;
    for line_res in reader.lines() {
        let line = line_res?;
        let num = sum_fn(&line).ok_or("invalid line")?;
        sum += num;
    }
    Ok(sum)
}

pub fn run(fln: &str) {
    let sum = get_line_sum(fln, &get_line_number).unwrap();
    println!("Part 1: {}", sum);

    let re = Regex::new("one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    let er = Regex::new("[0-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno").unwrap();
    let sum = get_line_sum(fln, &|l| get_spelled_line_number(&re, &er, l)).unwrap();
    println!("Part 2: {}", sum);
}
