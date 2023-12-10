use std::{error::Error, num::ParseIntError};
use std::str::FromStr;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn read_ints<T: FromStr>(s: &str) -> Result<Vec<T>, ParseIntError>
where ParseIntError: From<<T as FromStr>::Err> {
    let mut vec = Vec::new();

    for num in s.split_ascii_whitespace() {
        vec.push(T::from_str(num)?);
    }

    Ok(vec)
}
