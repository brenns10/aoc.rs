use std::fs::File;
use std::io::{self, Read, Result};

fn find_marker() -> Result<u32> {
    let mut buf = [0 as u8; 4];
    let br = io::BufReader::new(File::open("input.txt")?);
    for (i, c) in br.bytes().enumerate() {
        let c = c?;
        buf[i % 4] = c;
        let mut sbuf = buf.clone();
        sbuf.sort();
        println!("{} {} {:?} {:?}", i, c, buf, sbuf);
        if i >= 3 && (0..sbuf.len() - 1).all(|i| sbuf[i] != sbuf[i + 1]) {
            return Ok((i + 1) as u32);
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "Not Found"))
}

fn main() {
    let m = find_marker().unwrap();
    println!("Characters to read: {}", m);
}
