use std::fs::File;
use std::io::Read;

use crate::util::read_ints;

fn combine_numbers(nums: &Vec<usize>) -> usize {
    let mut combined: usize = 0;
    for num in nums {
        if *num < 10 {
            combined = 10 * combined + *num as usize;
        } else if *num < 100 {
            combined = 100 * combined + *num as usize;
        } else if *num < 1000 {
            combined = 1000 * combined + *num as usize;
        } else if *num < 10000 {
            combined = 10000 * combined + *num as usize;
        } else {
            panic!("Numbers too large to combine");
        }
    }
    combined
}

fn ways_to_win_bsearch(time: usize, record: usize) -> usize {
    let best = time / 2;

    let mut upper = best;
    let mut lower = 0;

    while lower < upper {
        let mid = (upper + lower) / 2;
        let score = mid * (time - mid);
        if score > record {
            upper = mid - 1;
        } else {
            lower = mid + 1;
        }
    }
    // The loop is over: lower == upper.
    let score = upper * (time - upper);
    let lower_bound = if score > record {upper} else {upper + 1};

    upper = time;
    lower = best;
    while lower < upper {
        let mid = (upper + lower) / 2;
        let score = mid * (time - mid);
        if score > record {
            lower = mid + 1;
        } else {
            upper = mid - 1;
        }
    }
    let score = upper * (time - upper);
    let upper_bound = if score > record {upper} else {upper - 1};

    upper_bound - lower_bound + 1
}

pub fn run(fln: &str) {
    let mut file = File::open(fln).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let line_end = contents.find("\n").unwrap();
    let times_str = &contents[..line_end];
    let dists_str = &contents[line_end + 1..];

    let colon = times_str.find(":").unwrap();
    let times: Vec<usize> = read_ints(&times_str[colon + 1..]).unwrap();

    let colon = dists_str.find(":").unwrap();
    let dists: Vec<usize> = read_ints(&dists_str[colon + 1..]).unwrap();

    assert!(times.len() == dists.len());

    let mut product = 1;
    for i in 0..times.len() {
        product *= ways_to_win_bsearch(times[i] as usize, dists[i] as usize);
    }
    println!("Part 1: {}", product);

    let time = combine_numbers(&times);
    let dist = combine_numbers(&dists);
    println!("Part 2: {}", ways_to_win_bsearch(time, dist));
}
