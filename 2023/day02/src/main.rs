use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::result::Result;

type MyResult<T> = Result<T, Box<dyn Error>>;

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_cubes(s: &str) -> MyResult<Cubes> {
    let mut cubes = Cubes{red: 0, green: 0, blue: 0};
    for colorstr in s.split(", ") {
        let spcidx = colorstr.find(' ').ok_or("missing space")?;
        let value = u32::from_str_radix(&colorstr[..spcidx], 10)?;
        match &colorstr[spcidx + 1..] {
            "red" => cubes.red = value,
            "green" => cubes.green = value,
            "blue" => cubes.blue = value,
            _ => return Err("invalid color".into()),
        }
    }
    Ok(cubes)
}

fn sum_possible(total: &Cubes) -> MyResult<u32> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut count: u32 = 0;
    for (i, liner) in reader.lines().enumerate() {
        let line = liner?;
        let colon_ix = line.find(':').ok_or("missing colon")?;
        let mut possible = true;
        for scenario in line[colon_ix + 2..].split("; ") {
            let cubes = parse_cubes(scenario)?;
            if cubes.red > total.red || cubes.green > total.green || cubes.blue > total.blue {
                possible = false;
                break;
            }
        }
        if possible {
            count += i as u32 + 1;
        }
    }

    Ok(count)
}

fn main() {
    let total = Cubes{red: 12, green: 13, blue: 14};
    println!("Sum of possible game IDs: {}", sum_possible(&total).unwrap());
}
