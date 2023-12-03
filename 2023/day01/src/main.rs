use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::result::Result;

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

fn get_line_sum() -> MyResult<u32> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut sum = 0;
    for line_res in reader.lines() {
        let line = line_res?;
        let num = get_line_number(&line).ok_or("invalid line")?;
        sum += num;
    }
    Ok(sum)
}

fn main() {
    let sum = get_line_sum().unwrap();
    println!("Sum: {}", sum);
}
