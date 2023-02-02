use std::cmp;
use std::env;
use std::error::Error;
use std::fs::File;
use std::collections::{HashSet, HashMap};
use std::io::BufRead;
use std::io::BufReader;
use std::ops::{Add, Sub};
use std::result::Result;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct C2D(isize, isize);

impl Add for C2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for C2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

const DIRECTIONS: &[C2D] = &[
    C2D(0, 1),  // north
    C2D(0, -1), // south
    C2D(-1, 0), // west
    C2D(1, 0),  // east
];

const SURROUND: &[C2D] = &[
    C2D(0, 1),   // N
    C2D(1, 1),   // NE
    C2D(1, 0),   // E
    C2D(1, -1),  // SE
    C2D(0, -1),  // S
    C2D(-1, -1), // SW
    C2D(-1, 0),  // W
    C2D(-1, 1),  // NW
];

const CLEAR: &[[C2D; 3]] = &[
    [C2D(-1, 1), C2D(0, 1), C2D(1, 1)],    // NW, N, NE
    [C2D(-1, -1), C2D(0, -1), C2D(1, -1)], // SW, S, SE
    [C2D(-1, 1), C2D(-1, 0), C2D(-1, -1)], // NW, W, SW
    [C2D(1, 1), C2D(1, 0), C2D(1, -1)],    // NE, E, SE
];

fn read_map(filename: &str) -> MyResult<HashSet<C2D>> {
    let mut y = 0;
    let reader = BufReader::new(File::open(filename)?);
    let mut res = HashSet::new();
    for line in reader.lines() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                res.insert(C2D(x as isize, y));
            }
        }
        y -= 1;
    }
    Ok(res)
}

fn do_turn(map: &mut HashSet<C2D>, turn: usize) -> isize {
    let mut moved = 0;

    // Propose moves into the map: destination: source
    let mut proposals: HashMap<C2D, Vec<C2D>> = HashMap::new();
    for key in map.iter() {
        // If the surrounding cells are all empty, do nothing
        if SURROUND.iter().all(|dir| !map.contains(&(*key + *dir))) {
            continue;
        }

        // Now consider each direction in order
        for i in 0..DIRECTIONS.len() {
            let dir = (turn + i) % DIRECTIONS.len();

            // Consider this direction: first, we must check
            // if all necessary cells are clear
            if CLEAR[dir].iter().any(|adj| map.contains(&(*key + *adj))) {
                continue;
            }

            // All necessary positions are clear, propose the move
            let newloc = *key + DIRECTIONS[dir];
            match proposals.get_mut(&newloc) {
                Some(val) => { val.push(*key); },
                None => { proposals.insert(newloc, vec![*key]); },
            };
            break;
        }
    }

    // Now, do the moves:
    for (dest, sources) in proposals.iter() {
        if sources.len() == 1 {
            let src = sources[0];
            map.remove(&src);
            map.insert(*dest);
            moved += 1;
        }
    }

    moved
}

fn min_max(map: &HashSet<C2D>) -> Option<(C2D, C2D)> {
    let mut min: Option<C2D> = None;
    let mut max: Option<C2D> = None;

    for coord in map {
        min = match min {
            None => Some(*coord),
            Some(other) => Some(C2D(cmp::min(other.0, coord.0), cmp::min(other.1, coord.1))),
        };
        max = match max {
            None => Some(*coord),
            Some(other) => Some(C2D(cmp::max(other.0, coord.0), cmp::max(other.1, coord.1))),
        };
    }
    if min.is_none() || max.is_none() {
        None
    } else {
        Some((min.unwrap(), max.unwrap()))
    }
}

fn print_map(map: &HashSet<C2D>) {
    let (min, max) = min_max(map).unwrap();
    // Iterate from high to low on Y
    for y in (min.1..max.1 + 1).rev() {
        for x in min.0..max.0+1 {
            if map.contains(&C2D(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn count_empty(map: &HashSet<C2D>) -> usize {
    let (min, max) = min_max(map).unwrap();
    let area = (max.0 - min.0 + 1) * (max.1 - min.1 + 1);
    (area as usize) - map.len()
}

fn main() {
    let mut filename = "input.txt";
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
    }
    let mut map = read_map(filename).unwrap();
    print_map(&map);
    for i in 0..10 {
        let moved = do_turn(&mut map, i);
        println!("In turn {i}, {moved} elves moved.");
        print_map(&map);
    }
    let empty = count_empty(&map);
    println!("At the end, {empty} squares were empty in minimal rectangle.");
}
