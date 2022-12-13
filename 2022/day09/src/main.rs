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

fn do_sim(rope_count: usize) -> MyResult<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut rope: Vec<Coord> = (0..rope_count).map(|_| (0, 0)).collect();
    let mut positions: HashSet<Coord> = HashSet::new();
    positions.insert(*rope.last().unwrap());
    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != 2 {
            return Err("Bad input line".into());
        }
        let dir = direction(tokens[0])?;
        let move_count = u32::from_str_radix(tokens[1], 10)?;
        for _ in 0..move_count {
            // Move the head of the rope
            rope[0] = (rope[0].0 + dir.0, rope[0].1 + dir.1);
            // Move the rest of the rope
            for i in 1..rope_count {
                rope[i] = update_tail(rope[i-1], rope[i]);
            }
            // Track the rope
            positions.insert(*rope.last().unwrap());
        }
    }
    println!("Tail unique positions: {}", positions.len());
    Ok(())
}

fn main() {
    println!("Doing simulation of 2 knots:");
    do_sim(2).unwrap();
    println!("Doing simulation of 10 knots:");
    do_sim(10).unwrap();
}
