use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::result::Result;
use std::vec::Vec;
use regex::Regex;
use std::num::ParseIntError;
use std::collections::HashSet;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn manhattan(c1: &Self, c2: &Self) -> isize {
        (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
    }
}

fn read_sensors(filename: &str) -> MyResult<Vec<(Coord, Coord)>> {
    let mut l: Vec<(Coord, Coord)> = Vec::new();
    let reader = io::BufReader::new(File::open(filename)?);
    let expr = Regex::new(r"-?\d+").unwrap();
    for line in reader.lines() {
        let nums: Vec<isize> = expr.find_iter(line?.as_ref())
                                   .map(|m| isize::from_str_radix(m.as_str(), 10))
                                   .collect::<Result<Vec<isize>, ParseIntError>>()?;
        if nums.len() != 4 {
            return Err("Invalid input line, need 4 integers".into());
        }
        l.push((Coord{x: nums[0], y: nums[1]}, Coord{x: nums[2], y: nums[3]}));
    }
    Ok(l)
}

fn main() {
    let sensors = read_sensors("input.txt").unwrap();
    let mut beacons: HashSet<isize> = HashSet::new();
    let mut no_beacons: HashSet<isize> = HashSet::new();
    const YLINE: isize = 2000000;
    for (sensor, beacon) in sensors.iter() {
        let manhattan = Coord::manhattan(sensor, beacon);
        println!("Manhattan distance of {}", manhattan);
        if beacon.y == YLINE {
            beacons.insert(beacon.x);
        }
        let diff = (sensor.y - YLINE).abs();
        println!("  Sensor is {} away from the y line", diff);
        if diff <= manhattan {
            let rem = manhattan - diff;
            println!("  This means {} points on the y line are guaranteed", 2 * diff + 1);
            for x in sensor.x - rem ..= sensor.x + rem {
                no_beacons.insert(x);
            }
        }
    }
    let really_no_beacons: HashSet<_> = no_beacons.difference(&beacons).collect();
    println!("There are no beacons on {} spaces in line y=2000000", really_no_beacons.len());
}
