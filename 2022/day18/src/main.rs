use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::{Add, Sub};
use std::num::ParseIntError;
use std::result::Result;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct C3D(i32, i32, i32);

impl Add for C3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for C3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

fn read_input(filename: &str) -> MyResult<Vec<C3D>> {
    let reader = BufReader::new(File::open(filename)?);
    let mut vec =  Vec::new();
    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<i32> = line.split(",")
                                   .map(|v| i32::from_str_radix(v, 10))
                                   .collect::<Result<_, ParseIntError>>()?;
        assert_eq!(tokens.len(), 3);
        vec.push(C3D(tokens[0], tokens[1], tokens[2]));
    }
    Ok(vec)
}

const ADJACENT: &[C3D] = &[
    C3D(0, 0, 1),
    C3D(0, 0, -1),
    C3D(0, 1, 0),
    C3D(0, -1, 0),
    C3D(1, 0, 0),
    C3D(-1, 0, 0),
];

fn main() {
    let input = read_input("input.txt").unwrap();
    let input: HashSet<_> = input.iter().map(|v| *v).collect();
    let mut uncovered = 0;
    for c in input.iter() {
        for dir in ADJACENT {
            let adj = *c + *dir;
            if !input.contains(&adj) {
                uncovered += 1;
            }
        }
    }
    println!("Surface area: {}", uncovered);
}
