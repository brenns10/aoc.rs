use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter;
use std::ops::{Add, Sub};
use std::result::Result;

use regex::Regex;

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

const FACINGS: &[C2D] = &[
    C2D(1, 0),  // right
    C2D(0, 1),  // down
    C2D(-1, 0), // left
    C2D(0, -1), // up
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Offmap,
    Open,
    Wall,
}

struct Map {
    arr: Vec<Cell>,
    width: usize,
    height: usize,
    start: C2D,
}

impl Map {
    fn ix(&self, coord: &C2D) -> usize {
        if coord.0 < 0 || coord.1 < 0 {
            panic!("Negative coordinate");
        }
        coord.0 as usize + coord.1 as usize * self.width
    }
    fn read_string(s: &str) -> MyResult<Map> {
        let lines: Vec<_> = s.lines().collect();
        let width = lines.iter().map(|s| s.len()).max().unwrap() + 1;
        let height = lines.len() + 1;
        let arr: Vec<Cell> = iter::repeat(Cell::Offmap).take(width * height).collect();
        let mut map = Map{arr, width, height, start: C2D(0, 0)};
        let mut first = true;
        for (y, line) in lines.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                let cell = match c {
                    ' ' => continue,
                    '.' => Cell::Open,
                    '#' => Cell::Wall,
                    _ => return Err("invalid char".into()),
                };
                let coord = C2D(i as isize + 1, y as isize + 1);
                if first && cell == Cell::Open {
                    map.start = coord;
                    first = false;
                }
                let ix = map.ix(&coord);
                map.arr[ix] = cell;
            }
        }
        Ok(map)
    }
    fn get(&self, coord: &C2D) -> Cell {
        let ix = self.ix(coord);
        self.arr[ix]
    }
    fn in_bounds(&self, coord: &C2D) -> bool {
        (0 <= coord.0) && (coord.0 < self.width as isize) && (0 <= coord.1) && (coord.1 < self.height as isize)
    }
    fn mov(&self, coord: C2D, dir: usize, count: usize) -> C2D {
        let mut cur = coord;
        let dirc = FACINGS[dir];
        for _ in 0..count {
            let next = cur + dirc;
            if !self.in_bounds(&next) || self.get(&next) == Cell::Offmap {
                let mut bt = cur;
                while self.in_bounds(&(bt - dirc)) && self.get(&(bt - dirc)) != Cell::Offmap {
                    bt = bt - dirc;
                }
                if self.get(&bt) == Cell::Open {
                    cur = bt;
                }
            } else if self.get(&next) == Cell::Open {
                cur = next;
            } else {
                break;
            }
        }
        cur
    }
}

enum Instruction {
    Left,
    Right,
    Move(usize),
}

fn read_input(filename: &str) -> MyResult<(Map, Vec<Instruction>)> {
    let mut reader = File::open(filename)?;
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let (map, inst) = contents.split_once("\n\n").ok_or_else(|| "Bad map separation")?;
    let mapval = Map::read_string(map)?;
    let expr = Regex::new(r"(\d+)|L|R")?;
    let mut instrs = Vec::new();
    for v in expr.find_iter(inst) {
        if v.as_str() == "L" {
            instrs.push(Instruction::Left);
        } else if v.as_str() == "R" {
            instrs.push(Instruction::Right);
        } else {
            instrs.push(Instruction::Move(usize::from_str_radix(v.as_str(), 10)?));
        }
    }
    Ok((mapval, instrs))
}

fn do_path(map: &Map, instrs: &Vec<Instruction>) {
    let mut facing: usize = 0;
    let mut coord = map.start;
    for instr in instrs.iter() {
        use Instruction::*;
        match instr {
            Left => { facing = (facing + FACINGS.len() - 1) % FACINGS.len() },
            Right => { facing = (facing + 1) % FACINGS.len() },
            Move(amt) => {
                coord = map.mov(coord, facing, *amt)
            }
        }
    }
    println!("Final row={}, column={}, facing={}", coord.1, coord.0, facing);
    println!("Password: {}", coord.1 * 1000 + coord.0 * 4 + facing as isize);
}

fn main() {
    let mut filename = "input.txt";
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
    }
    let (map, instrs) = read_input(filename).unwrap();
    do_path(&map, &instrs);
}
