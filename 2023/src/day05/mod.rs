use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

struct Map {
    dst: usize,
    src: usize,
    len: usize,
}

pub fn read_ints(s: &str) -> MyResult<Vec<usize>> {
    let mut vec = Vec::new();

    for num in s.split_ascii_whitespace() {
        vec.push(usize::from_str_radix(num, 10)?);
    }

    Ok(vec)
}

fn read_maps(fln: &str) -> MyResult<(Vec<usize>, Vec<Vec<Map>>)> {
    let mut maps = Vec::new();

    let r = BufReader::new(File::open(fln).unwrap());
    let mut lines = r.lines();
    let seed_line = lines.next().ok_or("Missing first line")??;
    let index = seed_line.find(":").ok_or("Seed line missing colon")?;
    let seeds = read_ints(&seed_line[index + 1..])?;

    lines.next().ok_or("Missing delimiter")??;

    let mut current_map = Vec::new();
    for line in lines {
        let line = line?;

        if line == "" {
            maps.push(current_map);
            current_map = Vec::new();
        } else if line.contains(":") {
            // skip me
        } else {
            let ints = read_ints(&line)?;
            if ints.len() != 3 {
                return Err("Exactly three numbers required for maps".into());
            }
            current_map.push(Map{
                dst: ints[0],
                src: ints[1],
                len: ints[2],
            })
        }
    }
    if current_map.len() != 0 {
        maps.push(current_map);
    }
    Ok((seeds, maps))
}

pub fn run(fln: &str) {
    let (seeds, maps) = read_maps(fln).unwrap();
    let mut current = seeds.clone();
    for maplist in maps.iter() {
        for i in 0..current.len() {
            for map in maplist.iter() {
                if current[i] >= map.src && current[i] < map.src + map.len {
                    current[i] = current[i] - map.src + map.dst;
                    break;
                }
            }
        }
    }
    println!("Part 1: {}", current.iter().min().unwrap());
}
