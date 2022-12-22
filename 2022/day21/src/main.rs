use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result;
use std::str;
use std::vec::Vec;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn eval(&self, a1: isize, a2: isize) -> isize {
        use Op::*;
        match self {
            Add => a1 + a2,
            Sub => a1 - a2,
            Mul => a1 * a2,
            Div => a1 / a2,
        }
    }
}

type Name = [u8; 4];

#[derive(Clone, PartialEq, Eq, Hash)]
enum Job {
    Shout(isize),
    Oper(Op, [u8; 4], [u8; 4]),
}

type Jobs = HashMap<Name, Job>;

fn get_name(s: &str) -> MyResult<Name> {
    let mut name = [0 as u8; 4];
    let bytes = s.as_bytes();
    if bytes.len() != name.len() {
        return Err("Given name was not the right byte length".into());
    }
    for (i, byte) in s.as_bytes().iter().enumerate() {
        name[i] = *byte;
    }
    Ok(name)
}

fn read_input(filename: &str) -> MyResult<Jobs> {
    let mut jobs = HashMap::new();
    let reader = BufReader::new(File::open(filename)?);
    for line in reader.lines() {
        let line = line?;
        let (name, rest) = line.split_once(":").unwrap();
        let name: [u8; 4] = get_name(name)?;
        let tokens: Vec<_> = rest.trim().split_whitespace().collect();
        if tokens.len() == 3 {
            let op = match tokens[1] {
                "*" => Op::Mul,
                "/" => Op::Div,
                "+" => Op::Add,
                "-" => Op::Sub,
                _ => return Err(format!("Bad operation: {}", tokens[1]).into()),
            };
            jobs.insert(name, Job::Oper(op, get_name(tokens[0])?, get_name(tokens[2])?));
        } else if tokens.len() == 1 {
            let val = isize::from_str_radix(tokens[0], 10)?;
            jobs.insert(name, Job::Shout(val));
        }
    }
    Ok(jobs)
}

fn eval_rec(j: &Jobs, n: Name) -> isize {
    let job: &Job = j.get(&n).unwrap();
    match job {
        Job::Shout(val) => *val,
        Job::Oper(op, a1, a2) => {
            op.eval(eval_rec(j, *a1), eval_rec(j, *a2))
        },
    }
}

fn eval(j: &Jobs, n: &str) -> MyResult<isize> {
    let name = get_name(n)?;
    Ok(eval_rec(j, name))
}

fn main() {
    let mut filename = "input.txt";
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
    }
    let jobs = read_input(filename).unwrap();
    println!("Root monkey: {}", eval(&jobs, "root").unwrap());
}
