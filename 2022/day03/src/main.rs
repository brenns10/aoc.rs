use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::vec::Vec;
//use std::str;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn read_rucksacks() -> MyResult<Vec<(Vec<u8>, Vec<u8>)>> {
    let mut rucksacks: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for line in io::BufReader::new(File::open("input.txt")?).lines() {
        let mut all_items: Vec<u8> = Vec::from(line?);
        let mut second_compartment = all_items.split_off(all_items.len() / 2);
        all_items.sort();
        second_compartment.sort();
        rucksacks.push((all_items, second_compartment));
    }
    Ok(rucksacks)
}

/* Given two compartments, find the item present in both */
fn common_piece(lhs: Vec<u8>, rhs: Vec<u8>) -> Option<u8> {
    //println!("{} {}", str::from_utf8(&lhs).unwrap(), str::from_utf8(&rhs).unwrap());
    let lhs_iter = &mut lhs.into_iter();
    let rhs_iter = &mut rhs.into_iter();

    let mut lhs_val = lhs_iter.next()?;
    let mut rhs_val = rhs_iter.next()?;
    while lhs_val != rhs_val {
        if lhs_val < rhs_val {
            lhs_val = lhs_iter.next()?;
        } else {
            rhs_val = rhs_iter.next()?;
        }
    }
    //println!("{}", lhs_val as char);
    Some(lhs_val)
}

fn priority(item: u8) -> MyResult<u32> {
    if b'a' <= item && item <= b'z' {
        Ok((item - b'a' + 1) as u32)
    } else if b'A' <= item && item <= b'Z' {
        Ok((item - b'A' + 27) as u32)
    } else {
        Err("Invalid priority".into())
    }
}

fn main() {
    let rucksacks = read_rucksacks().unwrap();
    let total: u32 = rucksacks.into_iter().map(|t| priority(common_piece(t.0, t.1).unwrap()).unwrap()).sum();
    println!("Sum of common item priorities: {}", total);
}
