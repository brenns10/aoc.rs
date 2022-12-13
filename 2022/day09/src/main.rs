use std::fs::File;
use std::io::{self, BufRead};
use std::result::Result;
use std::error::Error;
use std::collections::HashSet;

type MyResult<T> = Result<T, Box<dyn Error>>;
type Coord = (i32, i32);

fn touching(head: Coord, tail: Coord) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn direction(name: &str) -> MyResult<Coord> {
    match name {
        "R" => Ok((0, 1)),
        "L" => Ok((0, -1)),
        "U" => Ok((1, 0)),
        "D" => Ok((-1, 0)),
        _ => Err("Unrecognized direction".into()),
    }
}

fn update_tail(head: Coord, tail: Coord) -> Coord {
    let mut tail = tail;
    if touching(head, tail) {
        return tail;
    }
    if head.1 != tail.1 {
        tail.1 += if head.1 > tail.1 { 1 } else { -1 }
    }
    if head.0 != tail.0 {
        tail.0 += if head.0 > tail.0 { 1 } else { -1 }
    }
    tail
}

fn do_sim() -> MyResult<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut head: Coord = (0, 0);
    let mut tail: Coord = (0, 0);
    let mut positions: HashSet<Coord> = HashSet::new();
    positions.insert(tail);
    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != 2 {
            return Err("Bad input line".into());
        }
        let dir = direction(tokens[0])?;
        let count = u32::from_str_radix(tokens[1], 10)?;
        for _ in 0..count {
            head = (head.0 + dir.0, head.1 + dir.1);
            tail = update_tail(head, tail);
            positions.insert(tail);
        }
    }
    println!("Tail unique positions: {}", positions.len());
    Ok(())
}

fn main() {
    do_sim().unwrap();
}
