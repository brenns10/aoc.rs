use std::{error::Error, num::ParseIntError};
use std::str::FromStr;

pub type MyResult<T> = Result<T, Box<dyn Error>>;
pub type RunResult = MyResult<(Option<isize>, Option<isize>)>;

#[allow(dead_code)]
pub fn return_part1(v: isize) -> RunResult { Ok((Some(v), None)) }
pub fn return_part1and2(v: isize, w: isize) -> RunResult { Ok((Some(v), Some(w))) }

pub fn read_ints<T: FromStr>(s: &str) -> Result<Vec<T>, ParseIntError>
where ParseIntError: From<<T as FromStr>::Err> {
    let mut vec = Vec::new();

    for num in s.split_ascii_whitespace() {
        vec.push(T::from_str(num)?);
    }

    Ok(vec)
}
