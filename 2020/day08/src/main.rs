use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

# [derive(Debug, Copy, Clone)]
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

fn execute(instrs: &Vec<Instr>) -> Result<(isize, bool), String> {
    let mut acc: isize = 0;
    let mut idx: usize = 0;
    let mut cnt: isize = 0;
    let mut seen = vec![-1; instrs.len()];

    while idx < instrs.len() && seen[idx] < 0 {
        seen[idx] = cnt;
        cnt += 1;
        match instrs[idx].op {
            Op::Acc => {
                acc += instrs[idx].arg;
                idx += 1;
            }
            Op::Jmp => {
                let new_idx = (idx as isize) + instrs[idx].arg;
                if new_idx < 0 || new_idx >= (instrs.len() as isize) + 1 {
                    return Err(format!("Bad jump target {} from instr {}", new_idx, idx))
                }
                idx = new_idx as usize;
            }
            Op::Nop => { idx += 1 },
        }
    }
    let infinite_loop = idx < instrs.len();
    Ok((acc, infinite_loop))
}

fn exec_until_loop(instrs: &Vec<Instr>) -> Result<isize, String> {
    execute(instrs).and_then(
        |r| {
            if r.1 { Ok(r.0) }
            else { Err("Did not infinite loop".to_string()) }
        }
    )
}

fn find_swapped_instr(instrs: &mut Vec<Instr>) -> Result<(isize, usize), String> {
    for i in 0..instrs.len() {
        let orig_op = instrs[i].op;
        instrs[i].op = match instrs[i].op {
            Op::Nop => Op::Jmp,
            Op::Jmp => Op::Nop,
            _ => continue
        };
        if let Ok((acc, false)) = execute(instrs) {
            return Ok((acc, i));
        }
        instrs[i].op = orig_op;
    }
    Err("None worked".to_string())
}

fn main() {
    let mut prog = read_instrs("input.txt").unwrap();
    let acc1 = exec_until_loop(&prog).unwrap();
    println!("Accumulator {} before loop", acc1);
    let (acc2, instr) = find_swapped_instr(&mut prog).unwrap();
    println!("Accumulator {} after swapping instruction {}", acc2, instr);
}
