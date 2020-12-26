use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

fn read_map(filename: &str) -> Result<Vec<Vec<char>>, String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut cols: usize = 0;
    let mut first = true;
    let mut rows: Vec<Vec<char>> = Vec::new();

    for (line_no, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| e.to_string())?;
        let chars: Vec<char> = line.chars().collect();
        for c in &chars {
            if *c != '#' && *c != '.' {
                return Err(format!("Invalid character {} at line {}", c, line_no + 1));
            }
        }
        if first {
            cols = chars.len();
            first = false;
        }
        if chars.len() != cols {
            return Err(format!("At line {}, expected {} chars but found {}",
                               line_no + 1, cols, chars.len()));
        }
        rows.push(chars);
    }
    Ok(rows)
}

fn count_trees(map: &Vec<Vec<char>>, down: usize, right: usize) -> usize {
    let mut col = 0;
    let mut row = 0;
    let mut trees = 0;
    let cols = map[0].len();
    while row < map.len() {
        if map[row][col] == '#' {
            trees += 1;
        }
        col = (col + right) % cols;
        row += down;
    }
    trees
}

fn main() {
    let map = read_map("input.txt").unwrap();
    let slope = 3;
    println!("Trees in slope {}: {}", slope, count_trees(&map, 1, slope));
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut trees = 1;
    for (down, right) in slopes.iter() {
        let this_trees = count_trees(&map, *down, *right);
        println!("Trees for down {} right {}: {}", down, right, this_trees);
        trees *= this_trees;
    }
    println!("Product of the above: {}", trees);
}
