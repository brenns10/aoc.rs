use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::result::Result;
use std::env;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn do_cycle(cycle: &mut usize, x: &isize, sigstrength: &mut isize) {
    println!("During cycle {}, x={}", cycle, x);
    if *cycle % 40 == 20 {
        println!("Strength component, cycle {}: {}", cycle, (*cycle as isize) * *x);
        *sigstrength += (*cycle as isize) * *x;
    }
    *cycle += 1;
}

fn read_instructions(filename: &str) -> MyResult<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut cycle: usize = 1;
    let mut x: isize = 1;
    let mut sigstrength: isize = 0;

    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 1 && tokens[0] == "noop" {
            do_cycle(&mut cycle, &x, &mut sigstrength);
        } else if tokens.len() == 2 && tokens[0] == "addx" {
            let increment = isize::from_str_radix(tokens[1], 10)?;
            do_cycle(&mut cycle, &x, &mut sigstrength);
            do_cycle(&mut cycle, &x, &mut sigstrength);
            x += increment;
        } else {
            return Err(format!("Bad input line: {}", line).into());
        }
    }
    println!("Signal strength: {}", sigstrength);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() >= 2 {
        filename = &args[1];
    }
    read_instructions(filename).unwrap();
}
