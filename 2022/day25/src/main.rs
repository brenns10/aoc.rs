use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::result::Result;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn from_snafu(val: &str) -> isize {
    let mut ival = 0;

    for c in val.chars() {
        ival *= 5;
        ival += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Bad digit in SNAFU literal"),
        };
    }

    ival
}

fn to_snafu(val: isize) -> String {
    let mut lit = String::new();
    let mut val = val;

    while val != 0 {
        let mut rem = val % 5;
        val = val / 5;
        if rem >= 3 {
            rem -= 5;
            val += 1;
        }
        lit.insert(0, match rem {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!("Bad rem={} in to_snafu()", rem),
        })
    }

    lit
}

fn read_snafus(filename: &str) -> MyResult<Vec<String>> {
    let reader = BufReader::new(File::open(filename)?);
    let mut res = Vec::new();

    for line in reader.lines() {
        res.push(line?);
    }

    Ok(res)
}

fn main() {
    let mut filename = "input.txt";
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
    }
    let snafus = read_snafus(filename).unwrap();
    let mut sum = 0;
    for val in snafus.iter() {
        sum += from_snafu(val);
    }
    print!("{}", to_snafu(sum));
}
