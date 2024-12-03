use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::util::read_ints;
use crate::util::RunResult;

fn is_safe(report: &Vec<i32>) -> bool {
    let mut inc = 0;
    let mut dec = 0;
    for i in 0..report.len() - 1 {
        let dif = report[i] - report[i + 1];
        if dif < 0 {
            inc += 1;
        } else if dif > 0 {
            dec += 1;
        } else {
            return false;
        }
        if dif.abs() > 3 {
            return false;
        }
    }
    inc == 0 || dec == 0
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    // I would try to do it a bit more efficiently, but this is so much easier.
    for i in 0..report.len() {
        let mut cp = report.clone();
        cp.remove(i);
        if is_safe(&cp) {
            return true;
        }
    }
    false
}

pub fn run(fln: &str) -> RunResult {
    let reader = BufReader::new(File::open(fln)?);
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        reports.push(read_ints(&line?)?);
    }
    let num_safe = reports.iter().map(|r| if is_safe(r) { 1 } else { 0 }).sum();
    let num_dampener = reports.iter().map(|r| if is_safe_with_dampener(r) { 1 } else { 0 }).sum();
    Ok((Some(num_safe), Some(num_dampener)))
}
