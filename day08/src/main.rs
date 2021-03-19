use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

//# [derive(Debug)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

struct Instr {
    op: Op,
    arg: isize,
}

fn read_instrs(filename: &str) -> Result<Vec<Instr>, String> {
    let f = File::open(filename).map_err(|e| e.to_string())?;
    let br = BufReader::new(f);
    let expr = Regex::new("(acc|jmp|nop) ([+-]\\d+)").unwrap();
    let mut instrs: Vec<Instr> = Vec::new();
    for line in br.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let caps = expr.captures(&line).ok_or(format!("No match in line \"{}\"", line))?;
        let op = match caps.get(1).unwrap().as_str() {
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            "nop" => Op::Nop,
            other => return Err(format!("Bad instruction kind \"{}\"", other)),
        };
        let arg: isize = caps.get(2)
                             .unwrap()
                             .as_str()
                             .parse()
                             .map_err(|_| format!("bad arg in line \"{}\"", line))?;
        instrs.push(Instr{op: op, arg: arg});
    }
    Ok(instrs)
}

fn exec_until_loop(instrs: &Vec<Instr>) -> Result<isize, String> {
    let mut acc: isize = 0;
    let mut idx: usize = 0;
    let mut seen = vec![false; instrs.len()];

    while idx < instrs.len() && !seen[idx] {
        seen[idx] = true;
        match instrs[idx].op {
            Op::Acc => {
                acc += instrs[idx].arg;
                idx += 1;
            }
            Op::Jmp => {
                let new_idx = (idx as isize) + instrs[idx].arg;
                if new_idx < 0 || new_idx >= (instrs.len() as isize) {
                    return Err(format!("Bad jump target {} from instr {}", new_idx, idx))
                }
                idx = new_idx as usize;
            }
            Op::Nop => { idx += 1 },
        }
    }
    if idx >= instrs.len() {
        return Err("Program terminated without loop".to_string());
    }

    Ok(acc)
}

fn main() {
    let prog = read_instrs("input.txt").unwrap();
    match exec_until_loop(&prog) {
        Ok(value) => println!("Accumulator {} before loop", value),
        Err(errstr) => println!("Error! {}", errstr),
    }
}
