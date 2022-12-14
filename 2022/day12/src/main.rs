use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::result::Result;
use std::vec::Vec;
use std::collections::VecDeque;
use std::{thread,time};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

struct CoordAdjacent {
    start: Coord,
    state: u8,
    max_row: usize,
    max_col: usize,
}

impl Coord {
    fn adjacent<T>(&self, arr: &RectArray<T>) -> CoordAdjacent {
        CoordAdjacent{start: *self, state: 0, max_row: arr.rows() - 1, max_col: arr.cols - 1}
    }
}

impl Iterator for CoordAdjacent {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        while self.state < 4 {
            let old = self.state;
            self.state += 1;
            match old {
                0 if self.start.row > 0 => {return Some(Coord{row: self.start.row - 1, col: self.start.col})},
                1 if self.start.row < self.max_row => {return Some(Coord{row: self.start.row + 1, col: self.start.col})},
                2 if self.start.col > 0 => {return Some(Coord{row: self.start.row, col: self.start.col - 1})},
                3 if self.start.col < self.max_col => {return Some(Coord{row: self.start.row, col: self.start.col + 1})},
                _ => {},
            };
        }
        None
    }
}

#[derive(Clone, Debug)]
struct RectArray<T> {
    arr: Vec<T>,
    cols: usize,
    start: Coord,
    end: Coord,
}

impl RectArray<u8> {
    fn from_topo_map(filename: &str) -> MyResult<RectArray<u8>> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut arr: Vec<u8> = Vec::new();
        let mut cols = 0;
        let mut first_line = true;
        let mut start = Coord{row: 0, col: 0};
        let mut end = start;

        for line in reader.lines() {
            let line = line?;
            for c in line.chars() {
                if c.is_ascii_lowercase() {
                    arr.push((c as u8) - ('a' as u8));
                } else if c == 'S' {
                    arr.push(0);
                    start.row = (arr.len() - 1) / cols;
                    start.col = (arr.len() - 1) % cols;
                } else if c == 'E' {
                    arr.push(25);
                    end.row = (arr.len() - 1) / cols;
                    end.col = (arr.len() - 1) % cols;
                } else {
                    return Err(format!("Grid value is not a-z: '{}'", c).into());
                }
            }
            if first_line {
                cols = arr.len();
                first_line = false;
            } else {
                assert_eq!(arr.len() % cols, 0);
            }
        }
        Ok(RectArray{arr, cols, start, end})
    }
}

impl<T> RectArray<T> {
    fn rows(&self) -> usize {
        self.arr.len() / self.cols
    }
}

impl<T> RectArray<T>
   where T: Copy {
    fn new(rows: usize, cols: usize, fill: T) -> RectArray<T> {
        let arr: Vec<T> = (0..rows*cols).map(|_| fill).collect();
        let empty = Coord{row: 0, col: 0};
        RectArray{arr, cols, start: empty, end: empty}
    }
    fn get(&self, c: Coord) -> &T {
        &self.arr[c.row * self.cols + c.col]
    }
    fn set(&mut self, c: Coord, val: T) {
        self.arr[c.row * self.cols + c.col] = val;
    }
}

fn do_shortest_path(topo: &RectArray<u8>) -> Option<usize> {
    let mut q: VecDeque<Coord> = VecDeque::new();
    let mut paths: RectArray<Option<usize>> = RectArray::new(topo.rows(), topo.cols, None);
    paths.set(topo.start, Some(0));
    q.push_back(topo.start);

    let mut prev_len = 0;
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        let height = *topo.get(cur);
        let len = paths.get(cur).unwrap();
        if len != prev_len {
            print_path_arr(&paths, len);
            prev_len = len;
        }
        if cur == topo.end {
            return Some(len)
        }
        for step in cur.adjacent(topo) {
            if *topo.get(step) > height + 1 {
                continue;
            }
            if let None = paths.get(step) {
                paths.set(step, Some(len + 1));
                q.push_back(step);
            }
        }
    }
    None
}

fn print_path_arr(paths: &RectArray<Option<usize>>, cur: usize) {
    // I really don't want to deal with ncurses, this seems "good enough"
    print!("\x1B[2J\x1B[1;1H");
    for row in 0..paths.rows() {
        for col in 0..paths.cols {
            match paths.get(Coord{row, col}) {
                None => { print!(" "); },
                Some(len) if *len < cur => { print!("."); },
                Some(_) => { print!("#"); }
            }
        }
        print!("\n");
    }
    print!("\n");
    thread::sleep(time::Duration::from_millis(50));
}

fn main() {
    let topo = RectArray::from_topo_map("input.txt").unwrap();
    let shortest = do_shortest_path(&topo).unwrap();
    println!("Shortest path from start to end: {}", shortest);
}
