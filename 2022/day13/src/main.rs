use std::error::Error;
use std::result::Result;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

type MyResult<T> = Result<T,Box<dyn Error>>;

#[derive(Eq)]
enum Data {
    Integer(i32),
    List(Vec<Data>),
}

impl Data {
    fn from_str_internal(s: &str, level: usize) -> MyResult<(Data, &str)> {
        if s.len() == 0 {
            return Err("Empty string is disallowed".into());
        } else if let Some('[') = s.chars().next() {
            let mut l: Vec<Data> = Vec::new();
            let mut rem = &s[1..];
            while rem.chars().next().ok_or_else(|| format!("Unterminated list: {}", s))? != ']' {
                let (data, next) = Data::from_str_internal(rem, level + 1)?;
                l.push(data);
                if let Some(',') = next.chars().next() {
                    rem = &next[1..];
                } else {
                    rem = next;
                }

            }
            if let Some(']') = rem.chars().next() {
                return Ok((Data::List(l), &rem[1..]));
            }
            return Err(format!("Unterminated list: {}", s).into());
        } else {
            let mut last = s.len();
            for (i, c) in s.char_indices() {
                if !c.is_numeric() {
                    last = i;
                    break;
                }
            }
            let val = i32::from_str_radix(&s[..last], 10)?;
            return Ok((Data::Integer(val), &s[last..]));
        }
    }
    fn from_str(s: &str) -> MyResult<Data> {
        //println!("{}", s);
        let (data, s) = Data::from_str_internal(s, 0)?;
        if s.len() != 0 {
            Err("Not all data was exhausted in line!".into())
        } else {
            Ok(data)
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        use Data::*;
        match (self, other) {
            (Integer(l), Integer(r)) => l.cmp(r),
            (List(l), List(r)) => {
                let mut liter = l.iter();
                let mut riter = r.iter();
                loop {
                    break match (liter.next(), riter.next()) {
                        (None, None) => Ordering::Equal,
                        (None, Some(_)) => Ordering::Less,
                        (Some(_), None) => Ordering::Greater,
                        (Some(ll), Some(rr)) => {
                            let sub = ll.cmp(rr);
                            if sub == Ordering::Equal {
                                continue;
                            }
                            sub
                        }
                    }
                }
            }
            (Integer(l), r) => {
                let ll = List(vec![Integer(*l)]);
                ll.cmp(r)
            }
            (l, Integer(r)) => {
                let rl = List(vec![Integer(*r)]);
                l.cmp(&rl)
            }
        }
    }
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    let mut sum = 0;
    f.read_to_string(&mut s).unwrap();

    for (i, grp) in s.split("\n\n").enumerate() {
        let lines: Vec<&str> = grp.split("\n").collect();
        let data_left = Data::from_str(lines[0]).unwrap();
        let data_right = Data::from_str(lines[1]).unwrap();
        if data_left < data_right {
            sum += i + 1;
        }
    }

    println!("Sum of correctly ordered indices: {}", sum);
}
