use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

fn seat_no(id: u16) -> u16 {
    id & 0x7
}

fn row_no(id: u16) -> u16 {
    id >> 3
}

fn parse_seat(line: &str) -> Result<u16, String> {
    let bstr: Result<String, String> =
        line.chars()
            .map(|x| match x {
                'F' => Ok('0'),
                'B' => Ok('1'),
                'L' => Ok('0'),
                'R' => Ok('1'),
                c => Err(format!("Invalid character '{}'", c))
            })
            .collect();
    bstr.map(|x| u16::from_str_radix(&x, 2).unwrap())
}

fn read_bytes(filename: &str) -> Result<Vec<u16>, String> {
    let f = File::open(filename).map_err(|e| e.to_string())?;
    let br = BufReader::new(f);
    br.lines()
      .map(|x| x.map_err(|e| e.to_string()))
      .map(|x| x.and_then(|s| if s.len() != 8 {Ok(s)} else {Err(format!("incorrect len"))}))
      .map(|x| x.and_then(|s| parse_seat(&s)))
      .collect()
}

fn main() {
    println!("Advent of Code Day 5!");
    let mut ids = read_bytes("input.txt").unwrap();
    ids.sort();
    println!("max id: {}", ids[ids.len() - 1]);
    let mut first = true;
    let mut prev: u16 = 0;
    for byte in ids {
        if first {
            first = false;
        } else {
            if byte != prev + 1 {
                println!("Skip: {} -> {} ({}/{} -> {}/{})", prev, byte, row_no(prev), seat_no(prev), row_no(byte), seat_no(byte));
            }
        }
        prev = byte;
    }
}
