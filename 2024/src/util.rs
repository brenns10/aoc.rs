use std::{error::Error, num::ParseIntError};
use std::str::FromStr;

pub type MyResult<T> = Result<T, Box<dyn Error>>;
pub type RunResult = MyResult<(Option<isize>, Option<isize>)>;

#[allow(dead_code)]
pub fn return_part1(v: isize) -> RunResult { Ok((Some(v), None)) }
#[allow(dead_code)]
pub fn return_part1and2(v: isize, w: isize) -> RunResult { Ok((Some(v), Some(w))) }

#[allow(dead_code)]
pub fn read_ints<T: FromStr>(s: &str) -> Result<Vec<T>, ParseIntError>
where ParseIntError: From<<T as FromStr>::Err> {
    let mut vec = Vec::new();

    for num in s.split_ascii_whitespace() {
        vec.push(T::from_str(num)?);
    }

    Ok(vec)
}

#[allow(dead_code)]
pub fn read_arr<T: FromStr + Default + Copy, const N: usize>(s: &str) -> MyResult<[T; N]>
where <T as FromStr>::Err: std::error::Error,
      <T as FromStr>::Err: 'static {
    let mut arr = [T::default(); N];
    let mut i = 0;
    for elem in s.split_ascii_whitespace() {
        arr[i] = T::from_str(elem)?;
        i += 1;
    }
    if i != N {
        Err("Wrong number of elements".into())
    } else {
        Ok(arr)
    }
}
