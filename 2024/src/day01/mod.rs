use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::util::MyResult;
use crate::util::RunResult;
use crate::util::read_arr;

fn read_lists(fln: &str) -> MyResult<(Vec<usize>, Vec<usize>)> {
    let reader = BufReader::new(File::open(fln)?);
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();

    for line_res in reader.lines() {
        let elems: [usize; 2] = read_arr(&line_res?)?;
        a.push(elems[0]);
        b.push(elems[1]);
    }
    Ok((a, b))
}

fn compute_distance(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut distance = 0;
    let mut a = a.clone();
    a.sort();
    let mut b = b.clone();
    b.sort();
    for (a, b) in a.iter().zip(b.iter()) {
        distance += if a < b { b - a } else { a - b };
    }
    distance
}

pub fn run(fln: &str) -> RunResult {
    let (a, b) = read_lists(fln)?;
    Ok((Some(compute_distance(&a, &b) as isize), None))
}
