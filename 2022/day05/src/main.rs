use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

type Stacks = Vec<Vec<u8>>;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn read_input() -> (Stacks, Vec<Move>) {
    let mut stacks: Stacks = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    let file = File::open("input.txt").unwrap();
    let mut iter = io::BufReader::new(file).lines();

    loop {
        let line: String = iter.next().unwrap().unwrap();
        let line_bytes: Vec<u8> = line.into();
        if line_bytes.len() >= 2 && (line_bytes[1] as char).is_ascii_digit() {
            break;
        }
        for ix in (0..line_bytes.len()).step_by(4) {
            let stack_ix = ix / 4;
            if stacks.len() <= stack_ix {
                stacks.push(Vec::new());
            }
            let c = line_bytes[ix + 1];
            if c == b' ' {
                continue;
            }
            stacks[stack_ix].push(c);
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    // Next line is empty, assert so
    assert_eq!(iter.next().unwrap().unwrap(), "");

    loop {
        match iter.next() {
            None => break,
            Some(line) => {
                let line: String = line.unwrap();
                let mut matches: Vec<u32> = Vec::new();
                for s in line.split(|c: char| !c.is_numeric()) {
                    if s == "" {continue;}
                    matches.push(u32::from_str_radix(s, 10).unwrap());
                }
                moves.push(Move{
                    count: matches[0] as usize,
                    from: matches[1] as usize,
                    to: matches[2] as usize,
                });
            }
        }
    }
    (stacks, moves)
}

fn read_boxes(stacks: &Stacks) -> String {
    let mut s = String::new();
    for stack in stacks {
        if !stack.is_empty() {
            s.push(stack[stack.len() - 1] as char)
        }
    }
    s
}

fn exec_moves(stacks: &mut Stacks, moves: &Vec<Move>) {
    for mov in moves {
        for _ in 0 .. mov.count {
            let val = stacks[mov.from - 1].pop().unwrap();
            stacks[mov.to - 1].push(val);
        }
    }
}

fn main() {
    let (mut stacks, moves): (Stacks, Vec<Move>) = read_input();
    println!("Stacks start with top reading: {}", read_boxes(&stacks));
    exec_moves(&mut stacks, &moves);
    println!("Current top reading: {}", read_boxes(&stacks));
}
