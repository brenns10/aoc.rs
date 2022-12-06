use std::fs::File;
use std::io::{self, Read, Result};
use std::iter;

/// Find the first occurrence of a sliding window of `size` bytes which are
/// all different from each other in `br`.
fn find_marker<'a, R>(br: R, size: usize) -> Result<u32>
    where R: Iterator<Item = &'a u8>
{
    let mut buf = Vec::from_iter(iter::repeat(0 as u8).take(size));
    for (i, c) in br.enumerate() {
        buf[i % size] = *c;
        let mut sbuf = buf.clone();
        sbuf.sort();
        if i >= size - 1 && (0..sbuf.len() - 1).all(|i| sbuf[i] != sbuf[i + 1]) {
            return Ok((i + 1) as u32);
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "Not Found"))
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = file.bytes().collect::<Result<Vec<u8>>>().unwrap();
    let m = find_marker(buf.iter(), 4).unwrap();
    println!("Characters to read, window 4: {}", m);
    let m = find_marker(buf.iter(), 14).unwrap();
    println!("Characters to read, window 14: {}", m);
}
