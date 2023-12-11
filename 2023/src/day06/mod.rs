use std::fs::File;
use std::io::Read;

use crate::util::MyResult;
use crate::util::read_ints;
use crate::util::return_part1and2;
use crate::util::RunResult;

fn combine_numbers(nums: &Vec<usize>) -> MyResult<usize> {
    let mut combined: usize = 0;
    for num in nums {
        let mut found = false;
        for pow in 1..10 {
            if *num < usize::pow(10, pow) {
                combined = usize::pow(10, pow) * combined + *num;
                found = true;
                break
            }
        }
        if !found {
            return Err("numbers too large to combine".into());
        }
    }
    Ok(combined)
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

pub fn run(fln: &str) -> RunResult {
    let mut file = File::open(fln)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let line_end = contents.find("\n").ok_or("missing newline")?;
    let times_str = &contents[..line_end];
    let dists_str = &contents[line_end + 1..];

    let colon = times_str.find(":").ok_or("missing colon")?;
    let times: Vec<usize> = read_ints(&times_str[colon + 1..])?;

    let colon = dists_str.find(":").ok_or("missing colon")?;
    let dists: Vec<usize> = read_ints(&dists_str[colon + 1..])?;

    assert!(times.len() == dists.len());

    let mut product = 1;
    for i in 0..times.len() {
        product *= ways_to_win_bsearch(times[i] as usize, dists[i] as usize);
    }
    println!("Part 1: {}", product);

    let time = combine_numbers(&times)?;
    let dist = combine_numbers(&dists)?;
    let part2 = ways_to_win_bsearch(time, dist);
    println!("Part 2: {}", ways_to_win_bsearch(time, dist));

    return_part1and2(product as isize, part2 as isize)
}
