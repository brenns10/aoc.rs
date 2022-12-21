use std::collections::{HashSet, VecDeque};
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

fn is_external(c: C3D, points: &HashSet<C3D>, internal: &mut HashSet<C3D>, external: &mut HashSet<C3D>) -> bool {
    if points.contains(&c) {
        panic!("Should not be called on a point in the set.")
    }
    let mut seen: HashSet<_> = HashSet::new();
    let mut q: VecDeque<_> = VecDeque::new();
    q.push_front(c);
    seen.insert(c);
    let mut count = 0;
    while !q.is_empty() && count < points.len() {
        let c = q.pop_back().unwrap();
        count += 1;
        if external.contains(&c) {
            external.extend(seen);
            return true;
        }
        if internal.contains(&c) {
            internal.extend(seen);
            return false;
        }
        for dir in ADJACENT {
            let next = c + *dir;
            if !seen.contains(&next) && !points.contains(&next) {
                q.push_front(next);
                seen.insert(next);
            }
        }
    }
    /* It is impossible for N points to enclose N other points. */
    if count >= points.len() {
        external.extend(seen);
        true
    } else {
        internal.extend(seen);
        false
    }
}

fn external_surface_area(points: &HashSet<C3D>) -> u32 {
    let mut internal: HashSet<C3D> = HashSet::new();
    let mut external: HashSet<C3D> = HashSet::new();

    let mut sa = 0;

    for c in points.iter() {
        for dir in ADJACENT {
            let adj = *c + *dir;
            if !points.contains(&adj) && is_external(adj, points, &mut internal, &mut external) {
                sa += 1;
            }
        }
    }
    sa
}

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
    println!("Extarnal surface area: {}", external_surface_area(&input));
}
