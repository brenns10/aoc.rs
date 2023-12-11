use std::fs::File;
use std::io::Read;

use crate::util::read_ints;

fn count_ways_to_win(time: u32, record: u32) -> u32 {
    let mut ways = 0;

    for wait in 0..time {
        let score = wait * (time - wait);
        if score > record {
            ways += 1;
        }
    }

    ways
}

pub fn run(fln: &str) {
    let mut file = File::open(fln).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let line_end = contents.find("\n").unwrap();
    let times_str = &contents[..line_end];
    let dists_str = &contents[line_end + 1..];

    let colon = times_str.find(":").unwrap();
    let times: Vec<u32> = read_ints(&times_str[colon + 1..]).unwrap();

    let colon = dists_str.find(":").unwrap();
    let dists: Vec<u32> = read_ints(&dists_str[colon + 1..]).unwrap();

    assert!(times.len() == dists.len());

    let mut product = 1;
    for i in 0..times.len() {
        product *= count_ways_to_win(times[i], dists[i]);
    }
    println!("Part 1: {}", product);
}
