use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter;
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

const  LEFT: u8 = 0x1;
const RIGHT: u8 = 0x2;
const    UP: u8 = 0x4;
const  DOWN: u8 = 0x8;

const DIRECTIONS: &[u8] = &[
    LEFT,
    RIGHT,
    UP,
    DOWN,
];

const  CLEFT: C2D = C2D(0, -1);
const CRIGHT: C2D = C2D(0, 1);
const    CUP: C2D = C2D(-1, 0);
const  CDOWN: C2D = C2D(1, 0);

const COORD_DIRECTIONS: &[C2D] = &[
    CLEFT,
    CRIGHT,
    CUP,
    CDOWN,
];

#[derive(Clone)]
struct Map {
    arr: Vec<u8>,
    width: usize,
    height: usize,
    start_col: usize,
    end_col: usize,
}

impl Map {
    fn ix(&self, coord: &C2D) -> usize {
        if coord.0 < 0 || coord.1 < 0 {
            panic!("Negative coordinate: {:?}", coord);
        }
        coord.0 as usize + coord.1 as usize * self.height
    }
    fn read(filename: &str) -> MyResult<Map> {
        let mut contents = String::new();
        File::open(filename).unwrap().read_to_string(&mut contents).unwrap();
        let lines: Vec<_> = contents.lines().collect();
        let width = lines[0].len() - 2;
        let height = lines.len() - 2;
        let start_col = lines[0].find('.').ok_or("Missing starting place")? - 1;
        let end_col = lines[lines.len() - 1].find('.').ok_or("Missing ending place")? - 1;
        let arr: Vec<u8> = iter::repeat(0).take(width * height).collect();
        let mut map = Map{arr, width, height, start_col, end_col};
        for (row, line) in lines[1..lines.len()-1].iter().enumerate() {
            for (col, c) in line[1..line.len()-1].chars().enumerate() {
                let cell = match c {
                    '<' => LEFT,
                    '>' => RIGHT,
                    '^' => UP,
                    'v' => DOWN,
                    '.' => 0,
                    _ => return Err(format!("invalid char: {}", c).into()),
                };
                let coord = C2D(row as isize, col as isize);
                let ix = map.ix(&coord);
                if ix == 3000 {
                    println!("coord: {:?}", coord);
                }
                map.arr[ix] = cell;
            }
        }
        Ok(map)
    }
    fn print(&self, locations: &HashSet<C2D>) {
        for row in 0..self.height {
            for col in 0..self.width {
                let coord = C2D(row as isize, col as isize);
                let cell = self.get(&coord);
                let mut c = match cell {
                    0 => '.',
                    LEFT => '<',
                    RIGHT => '>',
                    UP => '^',
                    DOWN => 'v',
                    _ => '?',
                };
                if locations.contains(&coord) {
                    if c != '.' {
                        println!("Bad position for party")
                    }
                    c = 'E';
                }
                print!("{}", c);
            }
            print!("\n");
        }
    }
    fn get(&self, coord: &C2D) -> u8 {
        let ix = self.ix(coord);
        self.arr[ix]
    }
    fn set(&mut self, coord: &C2D, dir: u8) {
        let delta = match dir {
            UP => CUP,
            DOWN => CDOWN,
            LEFT => CLEFT,
            RIGHT => CRIGHT,
            _ => {panic!("Bad direction")}
        };
        let next = *coord + delta + C2D(self.height as isize, self.width as isize);
        let normed = C2D(next.0 % self.height as isize, next.1 % self.width as isize);
        let ix = self.ix(&normed);
        self.arr[ix] |= dir;
    }
    fn in_bounds(&self, coord: &C2D) -> bool {
        (0 <= coord.0) && (coord.0 < self.height as isize) && (0 <= coord.1) && (coord.1 < self.width as isize)
    }
    fn step_blizzard(&self) -> Map {
        let mut next = Map{
            arr: iter::repeat(0).take(self.width * self.height).collect(),
            width: self.width,
            height: self.height,
            start_col: self.start_col,
            end_col: self.end_col,
        };

        for row in 0..self.height {
            for col in 0..self.width {
                let coord = C2D(row as isize, col as isize);
                let val = self.get(&coord);
                for dir in DIRECTIONS {
                    if (val & *dir) != 0 {
                        next.set(&coord, *dir);
                    }
                }
            }
        }

        next
    }
}

fn do_search(states: &Vec<Map>, verbose: bool) -> usize {
    let start = C2D(-1, states[0].start_col as isize);
    let end = C2D(states[0].height as isize, states[0].end_col as isize);

    let mut cur: HashSet<C2D> = HashSet::new();
    cur.insert(start);

    let mut time = 0;
    loop {
        let mut next: HashSet<C2D> = HashSet::new();
        time += 1;
        if verbose {
            println!("Time {}", time);
        }

        let next_map = &states[time % states.len()];

        for coord in cur {
            if coord.0 < 0 {
                // Oops, starting position, can only move down or wait
                next.insert(coord);
                let down = coord + CDOWN;
                if next_map.get(&down) == 0 {
                    next.insert(coord + CDOWN);
                }
            } else {
                // Ok, try to move any direction
                if next_map.get(&coord) == 0 {
                    next.insert(coord); // we can only wait if no blizzard
                }
                for dir in COORD_DIRECTIONS {
                    let new_coord = coord + *dir;
                    if new_coord == end {
                        return time; // WE WIN
                    }
                    if !next_map.in_bounds(&new_coord) {
                        continue;
                    }
                    if next_map.get(&new_coord) == 0 {
                        next.insert(new_coord);
                    }
                }
            }
        }

        if verbose {
            next_map.print(&next);
            println!();
        }

        cur = next;
    }
}

fn main() {
    let mut filename = "input.txt";
    let mut verbose = false;
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
        verbose = true;
    }

    /*
     * The blizzard will repeat every MxN ticks. Though, you never know, maybe
     * there could be a smaller period if we got lucky. Precompute the blizzard
     * states so that we don't need to worry about it later on.
     */
    let mut steps = vec![Map::read(filename).unwrap()];
    loop {
        let next = steps[steps.len() - 1].step_blizzard();
        if next.arr == steps[0].arr {
            break;
        }
        steps.push(next);
    }

    /*
     * Now that we have the blizzard states, we need to do a search. The best
     * way to limit the (possibly exponential) search space is to simulate all
     * possible paths at once, in lockstep, synchronized by the current time.
     * The first one which can come up with the solution is the winner.
     */
    let time = do_search(&steps, verbose);
    println!("Minimum time: {}", time);
}
