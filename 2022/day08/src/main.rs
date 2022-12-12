use std::error::Error;
use std::fs::File;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::result::Result;

type MyResult<T> = Result<T, Box<dyn Error>>;

struct RectArray {
    arr: Vec<u8>,
    cols: usize,
}

impl RectArray {
    fn from_file() -> MyResult<RectArray> {
        let file = File::open("input.txt")?;
        let reader = io::BufReader::new(file);
        let mut arr: Vec<u8> = Vec::new();
        let mut cols = 0;
        let mut first_line = true;

        for line in reader.lines() {
            let line = line?;
            for c in line.chars() {
                match c.to_digit(10) {
                    Some(num) => arr.push(num as u8),
                    None => {
                        return Err("Bad Content of input.txt".into());
                    }
                }
            }
            if first_line {
                cols = arr.len();
                first_line = false;
            } else {
                assert_eq!(arr.len() % cols, 0);
            }
        }
        Ok(RectArray{arr, cols})
    }
    fn rows(&self) -> usize {
        self.arr.len() / self.cols
    }
    fn get(&self, row: usize, col: usize) -> u8 {
        self.arr[row * self.cols + col]
    }
}

fn do_visible<I>(iter: I, arr: &RectArray, visible: &mut HashSet<(usize, usize)>)
  where I: Iterator<Item = (usize, usize)>
{
    let mut max_seen: i8 = -1;
    for (row, col) in iter {
        let val = arr.get(row, col) as i8;
        if val > max_seen {
            visible.insert((row, col));
            max_seen = val;
        }
    }
}

fn count_visible_trees(arr: &RectArray) -> usize {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for row in 0..arr.rows() {
        do_visible((0..arr.cols).map(|v| (row, v)), &arr, &mut visible);
        do_visible((0..arr.cols).rev().map(|v| (row, v)), &arr, &mut visible);
    }
    for col in 0..arr.cols {
        do_visible((0..arr.rows()).map(|v| (v, col)), &arr, &mut visible);
        do_visible((0..arr.rows()).rev().map(|v| (v, col)), &arr, &mut visible);
    }
    visible.len()
}

fn main() {
    let arr = RectArray::from_file().unwrap();
    let len = count_visible_trees(&arr);
    println!("Array of rows, cols: {}, {}", arr.rows(), arr.cols);
    println!("Number of visible trees: {}", len);
}
