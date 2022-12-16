use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::result::Result;
use std::vec::Vec;
use std::ops::Range;
use itertools::Itertools;
use std::cmp;
use std::thread;
use std::time::Duration;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn from_coord_str(string: &str) -> MyResult<Self> {
        if let Some((x, y)) = string.split(",").collect_tuple() {
            Ok(Coord{
                x: isize::from_str_radix(x, 10)?,
                y: isize::from_str_radix(y, 10)?,
            })
        } else {
            Err("Invalid coordinate string".into())
        }
    }

    fn line_between(c1: &Self, c2: &Self) -> Option<Vec<Coord>> {
        if c1.x == c2.x {
            Some((cmp::min(c1.y, c2.y)..(cmp::max(c1.y, c2.y)+1)).map(|y| Coord{x: c1.x, y}).collect())
        } else if c1.y == c2.y {
            Some((cmp::min(c1.x, c2.x)..(cmp::max(c1.x, c2.x)+1)).map(|x| Coord{x, y: c1.y}).collect())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct RectArray<T>
where T: Copy
{
    x: Range<isize>,
    y: Range<isize>,
    arr: Box<[T]>,
}

impl<T> RectArray<T>
   where T: Copy {
    fn x_range(&self) -> Range<isize> {
        self.x.clone()
    }
    fn y_range(&self) -> Range<isize> {
        self.y.clone()
    }
    fn new_ranged(x: &Range<isize>, y: &Range<isize>, fill: &T) -> RectArray<T> {
        let arr: Vec<T> = (0..x.len()*y.len()).map(|_| *fill).collect();
        RectArray{arr: arr.into_boxed_slice(), x: x.clone(), y: y.clone()}
    }
    fn index(&self, c: &Coord) -> usize {
        if !self.x.contains(&c.x) {
            panic!("Accessing RectArray with x={} out of bounds ({}, {})", c.x, self.x.start, self.x.end);
        }
        if !self.y.contains(&c.y) {
            panic!("Accessing RectArray with y={} out of bounds ({}, {})", c.x, self.y.start, self.y.end);
        }
        let x = (c.x - self.x.start) as usize;
        let y = (c.y - self.y.start) as usize;
        y * self.x.len() + x
    }
    pub fn get(&self, c: &Coord) -> &T {
        &self.arr[self.index(&c)]
    }
    pub fn set(&mut self, c: &Coord, val: T) {
        self.arr[self.index(&c)] = val;
    }
}

#[derive(Clone, Copy, Debug)]
enum CaveBlock {
    Air,
    Rock,
    Sand,
}

fn find_ranges(paths: &Vec<Vec<Coord>>) -> (Range<isize>, Range<isize>) {
    let mut x_range = isize::max_value()..isize::min_value();
    let mut y_range = x_range.clone();
    for path in paths.iter() {
        for c in path.iter() {
            if x_range.start > c.x {
                x_range.start = c.x
            }
            if x_range.end <= c.x {
                x_range.end = c.x + 1
            }
            if y_range.start > c.y {
                y_range.start = c.y
            }
            if y_range.end <= c.y {
                y_range.end = c.y + 1
            }
        }
    }
    (x_range, y_range)
}

fn draw_paths(paths: &Vec<Vec<Coord>>, arr: &mut RectArray<CaveBlock>) {
    for path in paths.iter() {
        for (start, end) in path.iter().tuple_windows() {
            for coord in Coord::line_between(start, end).unwrap().iter() {
                arr.set(coord, CaveBlock::Rock);
            }
        }
    }
}

fn print_cave(arr: &RectArray<CaveBlock>) {
    use CaveBlock::*;
    print!("\x1B[2J\x1B[1;1H");
    for y in arr.y_range() {
        for x in arr.x_range() {
            match arr.get(&Coord{x, y}) {
                Air => { print!(" ") },
                Rock => { print!("#") },
                Sand => { print!("O") },
            }
        }
        print!("\n");
    }
}

fn drop_sand(arr: &mut RectArray<CaveBlock>, floor: bool) -> bool {
    use CaveBlock::*;
    let mut coord = Coord{x: 500, y: 0};

    while arr.y_range().contains(&(coord.y + 1)) {
        let below = Coord{x: coord.x, y: coord.y + 1};
        if let Air = arr.get(&below) {
            coord = below;
            continue;
        }
        let below_left = Coord{x: coord.x - 1, y: coord.y + 1};
        if let Air = arr.get(&below_left) {
            coord = below_left;
            continue;
        }
        let below_right = Coord{x: coord.x + 1, y: coord.y + 1};
        if let Air = arr.get(&below_right){
            coord = below_right;
            continue;
        }
        if let Air = arr.get(&coord) {
            arr.set(&coord, Sand);
           return true;
        } else {
            return false;
        }
    }
    if floor {
        if let Air = arr.get(&coord) {
            arr.set(&coord, Sand);
            return true;
        }
    }
    false
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let verbose = false;
    let more_verbose = false;
    let paths: Vec<Vec<Coord>> = reader
        .lines()
        .map(|l| l.unwrap().split(" -> ")
                           .map(Coord::from_coord_str)
                           .collect::<MyResult<Vec<Coord>>>().unwrap())
        .collect();

    let (mut x_range, mut y_range) = find_ranges(&paths);
    if y_range.start > 0 {
        y_range.start = 0
    }
    y_range.end += 1;
    // Add enough space to have a full pile on etiher side
    x_range.start -= y_range.len() as isize;
    x_range.end += y_range.len() as isize;
    let mut cave = RectArray::new_ranged(&x_range, &y_range, &CaveBlock::Air);
    draw_paths(&paths, &mut cave);
    let mut settled = 0;
    if verbose { print_cave(&cave); }
    while drop_sand(&mut cave, false) {
        settled += 1;
        if verbose && settled % 10 == 0 {
            thread::sleep(Duration::from_millis(50));
            print_cave(&cave);
        }
    }
    println!("Cave has {} settled sand blocks before sand falls into the void (ground)", settled);
    while drop_sand(&mut cave, true) {
        settled += 1;
        if more_verbose && settled % 1000 == 0 {
            thread::sleep(Duration::from_millis(50));
            print_cave(&cave);
        }
    }
    if more_verbose {
        print_cave(&cave);
    }
    println!("Cave has {} settled sand blocks before the source is plugged", settled);
}
