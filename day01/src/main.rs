use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;
use std::error::Error;

type BoxResult<T> = Result<T,Box<dyn Error>>;

fn read_ints() -> BoxResult<Vec<i32>> {
    let mut ints: Vec<i32> = Vec::new();
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line_str = line?;
        let int = i32::from_str_radix(&line_str, 10)?;
        ints.push(int);
    }
    Ok(ints)
}

fn two_sum_to(v: &Vec<i32>, val: i32, mut start: usize, mut end: usize) -> Option<(usize, usize)> {
    while start < end {
        if v[start] + v[end] > val {
            end -= 1;
        } else if v[start] + v[end] < val {
            start += 1;
        } else {
            return Some((start, end))
        }
    }
    None
}

fn three_sum_to(v: &Vec<i32>, val: i32) -> Option<(usize, usize, usize)> {
    let mut i: usize = 0;
    while i < v.len() - 1 {
        if let Some((y, z)) = two_sum_to(v, val - v[i], i + 1, v.len() - 1) {
            return Some((i, y, z));
        }
        i += 1;
    }
    None
}

fn main() {
    let val = 2020;
    let mut ints = read_ints().unwrap();
    ints.sort();
    println!("Ints: {:?}, len {}", ints, ints.len());
    match two_sum_to(&ints, val, 0, ints.len() - 1) {
        Some((start, end)) => println!("Found 2: {} * {} = {}", ints[start], ints[end], ints[start] * ints[end]),
        None => println!("Two summing to {} not found", val),
    }

    match three_sum_to(&ints, val) {
        Some((x, y, z)) => println!("Found 3: {} * {} * {} = {}", ints[x], ints[y], ints[z], ints[x] * ints[y] * ints[z]),
        None => println!("Three summing to {} not found", val),
    }
}
