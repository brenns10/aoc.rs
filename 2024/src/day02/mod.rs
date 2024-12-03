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

pub fn run(fln: &str) -> RunResult {
    let reader = BufReader::new(File::open(fln)?);
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        reports.push(read_ints(&line?)?);
    }
    let num_safe = reports.iter().map(|r| if is_safe(r) { 1 } else { 0 }).sum();
    Ok((Some(num_safe), None))
}
