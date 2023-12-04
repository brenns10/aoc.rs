use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::ops::{Add, Sub};
use std::hash::Hash;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Hash)]
pub struct Point(isize, isize);

pub const DIRECTIONS: [Point; 8] = [
    Point(-1, -1), Point(-1,  0), Point(-1, 1),
    Point( 0, -1),                Point( 0, 1),
    Point( 1, -1), Point( 1,  0), Point( 1, 1),
];

impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1
    }
}
impl Eq for Point {}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub struct Adjacent {
    pt: Point,
    rows: isize,
    cols: isize,
    direction: usize,
}

impl Iterator for Adjacent {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        for dir_idx in self.direction..DIRECTIONS.len() {
            let adj = self.pt + DIRECTIONS[dir_idx];
            if adj.0 < 0 || adj.0 >= self.rows || adj.1 < 0 || adj.1 >= self.cols {
                continue
            }
            self.direction = dir_idx + 1;
            return Some(adj)
        }
        None
    }
}

pub struct Arr2D {
    pub rows: isize,
    pub cols: isize,
    data: Vec<char>,
}

impl Arr2D {
    pub fn read_file(fln: &str) -> Result<Arr2D, Error> {
        let reader = BufReader::new(File::open(fln)?);
        let mut lines = reader.lines();
        let first_line =
            lines
            .next()
            .ok_or(Error::other("Missing data: need at least one line"))??;
        let cols = first_line.len() as isize;
        let mut data: Vec<char> = first_line.chars().collect();
        let mut rows = 1;
        for line in lines {
            let line = line?;
            if line.len() == 0 {
                break;
            } else if line.len() as isize != cols {
                return Err(Error::other("Invalid array: column count incorrect"));
            }
            rows += 1;
            data.extend(line.chars());
        }
        Ok(Arr2D{rows, cols, data})
    }

    pub fn in_bounds(&self, ix: Point) -> bool {
        return ix.0 >= 0 && ix.0 < self.rows && ix.1 >= 0 && ix.1 < self.cols
    }

    pub fn to_index(&self, ix: Point) -> usize {
        if !self.in_bounds(ix) {
            panic!("({}, {}): index out of range (bounds: ({}, {}))", ix.0, ix.1, self.rows, self.cols);
        }
        (ix.0 * self.cols + ix.1) as usize
    }

    pub fn get(&self, ix: Point) -> char {
        self.data[self.to_index(ix)]
    }

    pub fn get_at(&self, row: isize, col: isize) -> char {
        self.get(Point(row, col))
    }

    pub fn adjacent(&self, ix: Point) -> Adjacent {
        Adjacent{pt: ix, rows: self.rows, cols: self.cols, direction: 0}
    }

    pub fn adjacent_to(&self, row: isize, col: isize) -> Adjacent {
        self.adjacent(Point(row, col))
    }
}

fn main() {
    let arr = Arr2D::read_file("input.txt").unwrap();
    let mut part_number_sum = 0;
    let mut gear_to_numbers: HashMap<Point, Vec<u32>> = HashMap::new();
    let mut gears: HashSet<Point> = HashSet::new();
    for row in 0..arr.rows {
        let mut number: u32 = 0;
        let mut is_part = false;
        for col in 0..arr.cols {
            if let Some(val) = arr.get_at(row, col).to_digit(10) {
                number = number * 10 + val;
                for point in arr.adjacent_to(row, col) {
                    let cell = arr.get(point);
                    if !cell.is_digit(10) && cell != '.' {
                        is_part = true;
                    }
                    if cell == '*' {
                        gears.insert(point);
                    }
                }
            } else if number > 0 {
                if is_part {
                    part_number_sum += number;
                    for gear in gears.iter() {
                        if let Some(vec) = gear_to_numbers.get_mut(gear) {
                            vec.push(number);
                        } else {
                            gear_to_numbers.insert(*gear, vec![number]);
                        }
                    }
                }
                number = 0;
                is_part = false;
                gears.clear();
            }
        }
        if number > 0 && is_part {
            part_number_sum += number;
            for gear in gears.iter() {
                if let Some(vec) = gear_to_numbers.get_mut(gear) {
                    vec.push(number);
                } else {
                    gear_to_numbers.insert(*gear, vec![number]);
                }
            }
        }
        gears.clear();
    }
    println!("Part 1: {}", part_number_sum);

    let mut sum_ratios = 0;
    for (_, numbers) in gear_to_numbers.iter() {
        if numbers.len() == 2 {
            sum_ratios += numbers[0] * numbers[1];
        }
    }
    println!("Part 2: {}", sum_ratios);
}
