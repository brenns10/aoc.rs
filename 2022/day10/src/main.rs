use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::result::Result;
use std::env;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn do_cycle(cycle: &mut usize, x: &isize, sigstrength: &mut isize) {
    //println!("During cycle {}, x={}", cycle, x);
    if *cycle % 40 == 20 {
        println!("Strength component, cycle {}: {}", cycle, (*cycle as isize) * *x);
        *sigstrength += (*cycle as isize) * *x;
    }
    *cycle += 1;
}

fn do_cycle_crt(cycle: &mut usize, x: &isize, _: &mut isize) {
    let cur_pixel = ((*cycle - 1) as isize) % 40;
    if cur_pixel == 0 && *cycle != 1 {
        print!("\n");
    }
    if *x - 1 <= cur_pixel && cur_pixel <= *x + 1 {
        print!("#");
    } else {
        print!(".");
    }
    *cycle += 1;
}

fn read_instructions(filename: &str, func: fn(&mut usize, &isize, &mut isize)) -> MyResult<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut cycle: usize = 1;
    let mut x: isize = 1;
    let mut sigstrength: isize = 0;

    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 1 && tokens[0] == "noop" {
            func(&mut cycle, &x, &mut sigstrength);
        } else if tokens.len() == 2 && tokens[0] == "addx" {
            let increment = isize::from_str_radix(tokens[1], 10)?;
            func(&mut cycle, &x, &mut sigstrength);
            func(&mut cycle, &x, &mut sigstrength);
            x += increment;
        } else {
            return Err(format!("Bad input line: {}", line).into());
        }
    }
    println!("\nSignal strength: {}", sigstrength);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() >= 2 {
        filename = &args[1];
    }
    println!("First, computing the signal strength:");
    read_instructions(filename, do_cycle).unwrap();
    println!("Second, printing the CRT:");
    read_instructions(filename, do_cycle_crt).unwrap();
}
