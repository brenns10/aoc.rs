use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut count_full = 0;
    let mut count_part = 0;
    for line in io::BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split(['-', ',']).collect();
        assert_eq!(fields.len(), 4);
        let fields: Vec<u32> = fields.iter().map(|s| u32::from_str_radix(s, 10).unwrap()).collect();
        if ((fields[0] <= fields[2]) && (fields[3] <= fields[1])) ||
           ((fields[2] <= fields[0]) && (fields[1] <= fields[3])) {
            count_full += 1;
        }
        if (fields[1] >= fields[2]) && (fields[0] <= fields[3]) {
            count_part += 1;
        }
    }
    println!("Pairs fully contained: {}", count_full);
    println!("Pairs partially contained: {}", count_part);
}
