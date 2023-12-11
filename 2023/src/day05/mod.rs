use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::MyResult;
use crate::util::read_ints;
use crate::util::return_part1and2;
use crate::util::RunResult;

struct Map {
    dst: usize,
    src: usize,
    len: usize,
}

fn read_maps(fln: &str) -> MyResult<(Vec<usize>, Vec<Vec<Map>>)> {
    let mut maps = Vec::new();

    let r = BufReader::new(File::open(fln)?);
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

pub fn run(fln: &str) -> RunResult {
    let (seeds, maps) = read_maps(fln)?;
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
    let part1 = current.iter().min().ok_or("impossible condition")?;
    println!("Part 1: {}", part1);

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        ranges.push((seeds[i], seeds[i] + seeds[i + 1]))
    }

    for maplist in maps.iter() {
        let mut next_ranges: Vec<(usize, usize)> = Vec::new();
        //println!("ranges len: {}", ranges.len());
        while !ranges.is_empty() {
            let (start, end) = ranges.pop().ok_or("impossible condition")?;
            let mut mapped = false;
            for map in maplist.iter() {
                let map_end = map.src + map.len;
                if start >= map_end || end <= map.src {
                    continue;
                }
                mapped = true;

                //println!("Current range: {}, {}", start, end);
                //println!("    map range: {}, {}", map.src, map_end);

                // Overlap: break the range into as many as 3 sub-ranges:
                // [start .. map.src) -> still unmapped
                // [map.src .. map_end) -> mapped to next ranges
                // [map_end .. end) -> still unmapped
                if start < map.src {
                    // still unmapped
                    //println!("unmapped range at start: {}, {}", start, map.src);
                    ranges.push((start, map.src));
                }
                // The mapped portion: always exists
                next_ranges.push(
                    (std::cmp::max(start, map.src) - map.src + map.dst,
                     std::cmp::min(end, map_end) - map.src + map.dst));
                //println!("mapping: {}, {}", std::cmp::max(start, map.src), std::cmp::min(end, map_end));
                if map_end < end {
                    ranges.push((map_end, end));
                    //println!("unmapped range at end: {}, {}", map_end, end);
                }
                // We've handled this range by breaking it up, no need to
                // continue looking through the maplist
                break;
            }
            if !mapped {
                // Never found any mapping for the entire range, great!
                //println!("unmapped");
                next_ranges.push((start, end));
            }
        }
        ranges = next_ranges;
    }
    //println!("final ranges len: {}", ranges.len()); //
    let part2 = ranges.iter().min().ok_or("impossible condition")?.0;
    println!("Part 2: {}", part2);
    return_part1and2(*part1 as isize, part2 as isize)
}
