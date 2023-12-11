use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

use crate::util::return_part1;
use crate::util::RunResult;

enum Dir {
    Left,
    Right,
}

impl Dir {
    fn new(s: char) -> Option<Self> {
        use Dir::*;
        match s {
            'L' => Some(Left),
            'R' => Some(Right),
            _ => None,
        }
    }
}

pub fn run(fln: &str) -> RunResult {
    let mut l = BufReader::new(File::open(fln)?).lines();

    let instrs = l.next().ok_or("not enough input")??
        .chars().map(Dir::new).collect::<Option<Vec<Dir>>>()
        .ok_or("Invalid directions")?;
    l.next().ok_or("not enough input")??;

    let expr = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)")?;
    let mut net: HashMap<String, (String, String)> = HashMap::new();

    for line in l {
        let line = line?;
        let m = expr.captures(&line).ok_or("invalid network node")?;
        let src = String::from(m.get(1).unwrap().as_str());
        let left = String::from(m.get(2).unwrap().as_str());
        let right = String::from(m.get(3).unwrap().as_str());
        net.insert(src, (left, right));
    }

    let mut curr = "AAA";
    let mut steps = 0;
    while curr != "ZZZ" {
        let dir = &instrs[steps % instrs.len()];
        let next = net.get(curr).ok_or("missing node")?;
        steps += 1;
        match dir {
            Dir::Left => {curr = &next.0},
            Dir::Right => {curr = &next.1},
        }
    }

    println!("Part 1: {}", steps);
    return_part1(steps as isize)
}
