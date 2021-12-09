use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;
use std::error::Error;

type BoxResult<T> = Result<T,Box<dyn Error>>;

fn read_ints() -> BoxResult<Vec<i32>> {
    let mut ints: Vec<i32> = Vec::new();
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line_str = line?;
        let int = i32::from_str_radix(&line_str, 10)?;
        ints.push(int);
    }
    Ok(ints)
}

fn num_increases(ints: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previous: Option<i32> = None;
    for int in ints {
        if let Some(prev_int) = previous {
            if *int > prev_int {
                count += 1;
            }
        }
        previous = Some(*int);
    }
    count
}

fn num_3window_increases(ints: &Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 3..ints.len() {
        if ints[i] > ints[i-3] {
            count += 1
        }
    }
    count
}

fn main() {
    let ints = read_ints().unwrap();
    println!("Increases: {}", num_increases(&ints));
    println!("3-window Increases: {}", num_3window_increases(&ints));
}
