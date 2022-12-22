use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
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

struct AlgebraicTerm {
    numerator: Vec<isize>,
    denominator: Vec<isize>,
}

fn mul_terms(a: &Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
    let mut out: Vec<isize> = Vec::new();
    for i in 0..a.len() {
        for j in 0..b.len() {
            let ix = i + j;
            if ix == out.len() {
                out.push(a[i] * b[j])
            } else if ix < out.len() {
                out[ix] += a[i] * b[j];
            } else {
                panic!("Impossible!")
            }
        }
    }
    let mut first_nonzero = out.len();
    for i in (0..out.len()).rev() {
        if out[i] != 0 {
            break;
        }
        first_nonzero = i;
    }
    if first_nonzero < out.len() {
        out.drain(first_nonzero..);
    }
    out
}

fn add_sub_terms(a: &Vec<isize>, b: &Vec<isize>, sub: bool) -> Vec<isize> {
    let len = a.len().max(b.len());
    let mut out: Vec<isize> = Vec::with_capacity(len);
    for i in 0..len {
        let mut val = 0;
        if i < a.len() {
            val += a[i];
        }
        if i < b.len() && !sub {
            val += b[i];
        } else if i < b.len() {
            val -= b[i];
        }
        out.push(val);
    }
    out
}

fn constant(v: isize) -> Vec<isize> {
    vec![v]
}
fn var(v: isize) -> Vec<isize> {
    vec![0, v]
}

const PRIMES: &[isize] = &[2, 3, 5, 7, 11, 13, 17, 19];

impl AlgebraicTerm {
    fn constant(v: isize) -> AlgebraicTerm {
        AlgebraicTerm{numerator: constant(v), denominator: constant(1)}
    }
    fn var(v: isize) -> AlgebraicTerm {
        AlgebraicTerm{numerator: var(v), denominator: constant(1)}
    }
    fn simplify(&mut self) {
        loop {
            let mut changed = false;
            for prime in PRIMES.iter() {
                if self.numerator.iter().all(|v| v % prime == 0)
                    && self.denominator.iter().all(|v| v % prime == 0) {
                    for term in self.numerator.iter_mut() {
                        *term = *term / *prime;
                    }
                    for term in self.denominator.iter_mut() {
                        *term = *term / *prime;
                    }
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }
    fn print(&self) {
        print!("(");
        let mut first = true;
        for i in (0..self.numerator.len()).rev() {
            if self.numerator[i] != 0 {
                let mut val = self.numerator[i];
                if !first {
                    if val < 0 {
                        print!(" - ");
                        val = val.abs();
                    } else {
                        print!(" + ");
                    }
                } else {
                    first = false;
                }
                if i != 0 {
                    print!("{} * X^{}", val, i);
                } else {
                    print!("{}", val);
                }
            }
        }
        print!(") / (");
        first = true;
        for i in (0..self.denominator.len()).rev() {
            if self.denominator[i] != 0 {
                let mut val = self.denominator[i];
                if !first {
                    if val < 0 {
                        print!(" - ");
                        val = val.abs();
                    } else {
                        print!(" + ");
                    }
                } else {
                    first = false;
                }
                if i != 0 {
                    print!("{}*X^{}", val, i);
                } else {
                    print!("{}", val);
                }
            }
        }
        print!(")\n");
    }
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
    fn eval_complex(&self, a1: AlgebraicTerm, a2: AlgebraicTerm) -> AlgebraicTerm {
        use Op::*;
        let mut res = match self {
            Add => {
                if a1.denominator == a2.denominator {
                    AlgebraicTerm{
                        numerator: add_sub_terms(&a1.numerator, &a2.numerator, false),
                        denominator: a1.denominator,
                    }
                } else {
                    AlgebraicTerm{
                        numerator: add_sub_terms(
                            &mul_terms(&a1.numerator, &a2.denominator),
                            &mul_terms(&a2.numerator, &a1.denominator),
                            false,
                        ),
                        denominator: mul_terms(&a1.denominator, &a2.denominator),
                    }
                }
            },
            Sub => {
                if a1.denominator == a2.denominator {
                    AlgebraicTerm{
                        numerator: add_sub_terms(&a1.numerator, &a2.numerator, true),
                        denominator: a1.denominator,
                    }
                } else {
                    AlgebraicTerm{
                        numerator: add_sub_terms(
                            &mul_terms(&a1.numerator, &a2.denominator),
                            &mul_terms(&a2.numerator, &a1.denominator),
                            true,
                        ),
                        denominator: mul_terms(&a1.denominator, &a2.denominator),
                    }
                }
            },
            Mul => {
                AlgebraicTerm{
                    numerator: mul_terms(&a1.numerator, &a2.numerator),
                    denominator: mul_terms(&a1.denominator, &a2.denominator),
                }
            },
            Div => {
                AlgebraicTerm{
                    numerator: mul_terms(&a1.numerator, &a2.denominator),
                    denominator: mul_terms(&a1.denominator, &a2.numerator),
                }
            }
        };

        res.simplify();

        res
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

const HUMN: Name = ['h' as u8, 'u' as u8, 'm' as u8, 'n' as u8];
const ROOT: Name = ['r' as u8, 'o' as u8, 'o' as u8, 't' as u8];

fn evalgebra_rec(j: &Jobs, n: Name) -> AlgebraicTerm {
    let job: &Job = j.get(&n).unwrap();
    if n == HUMN {
        return AlgebraicTerm::var(1);
    }
    let res = match job {
        Job::Shout(val) => AlgebraicTerm::constant(*val),
        Job::Oper(op, a1, a2) => {
            op.eval_complex(evalgebra_rec(j, *a1), evalgebra_rec(j, *a2))
        },
    };
    res
}

fn evalgebra(j: &Jobs) {
    // We are root
    let job: &Job = j.get(&ROOT).unwrap();
    if let Job::Oper(_, a1, a2) = job {
        let lhs = evalgebra_rec(j, *a1);
        let rhs = evalgebra_rec(j, *a2);
        let real_op = Op::Sub;
        let val = real_op.eval_complex(lhs, rhs);
        println!("Root returns the following algebra expression, solve for 0:");
        val.print();
        if val.numerator.len() == 2 {
            println!("We got an easy one! Not even quadratic. Solving for zero is just:");
            println!("X = {} / {} = {}", -val.numerator[0], val.numerator[1], -val.numerator[0] / val.numerator[1]);
        }
    } else {
        panic!("Root job should be operator");
    }
}

fn main() {
    let mut filename = "input.txt";
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
    }
    let jobs = read_input(filename).unwrap();
    println!("Root monkey: {}", eval(&jobs, "root").unwrap());

    println!("Doing algebra...");
    evalgebra(&jobs);
}
