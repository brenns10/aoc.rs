use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::error::Error;
use std::result::Result;
use std::cmp;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy)]
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

fn possible_game(i: usize, total: &Cubes, cubes: Vec<Cubes>) -> MyResult<usize> {
    let mut possible = true;
    for cube in cubes {
        if cube.red > total.red || cube.green > total.green || cube.blue > total.blue {
            possible = false;
            break;
        }
    }
    if possible {
        Ok(i + 1)
    } else {
        Ok(0)
    }
}

fn power_minimum(_: usize, cubes: Vec<Cubes>) -> MyResult<usize> {
    let mut cubes_iter = cubes.iter();
    let mut min_cube = *cubes_iter.next().ok_or("need at least one cube")?;
    for cube in cubes_iter {
        min_cube.red = cmp::max(min_cube.red, cube.red);
        min_cube.green = cmp::max(min_cube.green, cube.green);
        min_cube.blue = cmp::max(min_cube.blue, cube.blue);
    }
    Ok(min_cube.red as usize * min_cube.green as usize * min_cube.blue as usize)
}

fn sum_games(f: &dyn Fn(usize, Vec<Cubes>) -> MyResult<usize>) -> MyResult<usize> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut count: usize = 0;
    for (i, liner) in reader.lines().enumerate() {
        let line = liner?;
        let colon_ix = line.find(':').ok_or("missing colon")?;
        let cubes = line[colon_ix + 2..].split("; ").map(|s| parse_cubes(s)).collect::<MyResult<Vec<Cubes>>>()?;
        count += f(i, cubes)?;
    }
    Ok(count)
}

fn main() {
    let total = Cubes{red: 12, green: 13, blue: 14};
    let possible = sum_games(&|i, s| possible_game(i, &total, s)).unwrap();
    println!("Sum of possible game IDs: {}", possible);
    let powers = sum_games(&power_minimum).unwrap();
    println!("Sum of powers of minimum cube sets: {}", powers);
}
